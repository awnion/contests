# Problem F: speed and memory report

[gym 102906 F](https://codeforces.com/gym/102906/problem/F), limits `1 s` / `512 MB`

- **Story and honest bench:** [TLDR.md](TLDR.md) (styled version: `TLDR.html`)
- **Here:** the numbers behind it

## Rust vs C++, same algorithm

Local judge, M2 Max, 82 tests, per-test minimum of 5 runs. "Contest" is `f_layers_static`, "local" is `f_layers_do_not_submit` (`SIMD`: ARM `NEON` vector instructions, 8 cells at once).

| | Rust contest | C++ contest | Rust local | C++ local |
|---|---:|---:|---:|---:|
| Worst test (`k = 1`) | 34.1 ms | 31.4 ms | 14.4 ms | 🏆 **14.1 ms** |
| `k = 2`, answer 2501 | 18.6 ms | 16.8 ms | 8.5 ms | 🏆 **8.0 ms** |
| Sum over 82 tests | 409 ms | 369 ms | 282 ms | 🏆 **261 ms** |
| Lightest test | 2.2 ms | 2.0 ms | 2.2 ms | 2.0 ms |

- same algorithm → `Rust` is **2–9% slower** on the worst test
- an empty process costs ~2 ms; `Rust` loses ~0.2 ms there on every test
- local vs contest version: **2.2–2.4x faster** in both languages

## Solutions in the repository

| File | Idea | Complexity |
|---|---|---|
| `solutions/f.rs` | BFS over `(i, j)` states in a `HashMap` | `O(n·m·k)` |
| `solutions/f_dfs.rs`, `solutions/f_cpp/f_dfs.cpp`, `solutions/f.kt` | memoised DFS over reachable `(i, j)` | `O(states·k)` |
| `solutions/f_cpp/f.cpp`, `solutions/f.py` | layers: `dp[len][i]` = furthest position in `B` | `O(ans·n·k)` |
| `rust/f.rs` | full `(i, j)` table, running minimum via buckets | `O(n·m)` |
| `rust/f_grid_parity.rs` | full `(i, j)` table, counter instead of buckets, `1 bit` per cell | `O(n·m)` |
| `rust/f_layers_heap.rs` | layers with suffix maximum, corner bitmask in a `Vec` | `O(ans·(m + k))` |
| `rust/f_layers_static.rs`, `cpp/f_layers_static.cpp` | same, bitmask in a `static` array → **submitted** | `O(ans·(m + k))` |
| `rust/f_layers_do_not_submit.rs`, `cpp/f_layers_do_not_submit.cpp` | one in-place pass per layer, `SIMD` (ARM `NEON`: 8 cells per instruction) → **local only** | `O(ans·(m + k))` |
| `cpp/f_layers_simd.cpp` | same with three paths: `SIMD` for x86 (`SSE4.1`), `SIMD` for ARM (`NEON`), plain scalar → **submittable, not submitted yet** | `O(ans·(m + k))` |
| `submissions/` | all 10 accepted `C++` / `Rust` submissions as downloaded from Codeforces | |

Run any of them: `make judge-rs-<name>` / `make judge-cpp-<name>` (`make help` lists all).

Two ideas behind the new files:

- **Grid parity:** `dp[i][j]` is monotone and changes by at most 1 between neighbours
  - all `k` transitions out of a cell are `min` or `min + 1` → a counter instead of buckets
  - `dp mod 2` is enough for both the sweep and the backtrack
- **Layers:** state `(i, j)` is dominated by any `(i' ≥ i, j' ≥ j)`
  - a layer collapses to its suffix maximum `g` → `dp[l + 1][p] = nextB(g[prev[p]], a[p])`
  - layer cost: `O(m + k)` instead of `O(m·k)`
  - a back pointer always targets a corner of the staircase → `1 bit` per `(layer, position)`

## Codeforces data

- public API: `contest.status?contestId=102906`, snapshot of 2026-11-21
- problem F only: **633** submissions, **40** with 100 points, by **26** authors
- dates are `UTC+1`, memory is the Codeforces value in MB

### Top 10 by time

| # | Date | Submission | Author | Compiler | Time, ms | RAM, MB |
|---:|---|---|---|---|---:|---:|
| 1 | 2026-11-21 | 399286083 | **blinovsv** | C++23 (GCC 14-64, msys2) | **78** | **11.8** |
| 2 | 2026-11-21 | 399283895 | **blinovsv** | Rust 2024 | **93** | **3.0** |
| 3 | 2021-02-25 | 108495126 | **blinovsv** | C++17 (GCC 7-32) | **171** | **478.8** |
| 4 | 2021-08-13 | 125746507 | Tima10 | C++17 (GCC 9-64) | 171 | 482.9 |
| 5 | 2021-08-13 | 125747642 | acm | C++17 (GCC 9-64) | 171 | 482.9 |
| 6 | 2026-05-08 | 373867727 | **blinovsv** | Rust 2024 | **187** | **59.1** |
| 7 | 2025-07-08 | 328154273 | suuuiii | C++20 (GCC 13-64) | 202 | 188.5 |
| 8 | 2025-01-10 | 300420313 | klxwn_leg | C++23 (GCC 14-64, msys2) | 218 | 188.7 |
| 9 | 2026-05-08 | 373867536 | **blinovsv** | Rust 2024 | **218** | **104.3** |
| 10 | 2026-05-08 | 373867915 | **blinovsv** | Rust 2024 | **218** | **56.3** |

### Best per language

| Lang | # | Date | Submission | Author | Compiler | Time, ms | RAM, MB |
|---|---:|---|---|---|---|---:|---:|
| C++ | 1 | 2026-11-21 | 399286083 | **blinovsv** | C++23 (GCC 14-64, msys2) | **78** | **11.8** |
| C++ | 2 | 2021-08-13 | 125746507 | Tima10 | C++17 (GCC 9-64) | 171 | 482.9 |
| C++ | 3 | 2021-08-13 | 125747642 | acm | C++17 (GCC 9-64) | 171 | 482.9 |
| C++ | 4 | 2025-07-08 | 328154273 | suuuiii | C++20 (GCC 13-64) | 202 | 188.5 |
| Rust | 1 | 2026-11-21 | 399283895 | **blinovsv** | Rust 2024 | **93** | **3.0** |
| Java | 1 | 2022-12-10 | 184556532 | Alexander_Kozhevnikov | Java 17 | 421 | 270.9 |
| Java | 2 | 2023-07-25 | 215497856 | Stealty_Sorcerer, maksonchik20 | Java 17 | 452 | 272.6 |
| Kotlin | 1 | 2020-12-26 | 102405455 | LeoPro | Kotlin 1.4 | 670 | 507.3 |
| Kotlin | 2 | 2021-03-09 | 109431966 | **blinovsv** | Kotlin 1.4 | **686** | **378.2** |
| PyPy | 1 | 2021-02-18 | 107882415 | **blinovsv** | PyPy 3 | **967** | **195.0** |

### Record progression: time

| Date | Submission | Author | Compiler | Time, ms | RAM, MB |
|---|---|---|---|---:|---:|
| 2020-12-23 | 102137407 | Makcum888 | C++17 (GCC 7-32) | 452 | 406.6 |
| 2020-12-24 | 102189217 | withoutA | C++17 (GCC 7-32) | 436 | 384.2 |
| 2021-01-15 | 104430586 | JulianFernando | C++17 (GCC 7-32) | 389 | 287.5 |
| 2021-02-25 | 108495126 | **blinovsv** | C++17 (GCC 7-32) | 171 | 478.8 |
| 2026-11-21 | 399283895 | **blinovsv** | Rust 2024 | 93 | 3.0 |
| 2026-11-21 | 399286083 | **blinovsv** | C++23 (GCC 14-64, msys2) | 78 | 11.8 |

- `Rust` only: 997 ms → 374 ms → 358 ms → 218 ms → 187 ms → 93 ms

- `C++` only: 452 ms → 436 ms → 389 ms → 171 ms → 78 ms

### Record progression: memory

| Date | Submission | Author | Compiler | Time, ms | RAM, MB |
|---|---|---|---|---:|---:|
| 2020-12-23 | 102137407 | Makcum888 | C++17 (GCC 7-32) | 452 | 406.6 |
| 2020-12-24 | 102189217 | withoutA | C++17 (GCC 7-32) | 436 | 384.2 |
| 2020-12-26 | 102416648 | LeoPro | Kotlin 1.4 | 701 | 122.9 |
| 2021-08-31 | 127512384 | UncleSema | C++17 (GCC 7-32) | 967 | 81.0 |
| 2026-05-08 | 373867727 | **blinovsv** | Rust 2024 | 187 | 59.1 |
| 2026-05-08 | 373867915 | **blinovsv** | Rust 2024 | 218 | 56.3 |
| 2026-11-21 | 399283895 | **blinovsv** | Rust 2024 | 93 | 3.0 |

- `Rust` only: 143.8 MB → 104.3 MB → 59.1 MB → 56.3 MB → 3.0 MB

- `C++` only: 406.6 MB → 384.2 MB → 287.9 MB → 287.5 MB → 81.0 MB → 11.8 MB

### Own accepted submissions (12 of 37)

| Date | Submission | Compiler | Time, ms | RAM, MB |
|---|---|---|---:|---:|
| 2021-02-18 | 107882415 | PyPy 3 | 967 | 195.0 |
| 2021-02-25 | 108495126 | C++17 (GCC 7-32) | 171 | 478.8 |
| 2021-02-26 | 108532680 | Rust | 997 | 143.8 |
| 2021-02-27 | 108608540 | Rust | 374 | 143.8 |
| 2021-03-09 | 109431966 | Kotlin 1.4 | 686 | 378.2 |
| 2022-06-27 | 161961135 | Rust 2021 | 358 | 145.3 |
| 2022-06-27 | 161974857 | Rust 2021 | 373 | 145.3 |
| 2026-05-08 | 373867536 | Rust 2024 | 218 | 104.3 |
| 2026-05-08 | 373867727 | Rust 2024 | 187 | 59.1 |
| 2026-05-08 | 373867915 | Rust 2024 | 218 | 56.3 |
| 2026-11-21 | 399283895 | Rust 2024 | 93 | 3.0 |
| 2026-11-21 | 399286083 | C++23 (GCC 14-64, msys2) | 78 | 11.8 |

### All 40 full-score submissions by time

| # | Date | Submission | Author | Compiler | Time, ms | RAM, MB |
|---:|---|---|---|---|---:|---:|
| 1 | 2026-11-21 | 399286083 | **blinovsv** | C++23 (GCC 14-64, msys2) | 78 | 11.8 |
| 2 | 2026-11-21 | 399283895 | **blinovsv** | Rust 2024 | 93 | 3.0 |
| 3 | 2021-02-25 | 108495126 | **blinovsv** | C++17 (GCC 7-32) | 171 | 478.8 |
| 4 | 2021-08-13 | 125746507 | Tima10 | C++17 (GCC 9-64) | 171 | 482.9 |
| 5 | 2021-08-13 | 125747642 | acm | C++17 (GCC 9-64) | 171 | 482.9 |
| 6 | 2026-05-08 | 373867727 | **blinovsv** | Rust 2024 | 187 | 59.1 |
| 7 | 2025-07-08 | 328154273 | suuuiii | C++20 (GCC 13-64) | 202 | 188.5 |
| 8 | 2025-01-10 | 300420313 | klxwn_leg | C++23 (GCC 14-64, msys2) | 218 | 188.7 |
| 9 | 2026-05-08 | 373867536 | **blinovsv** | Rust 2024 | 218 | 104.3 |
| 10 | 2026-05-08 | 373867915 | **blinovsv** | Rust 2024 | 218 | 56.3 |
| 11 | 2022-06-27 | 161961135 | **blinovsv** | Rust 2021 | 358 | 145.3 |
| 12 | 2024-11-28 | 293643603 | Diall_ | C++20 (GCC 13-64) | 359 | 475.9 |
| 13 | 2022-06-27 | 161974857 | **blinovsv** | Rust 2021 | 373 | 145.3 |
| 14 | 2021-02-27 | 108608540 | **blinovsv** | Rust | 374 | 143.8 |
| 15 | 2025-08-12 | 333718676 | MisterGu | C++20 (GCC 13-64) | 374 | 262.0 |
| 16 | 2025-10-14 | 343621665 | FFTotoro | C++20 (GCC 13-64) | 374 | 296.0 |
| 17 | 2021-01-15 | 104430586 | JulianFernando | C++17 (GCC 7-32) | 389 | 287.5 |
| 18 | 2023-06-27 | 211289021 | Shefin_ | C++20 (GCC 11-64) | 405 | 480.4 |
| 19 | 2024-11-27 | 293485524 | Damirca | C++20 (GCC 13-64) | 405 | 475.7 |
| 20 | 2022-12-10 | 184556532 | Alexander_Kozhevnikov | Java 17 | 421 | 270.9 |
| 21 | 2024-08-09 | 275451150 | crowdforces | C++20 (GCC 13-64) | 421 | 287.3 |
| 22 | 2020-12-24 | 102189217 | withoutA | C++17 (GCC 7-32) | 436 | 384.2 |
| 23 | 2024-08-09 | 275440612 | Maram | C++20 (GCC 13-64) | 436 | 287.3 |
| 24 | 2021-01-10 | 103960843 | Kaurker | C++17 (GCC 7-32) | 451 | 407.0 |
| 25 | 2020-12-23 | 102137407 | Makcum888 | C++17 (GCC 7-32) | 452 | 406.6 |
| 26 | 2020-12-23 | 102141254 | alkr | C++17 (GCC 7-32) | 452 | 432.2 |
| 27 | 2023-07-25 | 215497856 | Stealty_Sorcerer, maksonchik20 | Java 17 | 452 | 272.6 |
| 28 | 2025-01-10 | 300419866 | klxwn_leg | Java 21 | 530 | 263.6 |
| 29 | 2020-12-25 | 102275736 | daniil-eshkin | C++17 (GCC 7-32) | 592 | 480.2 |
| 30 | 2024-07-19 | 271384971 | crowdforces | C++20 (GCC 13-64) | 593 | 380.3 |
| 31 | 2024-11-27 | 293514695 | Qutef | Java 21 | 593 | 265.3 |
| 32 | 2020-12-26 | 102405455 | LeoPro | Kotlin 1.4 | 670 | 507.3 |
| 33 | 2021-03-09 | 109431966 | **blinovsv** | Kotlin 1.4 | 686 | 378.2 |
| 34 | 2020-12-26 | 102416648 | LeoPro | Kotlin 1.4 | 701 | 122.9 |
| 35 | 2021-01-03 | 103124624 | DanielNash | C++17 (GCC 7-32) | 733 | 479.3 |
| 36 | 2020-12-27 | 102488872 | vintage_Vlad_Makeev | C++14 (GCC 6-32) | 764 | 287.9 |
| 37 | 2024-10-16 | 286163479 | Aldk | C++20 (GCC 13-64) | 952 | 384.1 |
| 38 | 2021-02-18 | 107882415 | **blinovsv** | PyPy 3 | 967 | 195.0 |
| 39 | 2021-08-31 | 127512384 | UncleSema | C++17 (GCC 7-32) | 967 | 81.0 |
| 40 | 2021-02-26 | 108532680 | **blinovsv** | Rust | 997 | 143.8 |

## Notes

- **`PyPy`:** 13 attempts over six weeks (2021-01-04 → 2021-02-18), passed with 33 ms to spare
- **`Rust`, 2021-02-26 → 2021-02-27:** one commit apart (`0b5427e`), only the index order of the two tables changed
- **`Kotlin`, 2021-03-09:** the accepted run took `686 ms`; the `358 ms` run seven minutes earlier scored **21 points**
- **`C++`, 2021-08-13:** two other authors hit the same `171 ms` with nearly the same memory, six months after 108495126
- **Lowest memory per language:** `Rust` 3.0 MB, `C++` 11.8 MB, `Kotlin` 122.9 MB (LeoPro), `PyPy` 195.0 MB, `Java` 263.6 MB (klxwn_leg)
- **Judge:** time and peak memory come from `wait4`; before that fix every test carried ~27 ms of overhead
