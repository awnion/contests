# 2048

- Game: https://www.codingame.com/multiplayer/optimization/2048

## Protocol

Taken from the statement and the referee source
(https://github.com/eulerscheZahl/2048).

Input, every turn:

```
seed          # current RNG state
score         # current score
r0c0 r0c1 r0c2 r0c3   # 4 lines, 4 integers each; 0 = empty cell
...
```

Output, every turn: one line with one or more moves out of `U`, `D`, `L`, `R`
(case-insensitive), applied in order. A `-` anywhere in the line only disables
the viewer.

Limits: 1 s for the first turn, 50 ms for the later ones, at most 600 turns
(lines of output) per game — hence several moves per line.

The tile spawn is deterministic, so the whole game can be simulated offline:

- new tile value: `4` if `seed & 0x10` else `2`
- new tile position: the `seed % free_count`-th empty cell, counting empty
  cells in column-major order (`x` outer, `y` inner)
- next seed: `seed * seed % 50515093`
- the game starts with two spawns from the test seed
- a tile merges at most once per move; a merge scores the new tile value
- a move that does not change the board is invalid: no spawn, and the rest of
  the line is dropped. Sending the same invalid move 10 times in a row (with
  no valid move in between) ends the game with the current score
- the game also ends, keeping the score, when no move is possible or after
  600 turns
- an empty line, an unknown character or a timeout loses the game. Unverified:
  that a lost game scores 0 (the referee reports points only on a normal end)

## Solutions

| File | Year | Language | Approach | Result |
| --- | --- | --- | --- | --- |
| [numpy_beam_search.py](python/numpy_beam_search.py) | ~2020 | Python + NumPy | Beam search over the exact simulation (depth 11, width 15), emits the first 8 moves of the best line; ~22 ms budget per turn | not recorded |
| [beam_search.py](python/beam_search.py) | 2026 | Python, no deps | Same algorithm and parameters as `numpy_beam_search.py`, board as a 16-int tuple, memoized row squash; identical moves on 400 random boards, about 4x faster locally (2.4 ms vs 9.6 ms per search) | not submitted |
| [beam_search.rs](rust/src/bin/beam_search.rs) | 2026 | Rust | 1:1 port of `numpy_beam_search.py`, quirks included; same moves, under 1 ms per turn locally, so the 22 ms cutoff never fires | not submitted; local judge: mean 91501.6 |

## Local judge

[tools/judge/](tools/judge/) is a minimal Rust port of the referee. It starts
the solution as a child process, plays one game per seed and prints a score
table. [tools/seeds.txt](tools/seeds.txt) holds the 30 validator seeds of the
game and is the default seed list.

```sh
cd optimization/2048
make rust   BIN=beam_search          # builds and judges rust/src/bin/beam_search.rs
make python SOL=numpy_beam_search    # judges python/numpy_beam_search.py, one game at a time
make rust   BIN=beam_search ARGS="-s 290797 --stderr"   # ARGS = judge options
make help                            # judge options
make test                            # judge self-tests
```

Without make: `cargo run --release --manifest-path tools/judge/Cargo.toml --
[OPTIONS] <COMMAND> [ARGS]...`.

- Everything after the options is the solution command; it is run as is, from
  the current directory.
- Time limits are enforced as on the site (1000 ms / 50 ms, `--first-ms`,
  `--turn-ms`); a slow turn loses the game. `--no-timeout` only counts slow
  turns.
- Games run in parallel on half of the cores by default. For a solution that
  uses its time budget, use `-j 1`: parallel games slow each other down and
  cause timeouts.
- A lost game counts as 0 points; the exit code is 1 if any game was lost.
- The summary prints both the total and the mean over the seeds (which of the
  two the site ranks by is unverified).
- Local scores are not the site's scores, see "Local results" in
  [AGENTS.md](../../AGENTS.md).
- `make test` replays 156 games
  recorded from the original `Board.java`
  ([reference_traces.txt](tools/judge/src/reference_traces.txt)) and checks
  the final state of each.

## Notes

- Local judge, 30 validator seeds, `beam_search.rs`: 26 of 30 games end with
  "same invalid move 10 times", not with a full board — the fallback move gets
  stuck (see the `i` shadowing quirk below). `numpy_beam_search.py` timed out
  (50 ms) on some seeds locally.

- The Rust port was checked against the Python version (with its time cutoff
  disabled) on 400 random boards: identical output. On CodinGame the two can
  still differ, because Python may hit the 22 ms cutoff and search less deep.

- `numpy_beam_search.py` is kept as originally written. Known quirks:
  - `np.mat` was removed in NumPy 2.0, so it only runs on NumPy 1.x (fine on
    CodinGame and with the pinned `python/pyproject.toml`).
  - The turn counter `i` is shadowed by the depth loop variable, so the
    fallback move `'RULD'[i % 4]` does not actually cycle per turn.
  - The heuristic (`cmp`) only looks at tile count and where the max of each
    row/column sits; score is not used.
