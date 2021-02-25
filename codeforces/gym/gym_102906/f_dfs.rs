
#![allow(unused_imports, unused_macros)]

use std::{
    any::type_name,
    cmp,
    collections::*,
    collections::HashMap,
    fmt::Debug,
    io::{self, prelude::*},
    iter,
    mem::{replace, swap},
    str::FromStr
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

fn prep_refs(k: usize, n: usize, seq: &Vec<usize>) -> Vec<usize> {
    let mut r = vec![n + 1; 5005 * k];
    for i in 0..k {
        let mut cur = n + 1;
        for j in (0..n).rev() {
            if seq[j] == i + 1 {
                cur = j + 1;
            }
            r[i * 5005 + j] = cur;
        }
    }
    r
}

struct FEnv {
    na: Vec<usize>,
    nb: Vec<usize>,
    n: usize,
    m: usize,
    k: usize,
}

fn dp(e: &FEnv, i: usize, j: usize, cache: &mut[usize]) -> usize {
    if i > e.n && j > e.m { return 0 }

    let r = cache[i * 5005 + j];

    if r < usize::MAX { return r }

    let mut r = usize::MAX;
    for el in 0..e.k {
        r = r.min(dp(e, e.na[el * 5005 + i], e.nb[el * 5005 + j], cache) + 1)
    }
    cache[i * 5005 + j] = r;
    r
}


fn backtrack(e: &FEnv, i: usize, j: usize, cache: &mut[usize], r: usize) {
    if r == 0 { return }
    for el in 0..e.k {
        if r - 1 == dp(e, e.na[el * 5005 + i], e.nb[el * 5005 + j], cache) {
            print!("{} ", el + 1);
            backtrack(e, e.na[el * 5005 + i], e.nb[el * 5005 + j], cache, r - 1);
            break;
        }
    }
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
        for _ in 0..=n.max(m) { print!("1 ") }
        return;
    }

    let na = prep_refs(k, n, &a);
    let nb = prep_refs(k, m, &b);

    let mut cache = vec![usize::MAX; 5005 * 5005];
    let env = FEnv { na, nb, n, m, k };

    let r = dp(&env, 0, 0, &mut cache);
    println!("{}", r);
    backtrack(&env, 0, 0, &mut cache, r);
    println!();
}
