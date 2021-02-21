
#![allow(unused_imports, unused_macros)]

use std::{
    cmp,
    collections::*,
    fmt::Debug,
    io::{self, prelude::*},
    iter,
    mem::{replace, swap},
    str::FromStr,
};

pub fn readline() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("fail readline");
    s
}

fn read_i<T: FromStr>() -> T {
    readline().trim().parse::<T>().ok().expect("fail parse int")
}

fn read_vi<T: FromStr>(_len: usize) -> Vec<T> {
    readline().trim().split_ascii_whitespace().map(|s| {
        s.parse::<T>().ok().expect("fail parse vector")
    }).collect()
}

fn
prep_refs(k: usize, n: usize, seq: &Vec<usize>)
-> Vec<Vec<usize>> {
    let mut r = vec![vec![n + 1; n + 2]; k];
    for i in 0..k {
        let mut cur = n + 1;
        for j in (0..n).rev() {
            if seq[j] == i + 1 {
                cur = j + 1;
            }
            r[i][j] = cur;
        }
    }
    r
}


fn main() {
    let k: usize = read_i();
    let n: usize = read_i();
    let a: Vec<usize> = read_vi(n);
    let m: usize = read_i();
    let b: Vec<usize> = read_vi(m);

    let n = a.len();
    let m = b.len();

    if k == 1 {
        println!("{}", n.max(m) + 1);
        for _ in 0..n.max(m) + 1 {
            print!("1 ");
        }
        return;
    }

    let na = prep_refs(k, n, &a);
    let nb = prep_refs(k, m, &b);

    let mut cache = vec![vec![usize::MAX; m + 2]; n + 2];

    struct FEnv {
        na: Vec<Vec<usize>>,
        nb: Vec<Vec<usize>>,
        n: usize,
        m: usize,
        k: usize,
    };

    let env = FEnv {
        na: na,
        nb: nb,
        n: n,
        m: m,
        k: k,
    };

    fn dp(e: &FEnv, i: usize, j: usize, cache: &mut Vec<Vec<usize>>) -> usize {
        if i > e.n && j > e.m { return 0 }
        if cache[i][j] < usize::MAX { return cache[i][j] }
        let mut r = usize::MAX;
        for el in 0..e.k {
            r = r.min(dp(e, e.na[el][i], e.nb[el][j], cache) + 1);
        }
        cache[i][j] = r;
        r
    };

    fn backtrack(e: &FEnv, i: usize, j: usize, cache: &mut Vec<Vec<usize>>, r: usize) {
        if r == 0 { return }
        for el in 0..e.k {
            if r - 1 == dp(e, e.na[el][i], e.nb[el][j], cache) {
                print!("{} ", el + 1);
                backtrack(e, e.na[el][i], e.nb[el][j], cache, r - 1);
                break;
            }
        }
    }

    let r = dp(&env, 0, 0, &mut cache);
    println!("{}", r);
    backtrack(&env, 0, 0, &mut cache, r);
    println!();
}
