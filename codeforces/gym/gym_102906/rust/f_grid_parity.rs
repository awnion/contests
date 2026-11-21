// dp[r][c] = length of the shortest sequence that is a subsequence of neither
// A[r+1..] nor B[c+1..]; row m+1 / column n+1 mean "already failed".
//
// dp is monotone and 1-Lipschitz, so every transition value seen from a cell
// is either min or min+1. Hence:
//   * the running minimum needs only a counter of "values equal to min";
//   * storing dp mod 2 is enough both for the sweep and for the backtrack.
use std::io::{self, Read, Write};

fn main() {
    let mut input = Vec::with_capacity(1 << 16);
    io::stdin().lock().read_to_end(&mut input).unwrap();
    let mut it = input
        .split(|c| *c <= b' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.iter().fold(0usize, |v, c| v * 10 + (c - b'0') as usize));

    let k = it.next().unwrap();
    let m = it.next().unwrap();
    let mut a = vec![0usize; m + 2];
    for i in 1..=m {
        a[i] = it.next().unwrap();
    }
    let n = it.next().unwrap();
    let mut b = vec![0usize; n + 2];
    for i in 1..=n {
        b[i] = it.next().unwrap();
    }

    let fr = m + 1;
    let fc = n + 1;
    let w = (n + 2 + 63) >> 6;

    // Next position holding the same value (or the fail index).
    let mut nxa = vec![fr; m + 2];
    let mut nxb = vec![fc; n + 2];
    let mut last = vec![fr; k + 1];
    for i in (1..=m).rev() {
        nxa[i] = last[a[i]];
        last[a[i]] = i;
    }
    last.iter_mut().for_each(|v| *v = fc);
    for i in (1..=n).rev() {
        nxb[i] = last[b[i]];
        last[b[i]] = i;
    }

    let mut par = vec![0u64; (m + 2) * w];
    let bit = |par: &[u64], base: usize, col: usize| -> usize {
        unsafe { (*par.get_unchecked(base + (col >> 6)) >> (col & 63)) as usize & 1 }
    };

    // Word offset of row nextA(row, x) for every x.
    let mut next_row = vec![fr * w; k + 1];
    // State of the sweep along column fc.
    let mut col_min = 0usize;
    let mut col_cnt = k;
    let mut total = 0usize;

    for row in (0..=fr).rev() {
        let base = row * w;
        let mut min = col_min;
        let mut cnt = col_cnt;

        if row != fr {
            par[base + (fc >> 6)] |= (((min + 1) & 1) as u64) << (fc & 63);
        }
        // The current word is kept in a register and stored without a reload.
        let mut cur = par[base + (n >> 6)];
        for col in (1..=n).rev() {
            if col & 63 == 63 {
                cur = 0;
            }
            cur |= (((min + 1) & 1) as u64) << (col & 63);
            unsafe {
                *par.get_unchecked_mut(base + (col >> 6)) = cur;
                let rx = *next_row.get_unchecked(*b.get_unchecked(col));
                cnt -= bit(&par, rx, col) ^ bit(&par, rx, *nxb.get_unchecked(col));
            }
            if cnt == 0 {
                min += 1;
                cnt = k;
            }
        }
        par[base] |= ((min + 1) & 1) as u64;
        total = min + 1;

        if row >= 1 && row <= m {
            col_cnt -= bit(&par, base, fc) ^ bit(&par, nxa[row] * w, fc);
            if col_cnt == 0 {
                col_min += 1;
                col_cnt = k;
            }
            next_row[a[row]] = base;
        }
    }

    // Backtrack: every transition value is d-1 or d, so parity identifies d-1.
    let occurrences = |s: &[usize], len: usize| {
        let mut start = vec![0usize; k + 2];
        for i in 1..=len {
            start[s[i] + 1] += 1;
        }
        for x in 1..=k + 1 {
            start[x] += start[x - 1];
        }
        let mut fill = start.clone();
        let mut occ = vec![0usize; len];
        for i in 1..=len {
            occ[fill[s[i]]] = i;
            fill[s[i]] += 1;
        }
        (start, occ)
    };
    let (end_a, occ_a) = occurrences(&a, m);
    let (end_b, occ_b) = occurrences(&b, n);
    let mut ptr_a = end_a.clone();
    let mut ptr_b = end_b.clone();

    let mut out = Vec::with_capacity(total * 5 + 16);
    writeln!(out, "{}", total).unwrap();
    let (mut r, mut c, mut d) = (0usize, 0usize, total);
    while d > 0 {
        for x in 1..=k {
            while ptr_a[x] < end_a[x + 1] && occ_a[ptr_a[x]] <= r {
                ptr_a[x] += 1;
            }
            while ptr_b[x] < end_b[x + 1] && occ_b[ptr_b[x]] <= c {
                ptr_b[x] += 1;
            }
            let nr = if ptr_a[x] < end_a[x + 1] { occ_a[ptr_a[x]] } else { fr };
            let nc = if ptr_b[x] < end_b[x + 1] { occ_b[ptr_b[x]] } else { fc };
            if bit(&par, nr * w, nc) == (d - 1) & 1 {
                write!(out, "{} ", x).unwrap();
                r = nr;
                c = nc;
                d -= 1;
                break;
            }
        }
    }
    out.push(b'\n');
    io::stdout().write_all(&out).unwrap();
}
