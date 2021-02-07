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

fn pack(x: usize, y: usize) -> usize {
    x * 5005 + y
}


fn unpack(h: usize) -> (usize, usize) {
    (h / 5005, h % 5005)
}

fn main() {
    let k: i64 = read_i();
    let mut n: usize = read_i();
    let a: Vec<i64> = read_vi(n);
    let mut m: usize = read_i();
    let b: Vec<i64> = read_vi(m);

    n = a.len();
    m = b.len();

    let mut visited = HashMap::new();
    let mut q = VecDeque::with_capacity(n + m);
    q.push_back(pack(0, 0));

    loop {
        let from_hash = match q.pop_front() {
            Some(from_hash) => from_hash,
            _ => break
        };
        let (from_x, from_y) = unpack(from_hash);

        for el in 1..=k {
            let mut x = from_x;
            let mut y = from_y;

            while x < n && a[x] != el {
                x += 1;
            }

            while y < m && b[y] != el {
                y += 1;
            }

            if x >= n && y >= m {
                let mut ans = VecDeque::new();
                ans.push_back(el);
                let (mut x, mut y) = (from_x, from_y);
                while x != 0 && y != 0 {
                    ans.push_front( if x <= n { a[x-1] } else { b[y-1] });
                    let new_pack = match visited.get(&pack(x, y)) {
                        Some(&n) => n,
                        _ => panic!("cant find a key in hash")
                    };
                    let new_pack = unpack(new_pack);
                    x = new_pack.0;
                    y = new_pack.1;
                }
                println!("{}", ans.len());
                let ans_string = ans
                    .into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{}", ans_string);
                return;
            }

            let to_hash = pack(n.min(x) + 1, m.min(y) + 1);
            if !visited.contains_key(&to_hash) {
                visited.insert(to_hash, from_hash);
                q.push_back(to_hash);
            }
        }

    }
}
