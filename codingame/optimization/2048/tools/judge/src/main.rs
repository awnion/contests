//! Local judge for the CodinGame 2048 optimization game.
//!
//! Minimal Rust port of the official referee
//! (https://github.com/eulerscheZahl/2048, `Board.java` + `Referee.java`):
//! runs a solution as a child process, one game per seed, and reports scores.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------- simulator

const SIZE: usize = 4;
const MODULUS: u64 = 50_515_093;
const MAX_TURNS: u32 = 600;
const DIRS: &str = "URDL";

/// Game state. Cell `(x, y)` (column, row) lives at `grid[x + y * SIZE]`.
struct Board {
    seed: u64,
    grid: [u32; SIZE * SIZE],
    score: u64,
    moves: u64,
    wrong_commands: u32,
    last_invalid: char,
}

impl Board {
    fn new(seed: u64) -> Self {
        let mut b = Board {
            seed,
            grid: [0; SIZE * SIZE],
            score: 0,
            moves: 0,
            wrong_commands: 0,
            last_invalid: ' ',
        };
        b.spawn_tile();
        b.spawn_tile();
        b
    }

    fn spawn_tile(&mut self) {
        // Free cells are enumerated column by column.
        let mut free = [0usize; SIZE * SIZE];
        let mut n = 0;
        for x in 0..SIZE {
            for y in 0..SIZE {
                if self.grid[x + y * SIZE] == 0 {
                    free[n] = x + y * SIZE;
                    n += 1;
                }
            }
        }
        let cell = free[(self.seed % n as u64) as usize];
        self.grid[cell] = if self.seed & 0x10 == 0 { 2 } else { 4 };
        self.seed = self.seed * self.seed % MODULUS;
    }

    /// Slides the grid in direction `dir` (index into `DIRS`), returns the score gained.
    fn apply_move(grid: &mut [u32; SIZE * SIZE], dir: usize) -> u32 {
        const TARGET_START: [i32; 4] = [0, 3, 12, 0];
        const TARGET_STEP: [i32; 4] = [1, 4, 1, 4];
        const SOURCE_STEP: [i32; 4] = [4, -1, -4, 1];
        let mut merged = [false; SIZE * SIZE];
        let mut gained = 0;
        for i in 0..SIZE as i32 {
            let target = TARGET_START[dir] + i * TARGET_STEP[dir];
            for j in 1..SIZE as i32 {
                let mut src = (target + j * SOURCE_STEP[dir]) as usize;
                if grid[src] == 0 {
                    continue;
                }
                for k in (0..j).rev() {
                    let mid = (target + k * SOURCE_STEP[dir]) as usize;
                    if grid[mid] == 0 {
                        grid[mid] = grid[src];
                        grid[src] = 0;
                        src = mid;
                    } else {
                        if !merged[mid] && grid[mid] == grid[src] {
                            grid[src] = 0;
                            grid[mid] *= 2;
                            merged[mid] = true;
                            gained += grid[mid];
                        }
                        break;
                    }
                }
            }
        }
        gained
    }

    fn can_move_dir(&self, dir: usize) -> bool {
        if self.wrong_commands == 10 {
            return false;
        }
        let mut g = self.grid;
        Self::apply_move(&mut g, dir);
        g != self.grid
    }

    fn can_move(&self) -> bool {
        (0..4).any(|d| self.can_move_dir(d))
    }

    /// Applies one line of player output. `Err` means the game is lost.
    fn play_turn(&mut self, action: &str) -> Result<(), String> {
        if action.is_empty() {
            return Err("no action given".into());
        }
        for c in action.chars().flat_map(char::to_uppercase) {
            if c == '-' {
                continue; // "disable the viewer" flag
            }
            let Some(dir) = DIRS.find(c) else {
                return Err(format!("unknown command: {c}"));
            };
            if !self.can_move_dir(dir) {
                // The rest of the plan is dropped. Repeating the same invalid
                // action 10 times in a row ends the game (see `can_move_dir`).
                if c == self.last_invalid {
                    self.wrong_commands += 1;
                }
                self.last_invalid = c;
                return Ok(());
            }
            self.wrong_commands = 0;
            self.score += Self::apply_move(&mut self.grid, dir) as u64;
            self.moves += 1;
            self.spawn_tile();
        }
        Ok(())
    }

    /// The six input lines the player receives each turn.
    fn input(&self) -> String {
        let mut s = format!("{}\n{}\n", self.seed, self.score);
        for row in self.grid.chunks(SIZE) {
            let cells: Vec<String> = row.iter().map(u32::to_string).collect();
            s.push_str(&cells.join(" "));
            s.push('\n');
        }
        s
    }
}

// ------------------------------------------------------------------- runner

struct Opts {
    seeds: Vec<u64>,
    jobs: usize,
    first_ms: u64,
    turn_ms: u64,
    enforce: bool,
    show_stderr: bool,
    cmd: Vec<String>,
}

struct GameResult {
    seed: u64,
    /// Score on the board when the game stopped.
    score: u64,
    moves: u64,
    turns: u32,
    first_ms: f64,
    /// Slowest turn, the first one excluded.
    max_ms: f64,
    /// Turns over their time limit (only with `--no-timeout`, otherwise the game ends).
    slow_turns: u32,
    /// `Err` = game lost, counts as 0 points.
    end: Result<&'static str, String>,
}

impl GameResult {
    fn points(&self) -> u64 {
        if self.end.is_ok() {
            self.score
        } else {
            0
        }
    }
}

/// Upper bound for one answer when limits are not enforced, so a hung solution cannot block forever.
const HARD_LIMIT: Duration = Duration::from_secs(60);

fn run_game(seed: u64, opts: &Opts) -> GameResult {
    let mut board = Board::new(seed);
    let mut res = GameResult {
        seed,
        score: 0,
        moves: 0,
        turns: 0,
        first_ms: 0.0,
        max_ms: 0.0,
        slow_turns: 0,
        end: Ok(""),
    };

    let spawned = Command::new(&opts.cmd[0])
        .args(&opts.cmd[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(if opts.show_stderr { Stdio::inherit() } else { Stdio::null() })
        .spawn();
    let mut child = match spawned {
        Ok(c) => c,
        Err(e) => {
            res.end = Err(format!("cannot start `{}`: {e}", opts.cmd[0]));
            return res;
        }
    };
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    res.end = loop {
        if !board.can_move() {
            break Ok(if board.wrong_commands == 10 { "invalid action x10" } else { "no moves left" });
        }
        if res.turns == MAX_TURNS {
            break Ok("600 turns");
        }
        res.turns += 1;
        let first = res.turns == 1;
        let limit = Duration::from_millis(if first { opts.first_ms } else { opts.turn_ms });

        let start = Instant::now();
        if stdin.write_all(board.input().as_bytes()).and_then(|_| stdin.flush()).is_err() {
            break Err("solution exited".into());
        }
        let answer = rx.recv_timeout(if opts.enforce { limit } else { HARD_LIMIT });
        let elapsed = start.elapsed();
        let ms = elapsed.as_secs_f64() * 1e3;
        if first {
            res.first_ms = ms;
        } else {
            res.max_ms = res.max_ms.max(ms);
        }
        let action = match answer {
            Ok(a) => a,
            Err(mpsc::RecvTimeoutError::Timeout) => break Err(format!("timeout on turn {}", res.turns)),
            Err(mpsc::RecvTimeoutError::Disconnected) => break Err("solution exited".into()),
        };
        if elapsed > limit {
            res.slow_turns += 1;
        }
        if let Err(e) = board.play_turn(action.trim_end_matches(['\r', '\n'])) {
            break Err(e);
        }
    };

    let _ = child.kill();
    let _ = child.wait();
    res.score = board.score;
    res.moves = board.moves;
    res
}

const DEFAULT_SEEDS: &str = include_str!("../../seeds.txt");

const USAGE: &str = "\
Local judge for the CodinGame 2048 optimization game.

Usage: judge [OPTIONS] <COMMAND> [ARGS]...

Runs COMMAND once per seed, plays a full game against it and prints the scores.
Options must come before COMMAND.

Options:
  -s, --seed <N>        Play this seed (repeatable). Default: the 30 validator seeds
      --seeds <FILE>    Read seeds from FILE (one per line, `#` comments)
  -j, --jobs <N>        Games in parallel [default: half of the cores]
      --first-ms <MS>   Time limit of the first turn [default: 1000]
      --turn-ms <MS>    Time limit of the later turns [default: 50]
      --no-timeout      Do not fail on slow turns, only count them
      --stderr          Show the solution's stderr
  -h, --help            Print help
";

fn parse_seeds(text: &str) -> Result<Vec<u64>, String> {
    text.lines()
        .map(|l| l.split('#').next().unwrap().trim())
        .filter(|l| !l.is_empty())
        .map(parse_seed)
        .collect()
}

fn parse_seed(s: &str) -> Result<u64, String> {
    // The referee reads the seed as a non-negative Java int.
    match s.parse::<u64>() {
        Ok(n) if n <= i32::MAX as u64 => Ok(n),
        _ => Err(format!("bad seed `{s}` (expected 0..=2147483647)")),
    }
}

fn parse_args() -> Result<Opts, String> {
    let mut opts = Opts {
        seeds: Vec::new(),
        jobs: thread::available_parallelism().map_or(1, |n| (n.get() / 2).max(1)),
        first_ms: 1000,
        turn_ms: 50,
        enforce: true,
        show_stderr: false,
        cmd: Vec::new(),
    };
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut value = |name: &str| args.next().ok_or(format!("{name} needs a value"));
        let num = |name: &str, v: String| v.parse::<u64>().map_err(|_| format!("{name}: bad number `{v}`"));
        match a.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                std::process::exit(0);
            }
            "-s" | "--seed" => opts.seeds.push(parse_seed(&value(&a)?)?),
            "--seeds" => {
                let path = value(&a)?;
                let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
                opts.seeds.extend(parse_seeds(&text)?);
            }
            "-j" | "--jobs" => opts.jobs = num(&a, value(&a)?)?.max(1) as usize,
            "--first-ms" => opts.first_ms = num(&a, value(&a)?)?,
            "--turn-ms" => opts.turn_ms = num(&a, value(&a)?)?,
            "--no-timeout" => opts.enforce = false,
            "--stderr" => opts.show_stderr = true,
            "--" => {
                opts.cmd = args.collect();
                break;
            }
            s if s.starts_with('-') => return Err(format!("unknown option `{s}`")),
            _ => {
                opts.cmd = std::iter::once(a).chain(args).collect();
                break;
            }
        }
    }
    if opts.cmd.is_empty() {
        return Err("no solution command given".into());
    }
    if opts.seeds.is_empty() {
        opts.seeds = parse_seeds(DEFAULT_SEEDS)?;
    }
    Ok(opts)
}

fn main() {
    let opts = match parse_args() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}\n\n{USAGE}");
            std::process::exit(2);
        }
    };

    let started = Instant::now();
    let total = opts.seeds.len();
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let results: Mutex<Vec<(usize, GameResult)>> = Mutex::new(Vec::new());
    thread::scope(|s| {
        for _ in 0..opts.jobs.min(total) {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= total {
                    break;
                }
                let r = run_game(opts.seeds[i], &opts);
                let n = done.fetch_add(1, Ordering::Relaxed) + 1;
                eprintln!("[{n}/{total}] seed {} -> {}", r.seed, r.points());
                results.lock().unwrap().push((i, r));
            });
        }
    });
    let mut results = results.into_inner().unwrap();
    results.sort_by_key(|(i, _)| *i);

    println!(
        "{:>10} {:>9} {:>7} {:>5} {:>9} {:>8}  status",
        "seed", "score", "moves", "turns", "first ms", "max ms"
    );
    for (_, r) in &results {
        let mut status = match &r.end {
            Ok(why) => format!("ok: {why}"),
            Err(why) => format!("LOST: {why}"),
        };
        if r.slow_turns > 0 {
            status.push_str(&format!(", {} slow turns", r.slow_turns));
        }
        println!(
            "{:>10} {:>9} {:>7} {:>5} {:>9.1} {:>8.1}  {status}",
            r.seed, r.score, r.moves, r.turns, r.first_ms, r.max_ms
        );
    }

    let points: Vec<u64> = results.iter().map(|(_, r)| r.points()).collect();
    let sum: u64 = points.iter().sum();
    let lost = results.iter().filter(|(_, r)| r.end.is_err()).count();
    println!();
    println!(
        "games {total}, lost {lost}, total {sum}, mean {:.1}, min {}, max {}, wall {:.1} s",
        sum as f64 / total as f64,
        points.iter().min().unwrap(),
        points.iter().max().unwrap(),
        started.elapsed().as_secs_f64()
    );
    if lost > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example from the game statement.
    #[test]
    fn statement_example() {
        let mut g = [4, 4, 4, 0, 2, 0, 2, 2, 4, 4, 0, 4, 0, 2, 2, 4];
        let gained = Board::apply_move(&mut g, DIRS.find('L').unwrap());
        assert_eq!(g, [8, 4, 0, 0, 4, 2, 0, 0, 8, 4, 0, 0, 4, 4, 0, 0]);
        assert_eq!(gained, 8 + 4 + 8 + 4);
    }

    /// Games recorded from the original `Board.java`, driven like `Referee.java`.
    /// Line format: `seed|turns|moves|<final input, space separated>|<action per turn>`.
    #[test]
    fn matches_reference_referee() {
        let traces = include_str!("reference_traces.txt");
        assert!(traces.lines().count() > 100);
        for line in traces.lines() {
            let f: Vec<&str> = line.split('|').collect();
            let mut board = Board::new(f[0].parse().unwrap());
            let mut actions = f[4].split(' ').filter(|a| !a.is_empty());
            let mut turns = 0;
            while board.can_move() && turns < MAX_TURNS {
                turns += 1;
                board.play_turn(actions.next().expect("game ended too late")).unwrap();
            }
            assert!(actions.next().is_none(), "game ended too early: seed {}", f[0]);
            assert_eq!(turns.to_string(), f[1], "turns, seed {}", f[0]);
            assert_eq!(board.moves.to_string(), f[2], "moves, seed {}", f[0]);
            let input = board.input().split_whitespace().collect::<Vec<_>>().join(" ");
            assert_eq!(input, f[3], "final state, seed {}", f[0]);
        }
    }
}
