// DO NOT SUBMIT: tuned for a local run on this machine, not for a contest judge.
//
// Layered DP (see f_layers_static.rs for the idea):
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
// Build: rustc -C opt-level=3 -C target-cpu=native -C lto=fat -C codegen-units=1 -C panic=abort
// (make judge-rs-f_layers_do_not_submit)
use std::io::{self, Read, Write};

const MAXN: usize = 5000;
const CELLS: usize = (MAXN + 2 + 63) / 64 * 64; // 5056
const WORDS: usize = CELLS / 64; // 79
const DENSE_LIMIT: usize = 4_000_000;

#[repr(align(64))]
struct Cells<T>([T; CELLS]);

static mut CORNERS: [u64; (MAXN + 2) * WORDS] = [0; (MAXN + 2) * WORDS];

#[cfg(target_arch = "aarch64")]
mod simd {
    use std::arch::aarch64::*;
    pub type V = uint16x8_t;

    #[inline(always)]
    pub unsafe fn zero() -> V {
        unsafe { vdupq_n_u16(0) }
    }
    #[inline(always)]
    pub unsafe fn pack(lo: u64, hi: u64) -> V {
        unsafe { vreinterpretq_u16_u64(vcombine_u64(vcreate_u64(lo), vcreate_u64(hi))) }
    }
    #[inline(always)]
    pub unsafe fn load(p: *const u16) -> V {
        unsafe { vld1q_u16(p) }
    }
    #[inline(always)]
    pub unsafe fn or(a: V, b: V) -> V {
        unsafe { vorrq_u16(a, b) }
    }
    /// v = 8 cells of the new level, carry = max of everything to their right (all lanes).
    /// Stores the suffix maximum, returns (corner bits, new carry).
    #[inline(always)]
    pub unsafe fn step(v: V, carry: V, g: *mut u16) -> (u64, V) {
        unsafe {
            let zero = vdupq_n_u16(0);
            // in-register suffix maximum: lane j = max v[j..8]
            let mut s = vmaxq_u16(v, vextq_u16::<1>(v, zero));
            s = vmaxq_u16(s, vextq_u16::<2>(s, zero));
            s = vmaxq_u16(s, vextq_u16::<4>(s, zero));
            // maximum strictly to the right of lane j
            let right = vmaxq_u16(vextq_u16::<1>(s, zero), carry);
            let gv = vmaxq_u16(s, carry);
            vst1q_u16(g, gv);
            // one bit per lane: narrow the compare mask to bytes, keep bit j in lane j, add up
            let bits = vcreate_u8(0x8040_2010_0804_0201);
            let m = vand_u8(vmovn_u16(vcgtq_u16(v, right)), bits);
            (vaddv_u8(m) as u64, vdupq_laneq_u16::<0>(gv))
        }
    }
}

#[cfg(not(target_arch = "aarch64"))]
mod simd {
    pub type V = [u16; 8];

    #[inline(always)]
    pub unsafe fn zero() -> V {
        [0; 8]
    }
    #[inline(always)]
    pub unsafe fn pack(lo: u64, hi: u64) -> V {
        std::array::from_fn(|j| ((if j < 4 { lo } else { hi }) >> (16 * (j & 3))) as u16)
    }
    #[inline(always)]
    pub unsafe fn load(p: *const u16) -> V {
        unsafe { std::ptr::read_unaligned(p as *const V) }
    }
    #[inline(always)]
    pub unsafe fn or(a: V, b: V) -> V {
        std::array::from_fn(|j| a[j] | b[j])
    }
    #[inline(always)]
    pub unsafe fn step(v: V, carry: V, g: *mut u16) -> (u64, V) {
        let (mut best, mut bits) = (carry[0], 0u64);
        for j in (0..8).rev() {
            bits |= ((v[j] > best) as u64) << j;
            best = best.max(v[j]);
            unsafe { *g.add(j) = best };
        }
        (bits, [best; 8])
    }
}

/// One level, in place: `cells(at)` yields the new values of cells at..at+8 (it may read
/// g below at + 8), g becomes their suffix maximum and bit i of `row` is set iff cell i
/// is greater than everything to its right. `fail` is the value of the fail cell `fa`.
#[inline(always)]
unsafe fn level(g: *mut u16, row: *mut u64, words: usize, fa: usize, fail: u16, mut cells: impl FnMut(usize) -> simd::V) {
    unsafe {
        let mut fail_cells = [0u16; 8];
        fail_cells[fa & 7] = fail;
        let mut carry = simd::zero();
        for w in (0..words).rev() {
            let mut word = 0u64;
            for c in (0..8).rev() {
                let at = w * 64 + c * 8;
                let mut v = cells(at);
                if at == fa & !7 {
                    // `cells` yields 0 for the fail cell itself
                    v = simd::or(v, simd::load(fail_cells.as_ptr()));
                }
                let (bits, next) = simd::step(v, carry, g.add(at));
                word = (word << 8) | bits;
                carry = next;
            }
            *row.add(w) = word;
        }
    }
}

fn main() {
    let mut input = Vec::with_capacity(1 << 16);
    io::stdin().lock().read_to_end(&mut input).unwrap();
    let mut it = input
        .split(|c| *c <= b' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.iter().fold(0usize, |v, c| v * 10 + (c - b'0') as usize));

    let k = it.next().unwrap();
    let mut m = it.next().unwrap();
    let mut a: Vec<usize> = (0..m).map(|_| it.next().unwrap()).collect();
    let mut n = it.next().unwrap();
    let mut b: Vec<usize> = (0..n).map(|_| it.next().unwrap()).collect();
    if m > n {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut m, &mut n);
    }
    // 1-based from here on.
    a.insert(0, 0);
    b.insert(0, 0);
    let fa = m + 1;

    // prev[p] = previous occurrence of a[p] in A (0 if none), last[x] = last occurrence.
    let mut last = vec![0usize; k + 1];
    let mut prev = Cells([0u16; CELLS]);
    let prev = &mut prev.0;
    for p in 1..=m {
        prev[p] = last[a[p]] as u16;
        last[a[p]] = p;
    }

    // Positions of every value in B.
    let mut start = vec![0usize; k + 2];
    let mut occ = vec![0u16; n];
    for j in 1..=n {
        start[b[j] + 1] += 1;
    }
    for x in 1..=k + 1 {
        start[x] += start[x - 1];
    }
    let mut fill = start.clone();
    for j in 1..=n {
        occ[fill[b[j]]] = j as u16;
        fill[b[j]] += 1;
    }

    // nextB lookup on "position + 1" values, 0 = unreachable:
    //   nb(j + 1, x) = nextB(j, x) + 1,  nb(0, x) = 0.
    // Small k: dense table, column x - 1 holds nb(.., x) and column k is all zeros (it
    // serves cell 0 and the padding cells). Large k (few levels): binary search.
    let stride = n + 3;
    let dense = (k + 1) * stride <= DENSE_LIMIT;
    let mut table: Vec<u16> = Vec::new();
    let mut base = Cells([0u32; CELLS]);
    let base = &mut base.0;
    if dense {
        table = vec![(n + 2) as u16; (k + 1) * stride];
        for x in 1..=k {
            let column = &mut table[(x - 1) * stride..x * stride];
            column[0] = 0;
            let mut from = 1;
            for &p in &occ[start[x]..start[x + 1]] {
                column[from..=p as usize].fill(p + 1);
                from = p as usize + 1;
            }
        }
        table[k * stride..].fill(0);
        base.fill((k * stride) as u32);
        for p in 1..=m {
            base[p] = ((a[p] - 1) * stride) as u32;
        }
    }
    let sparse_nb = |j1: usize, x: usize| -> usize {
        if j1 == 0 {
            return 0;
        }
        // first occurrence of x strictly after position j1 - 1
        let s = &occ[start[x]..start[x + 1]];
        let i = s.partition_point(|&p| (p as usize) < j1);
        if i < s.len() { s[i] as usize + 1 } else { n + 2 }
    };

    // g = suffix maximum of the current level; level 0 is the single state (0, 0).
    // Back pointers, 1 bit per (level, position): bit i of a level is set iff
    // dp[i] > max dp[i+1..], i.e. i is a corner of the Pareto staircase.
    // The source of a state reached from suffix start q is the first corner >= q.
    let mut g = Cells([0u16; CELLS]);
    let g = &mut g.0;
    let mut scratch = Cells([0u16; CELLS]);
    let scratch = &mut scratch.0;
    let wl = (m + 2 + 63) >> 6;
    let corners: &mut [u64; (MAXN + 2) * WORDS] = unsafe { &mut *std::ptr::addr_of_mut!(CORNERS) };
    g[0] = 1;
    corners[0] = 1;
    let mut fail_ch: Vec<u16> = Vec::with_capacity(MAXN + 2);

    loop {
        // Fail cell of the next level: the best value reachable by leaving A.
        let (mut fbest, mut fch) = (0usize, 0usize);
        for x in 1..=k {
            let j1 = g[last[x]] as usize;
            let v = if dense { table[(x - 1) * stride + j1] as usize } else { sparse_nb(j1, x) };
            if v > fbest {
                fbest = v;
                fch = x;
            }
        }
        fail_ch.push(fch as u16);
        if fbest == n + 2 {
            break;
        }

        let row = unsafe { corners.as_mut_ptr().add(fail_ch.len() * wl) };
        let gp = g.as_mut_ptr();
        if dense {
            let (t, pr, bs) = (table.as_ptr(), prev.as_ptr(), base.as_ptr());
            unsafe {
                let cell = |p: usize| *t.add(*bs.add(p) as usize + *gp.add(*pr.add(p) as usize) as usize) as u64;
                level(gp, row, wl, fa, fbest as u16, |at| {
                    let lo = cell(at) | cell(at + 1) << 16 | cell(at + 2) << 32 | cell(at + 3) << 48;
                    let hi = cell(at + 4) | cell(at + 5) << 16 | cell(at + 6) << 32 | cell(at + 7) << 48;
                    simd::pack(lo, hi)
                });
            }
        } else {
            for p in 1..=m {
                scratch[p] = sparse_nb(g[prev[p] as usize] as usize, a[p]) as u16;
            }
            let sp = scratch.as_ptr();
            unsafe { level(gp, row, wl, fa, fbest as u16, |at| simd::load(sp.add(at))) };
        }
    }

    let first_corner = |level: usize, q: usize| -> usize {
        let row = &corners[level * wl..(level + 1) * wl];
        let mut w = q >> 6;
        let mut word = row[w] & (!0u64 << (q & 63));
        while word == 0 {
            w += 1;
            word = row[w];
        }
        (w << 6) + word.trailing_zeros() as usize
    };

    let len = fail_ch.len();
    let mut ans = vec![0usize; len];
    let mut i = fa;
    for l in (0..len).rev() {
        let (x, q) = if i == fa {
            let x = fail_ch[l] as usize;
            (x, last[x])
        } else {
            (a[i], prev[i] as usize)
        };
        ans[l] = x;
        i = first_corner(l, q);
    }

    let mut out = Vec::with_capacity(len * 5 + 16);
    writeln!(out, "{}", len).unwrap();
    for x in ans {
        write!(out, "{} ", x).unwrap();
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}
