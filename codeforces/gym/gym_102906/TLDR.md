2021... Last problem (Russian Olympiad in Informatics 2020—2021, St. Petersburg)...
Typical DP hard problem which for some reason became my playground for benchmark game:

**1st** and straightforward: symmetric DP over all `(i, j)` pairs: relatively easy 90 lines in `C++` for experienced contestants with a preallocated stack

**2nd** faster solution is DP by answer length:

- swap the answer with a state dimension: for each length `l` and position `i` in `A` keep the furthest reachable position in `B`
- _No one assumes you can write it online._ But it's cool to theorycraft around here 😎
- Transition is not hard to spot: `best[l + 1][nextA(i, x)] = max(nextB(best[l][i], x))`, but it's quite hard (annoying) to implement bug-free in an online contest. Usually not worth the risk.

> [!WARNING]
> **DISCLAIMER!** On Codeforces `Rust` can't use a 100 MB static stack, unlike `C++`.
> So it's not an apples-to-apples comparison. All numbers are just for illustration. [→ Honest bench](#honest-bench)

| | | | Time | RAM |
|---|---|---|---:|---:|
| 2021&nbsp;Feb | **First full score** on `PyPy`: barely squeezed into **1000 ms**!<br>• but had to use the “annoying smart alternative solution” (aka `Layered DP`)<br>• and I'm still the only one who could squeeze a Python-like solution for the problem 🤷 | PyPy&nbsp;3 | 967&nbsp;ms | 195.0&nbsp;MB |
| 2021&nbsp;Feb | Layered DP in `C++` → **fastest** solution overall for the next **5.7 years** | C++17 | 171&nbsp;ms | 478.8&nbsp;MB |
| 2021&nbsp;Feb | Memoised DFS in `Rust`: tables indexed `[value][position]` → **wrong order** 🙁 | Rust&nbsp;2018 | 997&nbsp;ms | 143.8&nbsp;MB |
| 2021&nbsp;Feb | Same code: just tables indexed `[position][value]` → **2.7x faster!!!** | Rust&nbsp;2018 | 374&nbsp;ms | 143.8&nbsp;MB |
| 2021&nbsp;Mar | Memoised DFS in `Kotlin`<br>• `Kotlin` unlike `Rust` isn't that sensitive to the index order<br>• first attempt **good enough** | Kotlin&nbsp;1.4 | 686&nbsp;ms | 378.2&nbsp;MB |
| 2022&nbsp;Jun | After more than a year I'm trying to improve… | Rust&nbsp;2021 | 358&nbsp;ms | 145.3&nbsp;MB |
| 2026&nbsp;May | Years passed… **GPT 5.5 Released!**<br>Revisited the solution with `GPT 5.5`: **almost beat** `C++` in heap-based `Rust` and **8x less memory** than `C++`!!! | Rust&nbsp;2024 | 187&nbsp;ms | 59.1&nbsp;MB |
| 2026&nbsp;Nov | **Fable 5.2 Released!**<br>WTF?? **IS THIS WITCHCRAFT**. Ok Ok. In hindsight it's not that hard:<br>• a layer solution from `PyPy`<br>• a back pointer always targets a **“corner”** of the layer<br>&nbsp;&nbsp;&nbsp;&nbsp;◦ `1 bit` per cell instead of `u16` → **160x less memory** than `C++`<br>• the bit table is a fixed-size `static` array<br>&nbsp;&nbsp;&nbsp;&nbsp;◦ no heap growth<br>&nbsp;&nbsp;&nbsp;&nbsp;◦ no stack limit | Rust&nbsp;2024 | 93&nbsp;ms&nbsp;(!) | 🏆&nbsp;**3.0&nbsp;MB** |
| 2026&nbsp;Nov | Line-by-line clone of the `Rust` code in `C++`<br>• **fastest** in any language, **1.2x faster** than `Rust`: same algorithm, so it's the compiler and runtime (or noise: one run each)<br>• 4x more RAM than `Rust`: every table is a static array sized for the worst case | C++23 | 🏆&nbsp;**78&nbsp;ms** | 11.8&nbsp;MB |

<a name="honest-bench"></a>

## Honest bench

**We use the latest `C++` and `Rust` compilers and count only the progression of the solutions:**

<sub>every accepted `C++` / `Rust` submission, downloaded from Codeforces and rebuilt on one machine (M2 Max, `clang 21 -std=c++23`, `rustc 1.102`); worst test time and peak memory over 82 tests</sub>

| | | | Time | RAM |
|---|---|---|---:|---:|
| 2021&nbsp;Feb | Layered DP: five full `5005 × 5005` `int` tables | C++ | 72.9&nbsp;ms | 345.9&nbsp;MB |
| 2021&nbsp;Feb | Memoised DFS, tables indexed `[value][position]` (**wrong order** 🙁) | Rust | 341.8&nbsp;ms | 145.9&nbsp;MB |
| 2021&nbsp;Feb | Same code: tables indexed `[position][value]` | Rust | 169.4&nbsp;ms | 146.0&nbsp;MB |
| 2022&nbsp;Jun | Same code resubmitted (second attempt: only reformatted) | Rust | 171.4&nbsp;ms | 146.0&nbsp;MB |
| 2026&nbsp;May | Symmetric table with linked-list buckets: `u16` `dp` table + `u16` choice table | Rust | 91.7&nbsp;ms | 98.5&nbsp;MB |
| 2026&nbsp;May | Choice table dropped: only the `u16` `dp` table, as a `static` | Rust | 79.7&nbsp;ms | 50.7&nbsp;MB |
| 2026&nbsp;May | Plus special cases for `k = 1`, `k = 2`, `k ≤ 3` | Rust | 64.6&nbsp;ms | 50.8&nbsp;MB |
| 2026&nbsp;Nov | Layered DP: suffix maximum per layer, `1 bit` back pointers in a `static` array | Rust | 34.1&nbsp;ms | 5.2&nbsp;MB |
| 2026&nbsp;Nov | Line-by-line clone of the `Rust` code | C++ | 31.2&nbsp;ms | 🏆&nbsp;**4.9&nbsp;MB** |
| 2026&nbsp;Nov | Not submitted, local only: one in-place pass per layer, `NEON` suffix maximum, 8 cells at a time | Rust | 14.8&nbsp;ms | 5.2&nbsp;MB |
| 2026&nbsp;Nov | Not submitted, local only: twin of the `Rust` `NEON` code | C++ | 🏆&nbsp;**14.5&nbsp;ms** | 5.0&nbsp;MB |
