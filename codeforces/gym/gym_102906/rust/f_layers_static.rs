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
use std::io::{self, Read, Write};

static mut CORNERS: [u64; 5002 * 79] = [0; 5002 * 79];

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
    let mut prev = vec![0u16; m + 1];
    for p in 1..=m {
        prev[p] = last[a[p]] as u16;
        last[a[p]] = p;
    }

    // nextB lookup on "position + 1" values, 0 = unreachable:
    //   nb(j + 1, x) = nextB(j, x) + 1,  nb(0, x) = 0.
    // Small k: dense table. Large k (few levels): binary search in occurrence lists.
    let dense = k * (n + 3) <= 4_000_000;
    let mut table: Vec<u16> = Vec::new();
    let mut start = vec![0usize; k + 2];
    let mut occ = vec![0u16; n];
    if dense {
        table = vec![(n + 2) as u16; (n + 3) * k];
        table[..k].fill(0);
        for j in (0..n).rev() {
            let (lo, hi) = table.split_at_mut((j + 2) * k);
            lo[(j + 1) * k..].copy_from_slice(&hi[..k]);
            lo[(j + 1) * k + b[j + 1] - 1] = (j + 2) as u16;
        }
    } else {
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
    }
    let nb = |j1: usize, x: usize| -> usize {
        if dense {
            unsafe { *table.get_unchecked(j1 * k + x - 1) as usize }
        } else if j1 == 0 {
            0
        } else {
            // first occurrence of x strictly after position j1 - 1
            let s = &occ[start[x]..start[x + 1]];
            let i = s.partition_point(|&p| (p as usize) < j1);
            if i < s.len() { s[i] as usize + 1 } else { n + 2 }
        }
    };

    let mut dp = vec![0u16; m + 2];
    let mut g = vec![0u16; m + 2];
    dp[0] = 1;
    // Back pointers, 1 bit per (level, position): bit i of a level is set iff
    // dp[i] > max dp[i+1..], i.e. i is a corner of the Pareto staircase.
    // The source of a state reached from suffix start q is the first corner >= q.
    let wl = (m + 2 + 63) >> 6;
    let corners: &mut [u64; 5002 * 79] = unsafe { &mut *std::ptr::addr_of_mut!(CORNERS) };
    let mut levels = 0usize;
    let mut fail_ch: Vec<u16> = Vec::new();

    while dp[fa] as usize != n + 2 {
        let base = levels * wl;
        levels += 1;
        let row = &mut corners[base..base + wl];
        let mut best = 0u16;
        let mut word = 0u64;
        for i in (0..=fa).rev() {
            unsafe {
                let v = *dp.get_unchecked(i);
                word |= ((v > best) as u64) << (i & 63);
                best = best.max(v);
                *g.get_unchecked_mut(i) = best;
                if i & 63 == 0 {
                    *row.get_unchecked_mut(i >> 6) = word;
                    word = 0;
                }
            }
        }

        dp[0] = 0;
        for p in 1..=m {
            unsafe {
                let q = *prev.get_unchecked(p) as usize;
                *dp.get_unchecked_mut(p) = nb(*g.get_unchecked(q) as usize, *a.get_unchecked(p)) as u16;
            }
        }

        let (mut fbest, mut fch) = (0usize, 0usize);
        for x in 1..=k {
            let v = nb(g[last[x]] as usize, x);
            if v > fbest {
                fbest = v;
                fch = x;
            }
        }
        dp[fa] = fbest as u16;
        fail_ch.push(fch as u16);
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
