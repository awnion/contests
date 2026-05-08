use std::fmt::Write;
use std::io::{self, Read};
use std::mem::MaybeUninit;

const MAX_K: usize = 5000;
const MAX_M: usize = 5000;
const MAX_N: usize = 5000;
const MAX_ROWS: usize = MAX_M + 2;
const MAX_COLS: usize = MAX_N + 2;
const MAX_ANSWER: usize = MAX_M + MAX_N + 2;
const MAX_VALUE: usize = MAX_ANSWER + 1;
const SMALL_K_LIMIT: usize = 3;

struct Solver {
    k: usize,
    m: usize,
    n: usize,
    cols: usize,
    fail_row: usize,
    fail_col: usize,
    a: [u16; MAX_ROWS],
    b: [u16; MAX_COLS],
    first_a: [u16; MAX_K + 1],
    first_b: [u16; MAX_K + 1],
    last_a: [u16; MAX_K + 1],
    last_b: [u16; MAX_K + 1],
    cnt_a: [u16; MAX_K + 1],
    cnt_b: [u16; MAX_K + 1],
    dp: Vec<MaybeUninit<u16>>,
}

impl Solver {
    #[inline(always)]
    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    #[inline(always)]
    fn pair_in_a(&self, x: usize, y: usize) -> bool {
        if x == y {
            self.cnt_a[x] >= 2
        } else {
            self.first_a[x] != 0 && self.last_a[y] != 0 && self.first_a[x] < self.last_a[y]
        }
    }

    #[inline(always)]
    fn pair_in_b(&self, x: usize, y: usize) -> bool {
        if x == y {
            self.cnt_b[x] >= 2
        } else {
            self.first_b[x] != 0 && self.last_b[y] != 0 && self.first_b[x] < self.last_b[y]
        }
    }

    #[inline(always)]
    fn bucket_remove(
        head: &mut [u16; MAX_VALUE],
        prev: &mut [u16; MAX_K + 1],
        next: &mut [u16; MAX_K + 1],
        x: usize,
        old_value: usize,
    ) {
        let p = prev[x] as usize;
        let q = next[x] as usize;
        if p == 0 {
            head[old_value] = q as u16;
        } else {
            next[p] = q as u16;
        }
        if q != 0 {
            prev[q] = p as u16;
        }
    }

    #[inline(always)]
    fn bucket_insert(
        head: &mut [u16; MAX_VALUE],
        prev: &mut [u16; MAX_K + 1],
        next: &mut [u16; MAX_K + 1],
        used: &mut [bool; MAX_VALUE],
        touched: &mut [u16; MAX_VALUE],
        touched_len: &mut usize,
        x: usize,
        new_value: usize,
    ) {
        if !used[new_value] {
            used[new_value] = true;
            touched[*touched_len] = new_value as u16;
            *touched_len += 1;
        }

        let h = head[new_value] as usize;
        prev[x] = 0;
        next[x] = h as u16;
        if h != 0 {
            prev[h] = x as u16;
        }
        head[new_value] = x as u16;
    }

    fn build_dp(&mut self) {
        let mut next_a_base = [0usize; MAX_K + 1];
        let mut value = [0u16; MAX_K + 1];
        let mut prev = [0u16; MAX_K + 1];
        let mut next = [0u16; MAX_K + 1];
        let mut head = [0u16; MAX_VALUE];
        let mut used = [false; MAX_VALUE];
        let mut touched = [0u16; MAX_VALUE];
        let mut touched_len = 0usize;

        let fail_row_base = self.fail_row * self.cols;
        unsafe {
            self.dp
                .get_unchecked_mut(fail_row_base + self.fail_col)
                .write(0);
        }
        for x in 1..=self.k {
            next_a_base[x] = fail_row_base;
        }

        for row in (0..=self.fail_row).rev() {
            let row_base = row * self.cols;

            for i in 0..touched_len {
                let v = touched[i] as usize;
                head[v] = 0;
                used[v] = false;
            }
            touched_len = 0;

            for x in 1..=self.k {
                let v = unsafe {
                    self.dp
                        .get_unchecked(next_a_base[x] + self.fail_col)
                        .assume_init() as usize
                };
                value[x] = v as u16;
                Self::bucket_insert(
                    &mut head,
                    &mut prev,
                    &mut next,
                    &mut used,
                    &mut touched,
                    &mut touched_len,
                    x,
                    v,
                );
            }

            let mut min_value = 0usize;
            while head[min_value] == 0 {
                min_value += 1;
            }

            for col in (0..=self.fail_col).rev() {
                    let id = row_base + col;
                if row == self.fail_row && col == self.fail_col {
                    unsafe {
                        self.dp.get_unchecked_mut(id).write(0);
                    }
                } else {
                    unsafe {
                        self.dp.get_unchecked_mut(id).write((min_value + 1) as u16);
                    }
                }

                if col > 0 && col <= self.n {
                    let x = self.b[col] as usize;
                    let old_value = value[x] as usize;
                    let new_value = unsafe {
                        self.dp
                            .get_unchecked(next_a_base[x] + col)
                            .assume_init() as usize
                    };
                    if new_value != old_value {
                        Self::bucket_remove(&mut head, &mut prev, &mut next, x, old_value);
                        Self::bucket_insert(
                            &mut head,
                            &mut prev,
                            &mut next,
                            &mut used,
                            &mut touched,
                            &mut touched_len,
                            x,
                            new_value,
                        );
                        value[x] = new_value as u16;
                        while head[min_value] == 0 {
                            min_value += 1;
                        }
                    }
                }
            }

            if row > 0 && row <= self.m {
                next_a_base[self.a[row] as usize] = row_base;
            }
        }
    }

    fn build_dp_small(&mut self) {
        let mut next_a_base = [0usize; MAX_K + 1];
        let mut value = [0u16; MAX_K + 1];

        let fail_row_base = self.fail_row * self.cols;
        unsafe {
            self.dp
                .get_unchecked_mut(fail_row_base + self.fail_col)
                .write(0);
        }
        for x in 1..=self.k {
            next_a_base[x] = fail_row_base;
        }

        for row in (0..=self.fail_row).rev() {
            let row_base = row * self.cols;

            let mut min_value = u16::MAX;
            for x in 1..=self.k {
                let v = unsafe {
                    self.dp
                        .get_unchecked(next_a_base[x] + self.fail_col)
                        .assume_init()
                };
                value[x] = v;
                if v < min_value {
                    min_value = v;
                }
            }

            for col in (0..=self.fail_col).rev() {
                let id = row_base + col;
                if row == self.fail_row && col == self.fail_col {
                    unsafe {
                        self.dp.get_unchecked_mut(id).write(0);
                    }
                } else {
                    unsafe {
                        self.dp.get_unchecked_mut(id).write(min_value + 1);
                    }
                }

                if col > 0 && col <= self.n {
                    let x = self.b[col] as usize;
                    let old_value = value[x];
                    let new_value = unsafe {
                        self.dp
                            .get_unchecked(next_a_base[x] + col)
                            .assume_init()
                    };
                    if new_value != old_value {
                        value[x] = new_value;
                        if old_value == min_value {
                            min_value = value[1];
                            for y in 2..=self.k {
                                if value[y] < min_value {
                                    min_value = value[y];
                                }
                            }
                        }
                    }
                }
            }

            if row > 0 && row <= self.m {
                next_a_base[self.a[row] as usize] = row_base;
            }
        }
    }

    fn build_dp_2(&mut self) {
        let mut next_a1_base = self.fail_row * self.cols;
        let mut next_a2_base = next_a1_base;
        let fail_row_base = next_a1_base;

        unsafe {
            self.dp
                .get_unchecked_mut(fail_row_base + self.fail_col)
                .write(0);
        }

        for row in (0..=self.fail_row).rev() {
            let row_base = row * self.cols;
            let mut v1 = unsafe {
                self.dp
                    .get_unchecked(next_a1_base + self.fail_col)
                    .assume_init()
            };
            let mut v2 = unsafe {
                self.dp
                    .get_unchecked(next_a2_base + self.fail_col)
                    .assume_init()
            };
            let mut min_value = if v1 < v2 { v1 } else { v2 };

            unsafe {
                self.dp
                    .get_unchecked_mut(row_base + self.fail_col)
                    .write(if row == self.fail_row { 0 } else { min_value + 1 });
            }

            for col in (1..=self.n).rev() {
                unsafe {
                    self.dp
                        .get_unchecked_mut(row_base + col)
                        .write(min_value + 1);
                }

                if self.b[col] == 1 {
                    let old = v1;
                    v1 = unsafe { self.dp.get_unchecked(next_a1_base + col).assume_init() };
                    if old == min_value {
                        min_value = if v1 < v2 { v1 } else { v2 };
                    }
                } else {
                    let old = v2;
                    v2 = unsafe { self.dp.get_unchecked(next_a2_base + col).assume_init() };
                    if old == min_value {
                        min_value = if v1 < v2 { v1 } else { v2 };
                    }
                }
            }

            unsafe {
                self.dp.get_unchecked_mut(row_base).write(min_value + 1);
            }

            if row > 0 && row <= self.m {
                if self.a[row] == 1 {
                    next_a1_base = row_base;
                } else {
                    next_a2_base = row_base;
                }
            }
        }
    }

    fn next_a_pos(&self, pos: usize, ch: u16) -> usize {
        let mut p = pos + 1;
        while p <= self.m {
            if self.a[p] == ch {
                return p;
            }
            p += 1;
        }
        self.fail_row
    }

    fn next_b_pos(&self, pos: usize, ch: u16) -> usize {
        let mut p = pos + 1;
        while p <= self.n {
            if self.b[p] == ch {
                return p;
            }
            p += 1;
        }
        self.fail_col
    }

    fn solve(&mut self) -> ([u16; MAX_ANSWER], usize) {
        if self.k == 2 {
            self.build_dp_2();
        } else if self.k <= SMALL_K_LIMIT {
            self.build_dp_small();
        } else {
            self.build_dp();
        }

        let mut answer = [0u16; MAX_ANSWER];
        let mut len = 0usize;
        let mut row = 0usize;
        let mut col = 0usize;

        while row != self.fail_row || col != self.fail_col {
            let mut best_ch = 1u16;
            let mut best_value = u16::MAX;

            for x in 1..=self.k {
                let ch = x as u16;
                let nr = if row == self.fail_row {
                    self.fail_row
                } else {
                    self.next_a_pos(row, ch)
                };
                let nc = if col == self.fail_col {
                    self.fail_col
                } else {
                    self.next_b_pos(col, ch)
                };
                let value = unsafe { self.dp.get_unchecked(self.idx(nr, nc)).assume_init() };
                if value < best_value {
                    best_value = value;
                    best_ch = ch;
                    if value == 0 {
                        break;
                    }
                }
            }

            answer[len] = best_ch;
            len += 1;

            if row != self.fail_row {
                row = self.next_a_pos(row, best_ch);
            }
            if col != self.fail_col {
                col = self.next_b_pos(col, best_ch);
            }
        }

        (answer, len)
    }
}

fn run() -> String {
    let mut input = [0u8; 1 << 20];
    let mut len = 0usize;
    loop {
        let read = io::stdin().read(&mut input[len..]).unwrap();
        if read == 0 {
            break;
        }
        len += read;
    }
    let mut scan_pos = 0usize;

    let mut solver = Solver {
        k: 0,
        m: 0,
        n: 0,
        cols: 0,
        fail_row: 0,
        fail_col: 0,
        a: [0; MAX_ROWS],
        b: [0; MAX_COLS],
        first_a: [0; MAX_K + 1],
        first_b: [0; MAX_K + 1],
        last_a: [0; MAX_K + 1],
        last_b: [0; MAX_K + 1],
        cnt_a: [0; MAX_K + 1],
        cnt_b: [0; MAX_K + 1],
        dp: Vec::new(),
    };

    solver.k = next_usize(&input[..len], &mut scan_pos);
    solver.m = next_usize(&input[..len], &mut scan_pos);
    for i in 1..=solver.m {
        let x = next_usize(&input[..len], &mut scan_pos);
        solver.a[i] = x as u16;
        if solver.first_a[x] == 0 {
            solver.first_a[x] = i as u16;
        }
        solver.last_a[x] = i as u16;
        solver.cnt_a[x] += 1;
    }

    solver.n = next_usize(&input[..len], &mut scan_pos);
    for i in 1..=solver.n {
        let x = next_usize(&input[..len], &mut scan_pos);
        solver.b[i] = x as u16;
        if solver.first_b[x] == 0 {
            solver.first_b[x] = i as u16;
        }
        solver.last_b[x] = i as u16;
        solver.cnt_b[x] += 1;
    }

    if solver.m > solver.n {
        let limit = solver.m + 1;
        for i in 0..=limit {
            std::mem::swap(&mut solver.a[i], &mut solver.b[i]);
        }
        for x in 1..=solver.k {
            std::mem::swap(&mut solver.first_a[x], &mut solver.first_b[x]);
            std::mem::swap(&mut solver.last_a[x], &mut solver.last_b[x]);
            std::mem::swap(&mut solver.cnt_a[x], &mut solver.cnt_b[x]);
        }
        std::mem::swap(&mut solver.m, &mut solver.n);
    }

    solver.cols = solver.n + 2;
    solver.fail_row = solver.m + 1;
    solver.fail_col = solver.n + 1;

    if solver.k == 1 {
        let answer_len = solver.m.max(solver.n) + 1;
        let mut out = String::new();
        writeln!(&mut out, "{}", answer_len).unwrap();
        for i in 0..answer_len {
            if i > 0 {
                out.push(' ');
            }
            out.push('1');
        }
        out.push('\n');
        return out;
    }

    for x in 1..=solver.k {
        if solver.last_a[x] == 0 && solver.last_b[x] == 0 {
            let mut out = String::new();
            writeln!(&mut out, "1").unwrap();
            writeln!(&mut out, "{}", x).unwrap();
            return out;
        }
    }

    for x in 1..=solver.k {
        for y in 1..=solver.k {
            if !solver.pair_in_a(x, y) && !solver.pair_in_b(x, y) {
                let mut out = String::new();
                writeln!(&mut out, "2").unwrap();
                writeln!(&mut out, "{} {}", x, y).unwrap();
                return out;
            }
        }
    }

    let total = (solver.fail_row + 1) * solver.cols;
    solver.dp = Vec::with_capacity(total);
    unsafe {
        solver.dp.set_len(total);
    }

    let (answer, answer_len) = solver.solve();

    let mut out = String::new();
    writeln!(&mut out, "{}", answer_len).unwrap();
    for i in 0..answer_len {
        if i > 0 {
            out.push(' ');
        }
        write!(&mut out, "{}", answer[i]).unwrap();
    }
    out.push('\n');
    out
}

fn next_usize(input: &[u8], pos: &mut usize) -> usize {
    while *pos < input.len() && input[*pos] <= b' ' {
        *pos += 1;
    }
    let mut value = 0usize;
    while *pos < input.len() && input[*pos] > b' ' {
        value = value * 10 + (input[*pos] - b'0') as usize;
        *pos += 1;
    }
    value
}

fn main() {
    print!("{}", run());
}
