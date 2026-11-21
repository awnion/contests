// Layered DP indexed by (answer length, position in A):
//   dp[l][i] = furthest position in B reachable by some sequence of length l
//              whose greedy match in A ends at i (m+1 / n+1 = "failed").
//
// A state (i, j) is dominated by (i' >= i, j' >= j), so a level may be replaced
// by its suffix maximum g[i] = max dp[l][i..]. Then for a position p of A with
// value x and previous occurrence q of x:
//   dp[l+1][p] = nextB(g[q], x)
// which makes a level cost O(m + k) instead of O(m * k).
// Total: O(ans * (m + k)), and ans * k <= n + m + k.
//
// Port of rust/f_layers_static.rs.
#include <algorithm>
#include <cstdint>
#include <cstdio>

typedef uint16_t u16;
typedef uint64_t u64;

static const int MAXN = 5000;
static const int DENSE_LIMIT = 4000000;

static char input[1 << 20];
static int seq_a[MAXN + 2], seq_b[MAXN + 2];
static int last_a[MAXN + 1];
static u16 prev_a[MAXN + 2];
// nextB lookup on "position + 1" values, 0 = unreachable:
//   nb(j + 1, x) = nextB(j, x) + 1,  nb(0, x) = 0.
// Small k: dense table. Large k (few levels): binary search in occurrence lists.
static u16 table[DENSE_LIMIT];
static int start[MAXN + 2], fill_pos[MAXN + 2];
static u16 occ[MAXN];
static u16 dp[MAXN + 2], g[MAXN + 2];
// Back pointers, 1 bit per (level, position): bit i of a level is set iff
// dp[i] > max dp[i+1..], i.e. i is a corner of the Pareto staircase.
// The source of a state reached from suffix start q is the first corner >= q.
static u64 corners[5002 * 79];
static u16 fail_ch[MAXN + 2];
static int ans[MAXN + 2];
static char output[(MAXN + 2) * 5 + 16];

static int k, m, n;
static bool dense;

static inline int nb(int j1, int x) {
  if (dense) return table[j1 * k + x - 1];
  if (j1 == 0) return 0;
  // first occurrence of x strictly after position j1 - 1
  const u16 *lo = occ + start[x], *hi = occ + start[x + 1];
  const u16 *it = std::lower_bound(lo, hi, (u16)j1);
  return it < hi ? *it + 1 : n + 2;
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

  dense = (long long)k * (n + 3) <= DENSE_LIMIT;
  if (dense) {
    std::fill(table, table + k, (u16)0);
    std::fill(table + k, table + (n + 3) * k, (u16)(n + 2));
    for (int j = n - 1; j >= 0; j--) {
      std::copy(table + (j + 2) * k, table + (j + 3) * k, table + (j + 1) * k);
      table[(j + 1) * k + b[j + 1] - 1] = (u16)(j + 2);
    }
  } else {
    for (int j = 1; j <= n; j++) start[b[j] + 1]++;
    for (int x = 1; x <= k + 1; x++) start[x] += start[x - 1];
    std::copy(start, start + k + 2, fill_pos);
    for (int j = 1; j <= n; j++) occ[fill_pos[b[j]]++] = (u16)j;
  }

  const int wl = (m + 2 + 63) >> 6;
  int levels = 0;
  dp[0] = 1;

  while (dp[fa] != n + 2) {
    u64 *row = corners + (size_t)levels * wl;
    u16 best = 0;
    u64 word = 0;
    for (int i = fa; i >= 0; i--) {
      u16 v = dp[i];
      word |= (u64)(v > best) << (i & 63);
      best = std::max(best, v);
      g[i] = best;
      if ((i & 63) == 0) {
        row[i >> 6] = word;
        word = 0;
      }
    }

    dp[0] = 0;
    for (int i = 1; i <= m; i++) dp[i] = (u16)nb(g[prev_a[i]], a[i]);

    int fbest = 0, fch = 0;
    for (int x = 1; x <= k; x++) {
      int v = nb(g[last_a[x]], x);
      if (v > fbest) {
        fbest = v;
        fch = x;
      }
    }
    dp[fa] = (u16)fbest;
    fail_ch[levels++] = (u16)fch;
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
  o += sprintf(o, "%d\n", levels);
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
