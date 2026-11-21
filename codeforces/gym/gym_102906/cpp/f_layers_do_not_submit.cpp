// DO NOT SUBMIT: tuned for a local run on this machine, not for a contest judge.
//
// Layered DP (see f_layers_static.cpp for the idea):
//
//   dp[l][i] = furthest position in B reachable by a sequence of length l whose greedy
//              match in A ends at i;  g[i] = max dp[l][i..];
//   dp[l+1][p] = nextB(g[prev[p]], a[p]).
//
// What is different from the contest version:
//   * one pass per level, in place: prev[p] < p, so walking p downwards the old g[prev[p]]
//     is still intact when the new g[p] is written; there is no dp array at all;
//   * 8 cells at a time: scalar gather packed into a register, NEON suffix maximum;
//   * nextB table stored as [value][position], so a lookup is base[p] + g[prev[p]].
//
// Twin of rust/f_layers_do_not_submit.rs.
#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <cstring>
#if defined(__aarch64__) && !defined(NO_NEON)
#include <arm_neon.h>
#endif

typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

static const int MAXN = 5000;
static const int CELLS = (MAXN + 2 + 63) / 64 * 64;  // 5056
static const int WORDS = CELLS / 64;                 // 79
static const int DENSE_LIMIT = 4000000;

static char input[1 << 20];
static int seq_a[MAXN + 2], seq_b[MAXN + 2];
static int last_a[MAXN + 1];
alignas(64) static u16 prev_a[CELLS];
alignas(64) static u32 base_a[CELLS];
// nextB lookup on "position + 1" values, 0 = unreachable:
//   nb(j + 1, x) = nextB(j, x) + 1,  nb(0, x) = 0.
// Small k: dense table, column x - 1 holds nb(.., x) and column k is all zeros (it
// serves cell 0 and the padding cells). Large k (few levels): binary search.
static u16 table[DENSE_LIMIT];
static int start[MAXN + 2], fill_pos[MAXN + 2];
static u16 occ[MAXN];
// g = suffix maximum of the current level.
alignas(64) static u16 g[CELLS];
alignas(64) static u16 scratch[CELLS];
// Back pointers, 1 bit per (level, position): bit i of a level is set iff
// dp[i] > max dp[i+1..], i.e. i is a corner of the Pareto staircase.
// The source of a state reached from suffix start q is the first corner >= q.
static u64 corners[(MAXN + 2) * WORDS];
static u16 fail_ch[MAXN + 2];
static int ans[MAXN + 2];
static char output[(MAXN + 2) * 5 + 16];

static int k, m, n;

static inline int sparse_nb(int j1, int x) {
  if (j1 == 0) return 0;
  // first occurrence of x strictly after position j1 - 1
  const u16 *lo = occ + start[x], *hi = occ + start[x + 1];
  const u16 *it = std::lower_bound(lo, hi, (u16)j1);
  return it < hi ? *it + 1 : n + 2;
}

#if defined(__aarch64__) && !defined(NO_NEON)
typedef uint16x8_t V;
static inline V v_zero() { return vdupq_n_u16(0); }
static inline V v_pack(u64 lo, u64 hi) {
  return vreinterpretq_u16_u64(vcombine_u64(vcreate_u64(lo), vcreate_u64(hi)));
}
static inline V v_load(const u16 *p) { return vld1q_u16(p); }
static inline V v_or(V a, V b) { return vorrq_u16(a, b); }
// v = 8 cells of the new level, carry = max of everything to their right (all lanes).
// Stores the suffix maximum, returns the corner bits and updates carry.
static inline u64 v_step(V v, V &carry, u16 *out) {
  const V zero = vdupq_n_u16(0);
  // in-register suffix maximum: lane j = max v[j..8]
  V s = vmaxq_u16(v, vextq_u16(v, zero, 1));
  s = vmaxq_u16(s, vextq_u16(s, zero, 2));
  s = vmaxq_u16(s, vextq_u16(s, zero, 4));
  // maximum strictly to the right of lane j
  V right = vmaxq_u16(vextq_u16(s, zero, 1), carry);
  V gv = vmaxq_u16(s, carry);
  vst1q_u16(out, gv);
  // one bit per lane: narrow the compare mask to bytes, keep bit j in lane j, add up
  const uint8x8_t bits = vcreate_u8(0x8040201008040201ULL);
  uint8x8_t mask = vand_u8(vmovn_u16(vcgtq_u16(v, right)), bits);
  carry = vdupq_laneq_u16(gv, 0);
  return vaddv_u8(mask);
}
#else
struct V {
  u16 c[8];
};
static inline V v_zero() { return V{}; }
static inline V v_pack(u64 lo, u64 hi) {
  V r;
  for (int j = 0; j < 8; j++) r.c[j] = (u16)((j < 4 ? lo : hi) >> (16 * (j & 3)));
  return r;
}
static inline V v_load(const u16 *p) {
  V r;
  memcpy(r.c, p, sizeof(r.c));
  return r;
}
static inline V v_or(V a, V b) {
  for (int j = 0; j < 8; j++) a.c[j] |= b.c[j];
  return a;
}
static inline u64 v_step(V v, V &carry, u16 *out) {
  u16 best = carry.c[0];
  u64 bits = 0;
  for (int j = 7; j >= 0; j--) {
    bits |= (u64)(v.c[j] > best) << j;
    best = std::max(best, v.c[j]);
    out[j] = best;
  }
  for (int j = 0; j < 8; j++) carry.c[j] = best;
  return bits;
}
#endif

// One level, in place: cells(at) yields the new values of cells at..at+8 (it may read
// g below at + 8), g becomes their suffix maximum and bit i of `row` is set iff cell i
// is greater than everything to its right. `fail` is the value of the fail cell `fa`.
template <class Cells>
static inline void level(u64 *row, int words, int fa, u16 fail, Cells cells) {
  u16 fail_cells[8] = {0};
  fail_cells[fa & 7] = fail;
  V carry = v_zero();
  for (int w = words - 1; w >= 0; w--) {
    u64 word = 0;
    for (int c = 7; c >= 0; c--) {
      const int at = w * 64 + c * 8;
      V v = cells(at);
      // `cells` yields 0 for the fail cell itself
      if (at == (fa & ~7)) v = v_or(v, v_load(fail_cells));
      word = (word << 8) | v_step(v, carry, g + at);
    }
    row[w] = word;
  }
}

int main() {
  size_t len = fread(input, 1, sizeof(input) - 1, stdin);
  input[len] = 0;
  const char *p = input;
  auto next_int = [&p]() {
    while (*p && *p <= ' ') ++p;
    int v = 0;
    while (*p > ' ') v = v * 10 + (*p++ - '0');
    return v;
  };

  k = next_int();
  m = next_int();
  for (int i = 1; i <= m; i++) seq_a[i] = next_int();
  n = next_int();
  for (int i = 1; i <= n; i++) seq_b[i] = next_int();
  int *a = seq_a, *b = seq_b;
  if (m > n) {
    std::swap(a, b);
    std::swap(m, n);
  }
  const int fa = m + 1;

  // prev_a[p] = previous occurrence of a[p] in A (0 if none), last_a[x] = last occurrence.
  for (int i = 1; i <= m; i++) {
    prev_a[i] = (u16)last_a[a[i]];
    last_a[a[i]] = i;
  }

  // Positions of every value in B.
  for (int j = 1; j <= n; j++) start[b[j] + 1]++;
  for (int x = 1; x <= k + 1; x++) start[x] += start[x - 1];
  std::copy(start, start + k + 2, fill_pos);
  for (int j = 1; j <= n; j++) occ[fill_pos[b[j]]++] = (u16)j;

  const int stride = n + 3;
  const bool dense = (long long)(k + 1) * stride <= DENSE_LIMIT;
  if (dense) {
    std::fill(table, table + (size_t)k * stride, (u16)(n + 2));
    for (int x = 1; x <= k; x++) {
      u16 *column = table + (size_t)(x - 1) * stride;
      column[0] = 0;
      int from = 1;
      for (int t = start[x]; t < start[x + 1]; t++) {
        const int pos = occ[t];
        std::fill(column + from, column + pos + 1, (u16)(pos + 1));
        from = pos + 1;
      }
    }
    // column k is already zero (static storage)
    std::fill(base_a, base_a + CELLS, (u32)(k * stride));
    for (int i = 1; i <= m; i++) base_a[i] = (u32)((a[i] - 1) * stride);
  }

  // Level 0 is the single state (0, 0).
  const int wl = (m + 2 + 63) >> 6;
  g[0] = 1;
  corners[0] = 1;
  int levels = 0;

  for (;;) {
    // Fail cell of the next level: the best value reachable by leaving A.
    int fbest = 0, fch = 0;
    for (int x = 1; x <= k; x++) {
      const int j1 = g[last_a[x]];
      const int v = dense ? table[(size_t)(x - 1) * stride + j1] : sparse_nb(j1, x);
      if (v > fbest) {
        fbest = v;
        fch = x;
      }
    }
    fail_ch[levels++] = (u16)fch;
    if (fbest == n + 2) break;

    u64 *row = corners + (size_t)levels * wl;
    if (dense) {
      auto cell = [](int i) -> u64 { return table[base_a[i] + g[prev_a[i]]]; };
      level(row, wl, fa, (u16)fbest, [&cell](int at) {
        const u64 lo = cell(at) | cell(at + 1) << 16 | cell(at + 2) << 32 | cell(at + 3) << 48;
        const u64 hi = cell(at + 4) | cell(at + 5) << 16 | cell(at + 6) << 32 | cell(at + 7) << 48;
        return v_pack(lo, hi);
      });
    } else {
      for (int i = 1; i <= m; i++) scratch[i] = (u16)sparse_nb(g[prev_a[i]], a[i]);
      level(row, wl, fa, (u16)fbest, [](int at) { return v_load(scratch + at); });
    }
  }

  int i = fa;
  for (int l = levels - 1; l >= 0; l--) {
    int x, q;
    if (i == fa) {
      x = fail_ch[l];
      q = last_a[x];
    } else {
      x = a[i];
      q = prev_a[i];
    }
    ans[l] = x;
    const u64 *row = corners + (size_t)l * wl;
    int w = q >> 6;
    u64 word = row[w] & (~0ULL << (q & 63));
    while (word == 0) word = row[++w];
    i = (w << 6) + __builtin_ctzll(word);
  }

  char *o = output;
  o += snprintf(o, 16, "%d\n", levels);
  for (int l = 0; l < levels; l++) {
    int v = ans[l];
    char tmp[8];
    int t = 0;
    do {
      tmp[t++] = (char)('0' + v % 10);
      v /= 10;
    } while (v);
    while (t) *o++ = tmp[--t];
    *o++ = ' ';
  }
  *o++ = '\n';
  fwrite(output, 1, o - output, stdout);
  return 0;
}
