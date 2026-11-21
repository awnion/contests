# Problem F: speed and memory report

- **Problem:** [gym 102906 F](https://codeforces.com/gym/102906/problem/F)
  - limits: `1 s` / `512 MB`
- **Data:** public Codeforces API (`contest.status?contestId=102906`), snapshot of 2026-11-21
  - only problem F is counted: **632** submissions, **39** with 100 points, by **26** authors
  - dates are `UTC+1`, as shown on the site
  - memory is the Codeforces value converted to MB

## TLDR

Last problem (Russian Olympiad in Informatics 2020—2021, St. Petersburg).
Typical DP hard problem:

- **Symmetric table:** the plain DP over all `(i, j)` pairs.
  - Assumed solution. Relatively easy 120 lines in C++ for experienced contestants.
- **Useful states only:** memoised DFS, or layers by answer length.
  - Noone assumes you can write it online. **But** it's cool to theorycraft around here.
  - `best[l][i]` — the furthest position in `B` reachable in `l` steps ending
    at `i` in `A`.
  - Transition: `best[l + 1][nextA(i, x)] = max(nextB(best[l][i], x))`.

| Date | What happened | Lang | ms | RAM, MB |
| --- | --- | --- | --- | --- |
| 2021-02-18 | **First full score** on `PyPy`!<br>• barely squeezed into `1000 ms` | PyPy 3 | 967 | 195.0 |
| 2021-02-25 | Layered DP in `C++`<br>• **fastest** solution overall for the next **5.7 years** | C++17 | 171 | 478.8 |
| 2021-02-26 | Memoised DFS in `Rust`<br>• next-occurrence tables indexed `[value][position]` (**wrong order**) | Rust | 997 | 143.8 |
| 2021-02-27 | Same code<br>• just tables indexed `[position][value]`<br>• **2.7x faster!!!** | Rust | 374 | 143.8 |
| 2021-03-09 | Memoised DFS in `Kotlin`<br>• `Kotlin` unlike `Rust` isn't that sensitive to the index order<br>• first attempt **good enough** | Kotlin 1.4 | 686 | 378.2 |
| 2022-06-27 | After more than a year I'm trying to improve... | Rust 2021 | 358 | 145.3 |
| 2026-05-08 | Years passed... Revisited the solution with AI: `GPT 5.5` or `5.6`<br>• **almost beat** `C++`<br>• **8x less memory** than `C++`!!! | Rust 2024 | 187 | 59.1 |
| 2026-11-21 | `Fable 5.2`!!! WTF?? **IS THIS WITCHCRAFT**. Ok Ok. In hindsight it's not that hard:<br>• a layer collapses to its **suffix maximum**<br>&nbsp;&nbsp;&nbsp;◦ cost per layer: `O(m + k)` instead of `O(m·k)`<br>&nbsp;&nbsp;&nbsp;◦ **1.8x faster** than `C++`<br>• a back pointer always targets a **"corner"** of the layer<br>&nbsp;&nbsp;&nbsp;◦ `1 bit` per cell instead of `u16`<br>&nbsp;&nbsp;&nbsp;◦ **160x less memory** than `C++`<br>• the bit table is a fixed-size `static` array<br>&nbsp;&nbsp;&nbsp;◦ no heap growth<br>&nbsp;&nbsp;&nbsp;◦ no stack limit | Rust 2024 | **93** | **3.0** |

Notes on the facts:

- **2021-02-26 → 2021-02-27 (`Rust`)**
  - one commit apart: `0b5427e`, "cache efficient optimization"
  - only the index order of the two next-occurrence tables changed
- **2021-03-09 (`Kotlin`)**
  - the accepted run took `686 ms`
  - the `358 ms` run seven minutes earlier scored **21 points**
  - the repository history has no `Kotlin` version with `[value][position]`
    tables, so the index-order remark is **not confirmed by the data**
- **2026-05-08**
  - the `C++` record it chased is, judging by its `478.8 MB`, the layered
    `solutions/f_cpp/f.cpp`, not the DFS one

## What changed in the last step

Submission 399283895 is `rust/f_layers_static.rs`. Against the previous best it
is 1.8x faster (171 ms, C++) and 19x smaller (56.3 MB, Rust).

- **Suffix maximum per layer — the main speed-up.** A state `(i, j)` is
  dominated by any `(i' >= i, j' >= j)`, so a layer collapses to its suffix
  maximum and costs `O(m + k)` instead of `O(m·k)`.
- **One bit instead of `u16` per back pointer.** The source of a transition is
  always a corner of the Pareto staircase, so a bitmask of corners per layer is
  enough. Locally: memory 51 MB → about 3 MB, time −15–20% on small `k`.
- **Fixed-size `static mut` array for the bitmask.** It lives in the zeroed
  data segment, not on the stack, so the main-thread stack size (which cannot
  be configured on Codeforces) does not matter. Locally it gained 0–2 ms over a
  `Vec`; a true on-stack array measured the same and was dropped because of the
  stack-size risk.
- **`nextB` lookup chosen by `k`.** Dense table for small `k`, binary search in
  occurrence lists for large `k`, so there is never an `n × k` table to fill.

The parity trick ("neighbouring `dp` values differ by at most 1, so `dp mod 2`
is enough") belongs to the other family and is in `rust/f_grid_parity.rs`; it
was not submitted.

## Top 10 by execution time

| # | Date | Submission | Author | Compiler | Time, ms | Memory, MB |
|---|---|---|---|---|---|---|
| 1 | 2026-11-21 | 399283895 | **blinovsv** | Rust 2024 | **93** | **3.0** |
| 2 | 2021-02-25 | 108495126 | **blinovsv** | C++17 (GCC 7-32) | **171** | **478.8** |
| 3 | 2021-08-13 | 125746507 | Tima10 | C++17 (GCC 9-64) | 171 | 482.9 |
| 4 | 2021-08-13 | 125747642 | acm | C++17 (GCC 9-64) | 171 | 482.9 |
| 5 | 2026-05-08 | 373867727 | **blinovsv** | Rust 2024 | **187** | **59.1** |
| 6 | 2025-07-08 | 328154273 | suuuiii | C++20 (GCC 13-64) | 202 | 188.5 |
| 7 | 2025-01-10 | 300420313 | klxwn_leg | C++23 (GCC 14-64, msys2) | 218 | 188.7 |
| 8 | 2026-05-08 | 373867536 | **blinovsv** | Rust 2024 | **218** | **104.3** |
| 9 | 2026-05-08 | 373867915 | **blinovsv** | Rust 2024 | **218** | **56.3** |
| 10 | 2022-06-27 | 161961135 | **blinovsv** | Rust 2021 | **358** | **145.3** |

## Best known solution per language

Best time per author, top entries per language. Where the leader is `blinovsv`,
the next authors are listed for comparison.

| Language | Rank | Date | Submission | Author | Compiler | Time, ms | Memory, MB |
|---|---|---|---|---|---|---|---|
| Rust | 1 | 2026-11-21 | 399283895 | blinovsv | Rust 2024 | 93 | 3.0 |
| Rust | — | | | no other author has a full-score Rust submission | | | |
| C++ | 1 | 2021-02-25 | 108495126 | blinovsv | C++17 (GCC 7-32) | 171 | 478.8 |
| C++ | 1 | 2021-08-13 | 125746507 | Tima10 | C++17 (GCC 9-64) | 171 | 482.9 |
| C++ | 1 | 2021-08-13 | 125747642 | acm | C++17 (GCC 9-64) | 171 | 482.9 |
| C++ | 4 | 2025-07-08 | 328154273 | suuuiii | C++20 (GCC 13-64) | 202 | 188.5 |
| Java | 1 | 2022-12-10 | 184556532 | Alexander_Kozhevnikov | Java 17 | 421 | 270.9 |
| Java | 2 | 2023-07-25 | 215497856 | Stealty_Sorcerer, maksonchik20 | Java 17 | 452 | 272.6 |
| Kotlin | 1 | 2020-12-26 | 102405455 | LeoPro | Kotlin 1.4 | 670 | 507.3 |
| Kotlin | 2 | 2021-03-09 | 109431966 | blinovsv | Kotlin 1.4 | 686 | 378.2 |
| PyPy | 1 | 2021-02-18 | 107882415 | blinovsv | PyPy 3 | 967 | 195.0 |
| PyPy | — | | | no other author has a full-score PyPy submission | | | |

Notes:

- C++: the two 2021-08-13 submissions have the same time and nearly the same
  memory as 108495126, submitted six months earlier.
- Lowest memory per language: Rust 3.0 MB (399283895), C++ 81.0 MB (UncleSema,
  127512384, 2021-08-31, 967 ms), Kotlin 122.9 MB (LeoPro, 102416648, 2020-12-26,
  701 ms), PyPy 195.0 MB (107882415), Java 263.6 MB (klxwn_leg, 300419866,
  2025-01-10, 530 ms).

## Record progression: time

Each row is a full-score submission that beat every earlier one.

All languages:

| Date | Submission | Author | Compiler | Time, ms | Memory, MB |
|---|---|---|---|---|---|
| 2020-12-23 | 102137407 | Makcum888 | C++17 (GCC 7-32) | 452 | 406.6 |
| 2020-12-24 | 102189217 | withoutA | C++17 (GCC 7-32) | 436 | 384.2 |
| 2021-01-15 | 104430586 | JulianFernando | C++17 (GCC 7-32) | 389 | 287.5 |
| 2021-02-25 | 108495126 | blinovsv | C++17 (GCC 7-32) | 171 | 478.8 |
| 2026-11-21 | 399283895 | blinovsv | Rust 2024 | 93 | 3.0 |

Rust (all by `blinovsv`):

| Date | Submission | Compiler | Time, ms | Memory, MB |
|---|---|---|---|---|
| 2021-02-26 | 108532680 | Rust | 997 | 143.8 |
| 2021-02-27 | 108608540 | Rust | 374 | 143.8 |
| 2022-06-27 | 161961135 | Rust 2021 | 358 | 145.3 |
| 2026-05-08 | 373867536 | Rust 2024 | 218 | 104.3 |
| 2026-05-08 | 373867727 | Rust 2024 | 187 | 59.1 |
| 2026-11-21 | 399283895 | Rust 2024 | 93 | 3.0 |

Other languages:

- C++: same as the first four rows of the all-languages table; 171 ms has stood since 2021-02-25.
- Java: 421 ms (2022-12-10) was the first full-score Java submission and is still the best.
- Kotlin: 670 ms (2020-12-26) was the first and is still the best.
- PyPy: 967 ms (2021-02-18) is the only full-score submission.

## Record progression: memory

All languages:

| Date | Submission | Author | Compiler | Time, ms | Memory, MB |
|---|---|---|---|---|---|
| 2020-12-23 | 102137407 | Makcum888 | C++17 (GCC 7-32) | 452 | 406.6 |
| 2020-12-24 | 102189217 | withoutA | C++17 (GCC 7-32) | 436 | 384.2 |
| 2020-12-26 | 102416648 | LeoPro | Kotlin 1.4 | 701 | 122.9 |
| 2021-08-31 | 127512384 | UncleSema | C++17 (GCC 7-32) | 967 | 81.0 |
| 2026-05-08 | 373867727 | blinovsv | Rust 2024 | 187 | 59.1 |
| 2026-05-08 | 373867915 | blinovsv | Rust 2024 | 218 | 56.3 |
| 2026-11-21 | 399283895 | blinovsv | Rust 2024 | 93 | 3.0 |

Rust (all by `blinovsv`):

| Date | Submission | Compiler | Time, ms | Memory, MB |
|---|---|---|---|---|
| 2021-02-26 | 108532680 | Rust | 997 | 143.8 |
| 2026-05-08 | 373867536 | Rust 2024 | 218 | 104.3 |
| 2026-05-08 | 373867727 | Rust 2024 | 187 | 59.1 |
| 2026-05-08 | 373867915 | Rust 2024 | 218 | 56.3 |
| 2026-11-21 | 399283895 | Rust 2024 | 93 | 3.0 |

C++: 406.6 → 384.2 → 287.9 (vintage_Vlad_Makeev, 2020-12-27) → 287.5 → 81.0 MB.

## All full-score submissions by time

All 39 submissions with 100 points, fastest first (ties by submission id).

| # | Date | Submission | Author | Compiler | Time, ms | Memory, MB |
|---|---|---|---|---|---|---|
| 1 | 2026-11-21 | 399283895 | **blinovsv** | Rust 2024 | 93 | 3.0 |
| 2 | 2021-02-25 | 108495126 | **blinovsv** | C++17 (GCC 7-32) | 171 | 478.8 |
| 3 | 2021-08-13 | 125746507 | Tima10 | C++17 (GCC 9-64) | 171 | 482.9 |
| 4 | 2021-08-13 | 125747642 | acm | C++17 (GCC 9-64) | 171 | 482.9 |
| 5 | 2026-05-08 | 373867727 | **blinovsv** | Rust 2024 | 187 | 59.1 |
| 6 | 2025-07-08 | 328154273 | suuuiii | C++20 (GCC 13-64) | 202 | 188.5 |
| 7 | 2025-01-10 | 300420313 | klxwn_leg | C++23 (GCC 14-64, msys2) | 218 | 188.7 |
| 8 | 2026-05-08 | 373867536 | **blinovsv** | Rust 2024 | 218 | 104.3 |
| 9 | 2026-05-08 | 373867915 | **blinovsv** | Rust 2024 | 218 | 56.3 |
| 10 | 2022-06-27 | 161961135 | **blinovsv** | Rust 2021 | 358 | 145.3 |
| 11 | 2024-11-28 | 293643603 | Diall_ | C++20 (GCC 13-64) | 359 | 475.9 |
| 12 | 2022-06-27 | 161974857 | **blinovsv** | Rust 2021 | 373 | 145.3 |
| 13 | 2021-02-27 | 108608540 | **blinovsv** | Rust | 374 | 143.8 |
| 14 | 2025-08-12 | 333718676 | MisterGu | C++20 (GCC 13-64) | 374 | 262.0 |
| 15 | 2025-10-14 | 343621665 | FFTotoro | C++20 (GCC 13-64) | 374 | 296.0 |
| 16 | 2021-01-15 | 104430586 | JulianFernando | C++17 (GCC 7-32) | 389 | 287.5 |
| 17 | 2023-06-27 | 211289021 | Shefin_ | C++20 (GCC 11-64) | 405 | 480.4 |
| 18 | 2024-11-27 | 293485524 | Damirca | C++20 (GCC 13-64) | 405 | 475.7 |
| 19 | 2022-12-10 | 184556532 | Alexander_Kozhevnikov | Java 17 | 421 | 270.9 |
| 20 | 2024-08-09 | 275451150 | crowdforces | C++20 (GCC 13-64) | 421 | 287.3 |
| 21 | 2020-12-24 | 102189217 | withoutA | C++17 (GCC 7-32) | 436 | 384.2 |
| 22 | 2024-08-09 | 275440612 | Maram | C++20 (GCC 13-64) | 436 | 287.3 |
| 23 | 2021-01-10 | 103960843 | Kaurker | C++17 (GCC 7-32) | 451 | 407.0 |
| 24 | 2020-12-23 | 102137407 | Makcum888 | C++17 (GCC 7-32) | 452 | 406.6 |
| 25 | 2020-12-23 | 102141254 | alkr | C++17 (GCC 7-32) | 452 | 432.2 |
| 26 | 2023-07-25 | 215497856 | Stealty_Sorcerer, maksonchik20 | Java 17 | 452 | 272.6 |
| 27 | 2025-01-10 | 300419866 | klxwn_leg | Java 21 | 530 | 263.6 |
| 28 | 2020-12-25 | 102275736 | daniil-eshkin | C++17 (GCC 7-32) | 592 | 480.2 |
| 29 | 2024-07-19 | 271384971 | crowdforces | C++20 (GCC 13-64) | 593 | 380.3 |
| 30 | 2024-11-27 | 293514695 | Qutef | Java 21 | 593 | 265.3 |
| 31 | 2020-12-26 | 102405455 | LeoPro | Kotlin 1.4 | 670 | 507.3 |
| 32 | 2021-03-09 | 109431966 | **blinovsv** | Kotlin 1.4 | 686 | 378.2 |
| 33 | 2020-12-26 | 102416648 | LeoPro | Kotlin 1.4 | 701 | 122.9 |
| 34 | 2021-01-03 | 103124624 | DanielNash | C++17 (GCC 7-32) | 733 | 479.3 |
| 35 | 2020-12-27 | 102488872 | vintage_Vlad_Makeev | C++14 (GCC 6-32) | 764 | 287.9 |
| 36 | 2024-10-16 | 286163479 | Aldk | C++20 (GCC 13-64) | 952 | 384.1 |
| 37 | 2021-02-18 | 107882415 | **blinovsv** | PyPy 3 | 967 | 195.0 |
| 38 | 2021-08-31 | 127512384 | UncleSema | C++17 (GCC 7-32) | 967 | 81.0 |
| 39 | 2021-02-26 | 108532680 | **blinovsv** | Rust | 997 | 143.8 |

## Own submission history

All 36 submissions of `blinovsv` for problem F. Time of 1000 ms means the time
limit was hit on some test.

| Date | Submission | Compiler | Points | Time, ms | Memory, MB |
|---|---|---|---|---|---|
| 2021-01-04 | 103139872 | PyPy 3 | 11 | 140 | 2.7 |
| 2021-01-04 | 103185153 | PyPy 3 | 0 | 1000 | 4.4 |
| 2021-01-06 | 103531305 | PyPy 3 | 36 | 1000 | 9.1 |
| 2021-01-06 | 103533226 | PyPy 3 | 0 | 233 | 3.5 |
| 2021-01-06 | 103535062 | PyPy 3 | 0 | 202 | 3.2 |
| 2021-01-06 | 103536307 | PyPy 3 | 36 | 1000 | 138.5 |
| 2021-01-06 | 103537453 | PyPy 3 | 61 | 1000 | 107.1 |
| 2021-01-06 | 103567830 | Rust | 11 | 31 | 0.4 |
| 2021-01-06 | 103572110 | PyPy 3 | 21 | 1000 | 71.8 |
| 2021-01-06 | 103583383 | Rust | 0 | 31 | 0.5 |
| 2021-01-06 | 103583742 | Rust | 61 | 1000 | 109.0 |
| 2021-02-18 | 107738732 | PyPy 3 | 61 | 1000 | 124.1 |
| 2021-02-18 | 107849505 | PyPy 3 | 11 | 794 | 512.0 |
| 2021-02-18 | 107876156 | PyPy 3 | 11 | 1045 | 512.0 |
| 2021-02-18 | 107881138 | PyPy 3 | 71 | 1000 | 193.8 |
| 2021-02-18 | 107882415 | PyPy 3 | **100** | 967 | 195.0 |
| 2021-02-21 | 108096460 | Rust | 36 | 249 | 96.3 |
| 2021-02-21 | 108097424 | Rust | 71 | 1000 | 287.7 |
| 2021-02-25 | 108451274 | Rust | 71 | 1000 | 287.4 |
| 2021-02-25 | 108495126 | C++17 (GCC 7-32) | **100** | 171 | 478.8 |
| 2021-02-26 | 108530384 | Rust | 71 | 1000 | 143.8 |
| 2021-02-26 | 108532680 | Rust | **100** | 997 | 143.8 |
| 2021-02-27 | 108607692 | Rust | 11 | 93 | 143.7 |
| 2021-02-27 | 108607820 | Rust | 11 | 109 | 143.7 |
| 2021-02-27 | 108608540 | Rust | **100** | 374 | 143.8 |
| 2021-03-09 | 109431887 | Kotlin 1.4 | 21 | 358 | 96.7 |
| 2021-03-09 | 109431966 | Kotlin 1.4 | **100** | 686 | 378.2 |
| 2022-06-27 | 161961135 | Rust 2021 | **100** | 358 | 145.3 |
| 2022-06-27 | 161974857 | Rust 2021 | **100** | 373 | 145.3 |
| 2022-06-27 | 161975454 | Rust 2021 | 0 | 15 | 1.2 |
| 2026-04-23 | 372214744 | Rust 2024 | 0 | 78 | 380.1 |
| 2026-04-23 | 372214972 | Rust 2024 | 0 | 78 | 49.3 |
| 2026-05-08 | 373867536 | Rust 2024 | **100** | 218 | 104.3 |
| 2026-05-08 | 373867727 | Rust 2024 | **100** | 187 | 59.1 |
| 2026-05-08 | 373867915 | Rust 2024 | **100** | 218 | 56.3 |
| 2026-11-21 | 399283895 | Rust 2024 | **100** | 93 | 3.0 |

PyPy took 13 attempts over six weeks (2021-01-04 to 2021-02-18) to reach 100
points, and passed with 33 ms to spare.

Which repository file each accepted submission corresponds to is not recorded.
Memory footprints suggest (unverified): 108495126 is `solutions/f_cpp/f.cpp`,
108608540 and the 2022 pair are `solutions/f_dfs.rs`, 109431966 is
`solutions/f.kt`, and the May 2026 trio are iterations of `rust/f.rs`.
Confirmed: 399283895 is `rust/f_layers_static.rs`.

## Solutions in the repository

| File | Idea | Complexity |
|---|---|---|
| `solutions/f.rs` | BFS over `(i, j)` states in a `HashMap` | `O(n·m·k)` |
| `solutions/f_dfs.rs`, `solutions/f_cpp/f_dfs.cpp`, `solutions/f.kt` | memoised DFS over reachable `(i, j)` | `O(states·k)` |
| `solutions/f_cpp/f.cpp`, `solutions/f.py` | layers: `dp[len][i]` = furthest position in `B` | `O(ans·n·k)` |
| `rust/f.rs` | full `(i, j)` table, running minimum via buckets | `O(n·m)` |
| `rust/f_grid_parity.rs` | full `(i, j)` table, counter instead of buckets, 1 parity bit per cell | `O(n·m)` |
| `rust/f_layers_heap.rs` | layers with suffix maximum; back pointers as a corner bitmask in a `Vec` | `O(ans·(m + k))` |
| `rust/f_layers_static.rs` | same, bitmask in a `static mut` array | `O(ans·(m + k))` |

Two observations behind the new files:

- **Grid parity.** `dp[i][j]` is monotone and changes by at most 1 between
  neighbours, so all `k` transitions out of a cell take only the values `min`
  and `min + 1`. A counter replaces the buckets, and `dp mod 2` is enough for
  both the sweep and the backtrack.
- **Layers.** State `(i, j)` is dominated by any `(i' >= i, j' >= j)`, so a
  layer can be replaced by its suffix maximum `g`. For position `p` of `A` with
  value `x` and previous occurrence `q` of `x`: `dp[l+1][p] = nextB(g[q], x)`.
  The source of a transition is always a corner of the Pareto staircase, so one
  bit per `(layer, position)` stores the back pointers.

## Local benchmark

Local judge, 82 tests, one session on a Mac (ARM). C++ built with
`clang++ -O2`, Rust with `rustc -O`. Times are the worst in each group, in ms,
minus ~26 ms of process start-up. The machine was under load: absolute numbers
drifted by up to 1.5x between sessions, so compare columns, not sessions.

| Group (n, m ≈ 5000) | `f.cpp` | `f_dfs.cpp` | `solutions/f.rs` | `f_dfs.rs` | `rust/f.rs` | `f_grid_parity` | `f_layers_heap` | `f_layers_static` |
|---|---|---|---|---|---|---|---|---|
| k=1 | 19 | 19 | 5 | 4 | 3 | 53 | 52 | 56 |
| k=2 periodic, ans=2501 | 82 | 22 | 5 | 8 | 18 | 51 | 13 | 11 |
| k=2 random, ans≈1790 | 110 | 160 | 881 | 160 | 21 | 53 | 7 | 5 |
| k=3 | 109 | 171 | TL | 162 | 98 | 52 | 3 | 5 |
| k=5 | 66 | 173 | TL | 164 | 101 | 58 | 5 | 3 |
| k=10 | 59 | 160 | TL | 160 | 62 | 56 | 5 | 4 |
| k=20 | 58 | 160 | TL | 161 | 54 | 53 | 4 | 5 |
| k=200 | 57 | 106 | TL | 124 | 59 | 55 | 5 | 5 |
| k=2000, ans=1 | 57 | 179 | 4 | 226 | 5 | 56 | 4 | 4 |
| k=5000, ans=2 | 68 | 121 | 3 | 106 | 2 | 51 | 2 | 4 |
| **Sum over 82 tests** | 2240 | 4101 | 19740 | 4043 | 1316 | 1592 | 373 | 374 |
| **Accepted of 82** | 82 | 82 | 67 | 82 | 82 | 82 | 82 | 82 |

- The two layered variants are equal within noise; `static` was ahead by 0–2 ms
  in a separate 60-run measurement, visible only at k=1.
- k=1 is the weak spot of the layered variants (5001 layers of 5000 positions).
  A one-line `k == 1` special case removes it; it is not in the files.
- `solutions/f.rs` (BFS) fails 15 tests on time locally in this session; in an
  earlier, less loaded session it failed 6.
- Local peak memory: `f_layers_*` under 1 MB by the judge's polling (about 3 MB
  worst case by calculation), `f_grid_parity` 5 MB, `rust/f.rs` 50 MB,
  `f_dfs.rs` 146 MB, `f_dfs.cpp` 288 MB, `f.cpp` 345 MB.
