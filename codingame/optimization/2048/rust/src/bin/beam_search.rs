// 1:1 port of ../../../python/numpy_beam_search.py (including its quirks).

use std::io::{self, BufRead};
use std::time::Instant;

type Grid = [[i32; 4]; 4];
type State = (Grid, String);

const DEPTH: usize = 11;
const MOVES: usize = 8;
const WIDTH: usize = 15;

fn squash(a: &mut [i32; 4]) {
    let n = a.len();
    let (mut w, mut r) = (0, 0);
    while w < n && r < n {
        if r <= w {
            r = w + 1;
        }
        while r < n && a[r] == 0 {
            r += 1;
        }
        if r >= n {
            break;
        }
        if a[w] == 0 {
            a[w] = a[r];
            a[r] = 0;
            r += 1;
        } else {
            if a[w] == a[r] {
                a[w] += a[r];
                a[r] = 0;
            }
            w += 1;
        }
    }
}

/// Cell of the grid that `np.rot90(grid, k)[i][j]` refers to.
fn rot90(k: usize, i: usize, j: usize) -> (usize, usize) {
    match k {
        0 => (i, j),
        1 => (j, 3 - i),
        2 => (3 - i, 3 - j),
        _ => (3 - j, i),
    }
}

fn make_move(grid: &Grid, direction: char, seed: u64, value: i32) -> Option<Grid> {
    let k = "LURD".find(direction).unwrap();
    let mut a = *grid;
    for i in 0..4 {
        let mut row = [0; 4];
        for j in 0..4 {
            let (y, x) = rot90(k, i, j);
            row[j] = a[y][x];
        }
        squash(&mut row);
        for j in 0..4 {
            let (y, x) = rot90(k, i, j);
            a[y][x] = row[j];
        }
    }

    if a == *grid || count_nonzero(&a) == 16 {
        return None;
    }

    // free cells in column-major order, like np.nonzero(a.T == 0)
    let free: Vec<(usize, usize)> = (0..4)
        .flat_map(|x| (0..4).map(move |y| (x, y)))
        .filter(|&(x, y)| a[y][x] == 0)
        .collect();
    let (x, y) = free[(seed % free.len() as u64) as usize];
    a[y][x] = value;

    Some(a)
}

fn count_nonzero(a: &Grid) -> i32 {
    a.iter().flatten().filter(|&&v| v != 0).count() as i32
}

/// Index of the first maximum, like np.argmax.
fn argmax(line: [i32; 4]) -> i32 {
    let mut best = 0;
    for i in 1..4 {
        if line[i] > line[best] {
            best = i;
        }
    }
    best as i32
}

fn cmp(x: &State) -> i32 {
    let (a, d) = x;
    let nz = count_nonzero(a);
    let mx: i32 = (0..4).map(|c| argmax([a[0][c], a[1][c], a[2][c], a[3][c]])).sum();
    let my: i32 = (0..4).map(|r| argmax(a[r])).sum();
    let d = d.as_bytes();
    let n = d.len();
    let pd = 12 * (n > 3 && d[n - 1] == d[n - 2] && d[n - 2] == d[n - 3]) as i32;

    nz + pd + mx + my * 4
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut i = 0;
    loop {
        let Some(seed_line) = lines.next() else { break };
        let seed: u64 = seed_line.trim().parse().unwrap();
        let _score: i64 = lines.next().unwrap().trim().parse().unwrap();

        let mut a: Grid = [[0; 4]; 4];
        for row in a.iter_mut() {
            let line = lines.next().unwrap();
            for (cell, v) in row.iter_mut().zip(line.split_whitespace()) {
                *cell = v.parse().unwrap();
            }
        }
        let ts = Instant::now();

        let mut predicted_seed = seed;
        let mut q: Vec<State> = vec![(a, String::new())];
        for depth in 0..DEPTH {
            i = depth; // the Python loop variable shadows the turn counter
            if ts.elapsed().as_millis() > 22 {
                eprintln!("br {}", ts.elapsed().as_millis());
                break;
            }
            let mut q2: Vec<State> = Vec::with_capacity(q.len() * 4);
            let predicted_value = 2 + 2 * (predicted_seed & 0x10 != 0) as i32;
            for (prev_a, prev_d) in &q {
                for direction in ['U', 'D', 'R', 'L'] {
                    if let Some(g) = make_move(prev_a, direction, predicted_seed, predicted_value) {
                        let mut d = prev_d.clone();
                        d.push(direction);
                        q2.push((g, d));
                    }
                }
            }

            q2.sort_by_cached_key(cmp); // stable, like sorted()
            q2.truncate(WIDTH);
            q = q2;
            predicted_seed = predicted_seed * predicted_seed % 50515093;
        }

        q.sort_by_cached_key(cmp);

        eprintln!("{}", ts.elapsed().as_millis());
        if let Some((_, d)) = q.first() {
            println!("{}", &d[..d.len().min(MOVES)]);
        } else {
            println!("{}", &"RULD"[i % 4..i % 4 + 1]);
        }
        i += 1;
    }
}
