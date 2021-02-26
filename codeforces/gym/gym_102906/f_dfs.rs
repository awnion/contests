
#![allow(unused_imports, unused_macros)]

use std::{any::type_name, cmp, collections::*, collections::HashMap, fmt::Debug, io::{self, prelude::*}, iter, mem::{replace, swap}, result, str::FromStr, usize};

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

type U = u16;
const SIZE: U = 5005;
const USIZE: usize = SIZE as usize;

struct Solution {
    k: U,
    n: U,
    m: U,
    na: Vec<U>,
    nb: Vec<U>,
    cache: Vec<U>,
}

macro_rules! g {
    ( $i:tt, $j:tt ) => {
        ($i as usize * USIZE) + ($j as usize)
    };
}

impl Solution {
    fn dp(&mut self, i: U, j: U) -> U {
        if i > self.n && j > self.m { return 0 }

        let r = self.cache[g![i, j]];
        if r < U::MAX { return r }

        let mut r = U::MAX;
        for el in 0..self.k as usize {
            r = r.min(self.dp(self.na[g!{el, i}], self.nb[g![el, j]]) + 1);
        }
        self.cache[g![i, j]] = r;
        r
    }

    fn backtrack(&mut self, i: U, j: U, r: U) {
        if r == 0 { return }
        for el in 0..self.k {
            if r - 1 == self.dp(self.na[g![el, i]], self.nb[g![el, j]]) {
                print!("{} ", el + 1);
                self.backtrack(self.na[g![el, i]], self.nb[g![el, j]], r - 1);
                break;
            }
        }
    }

    fn prep_refs(k: U, n: U, seq: Vec<U>) -> Vec<U> {
        let mut r = vec![n + 1; USIZE * USIZE];
        for el in 0..k {
            let mut cur = n + 1;
            for j in (0..n).rev() {
                if seq[j as usize] == el + 1 {
                    cur = j as U + 1;
                }
                r[g![el, j]] = cur;
            }
        }
        r
    }
}

// #[inline]
// fn f(i: U, j: U) -> usize {
//     USIZE * i as usize + j as usize
// }


fn main() {
    let k: U = read_i();
    let n: U = read_i();
    let a: Vec<U> = read_vi(n as usize);
    let m: U = read_i();
    let b: Vec<U> = read_vi(m as usize);

    let n = a.len() as U;
    let m = b.len() as U;

    if k == 1 {
        println!("{}", n.max(m) + 1);
        for _ in 0..=n.max(m) { print!("1 ") }
        return;
    }

    let na = Solution::prep_refs(k, n, a);
    let nb = Solution::prep_refs(k, m, b);
    let cache = vec![U::MAX; USIZE * USIZE];

    let mut solution = Solution { k, n, m, na, nb, cache };

    let r = solution.dp(0, 0);
    println!("{}", r);
    solution.backtrack(0, 0, r);
    println!();
}
