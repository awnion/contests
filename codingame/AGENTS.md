# CodinGame

Solutions for [CodinGame](https://www.codingame.com) games and puzzles.

## Layout

Top level mirrors the CodinGame sections; inside a section there is one
directory per game, named after the slug in the game URL.

| Section | Directory | URL pattern |
| --- | --- | --- |
| Optimization | `optimization/` | `/multiplayer/optimization/<slug>` |
| Puzzles | `puzzles/` | `/training/<difficulty>/<slug>` |
| Code golf | `codegolf/` | `/multiplayer/codegolf/<slug>` |
| Bot programming | `bot-programming/` | `/multiplayer/bot-programming/<slug>` |

Section directories are created when the first game is added to them.

Every game is a mini-project with one self-contained subproject per language,
so it is possible to switch between implementations freely. A language
directory is created only when there is a solution in that language.

```
codingame/
├── AGENTS.md
├── pg.ipynb                      # scratch notebook, not tied to a game
└── <section>/
    └── <game-slug>/
        ├── README.md             # link, protocol summary, solutions table, notes
        ├── python/               # uv project
        │   ├── pyproject.toml    # pinned to the CodinGame versions below
        │   ├── .python-version
        │   ├── uv.lock
        │   └── <approach>.py     # one file = one submission
        ├── rust/                 # cargo package
        │   ├── Cargo.toml        # pinned to the CodinGame versions below
        │   ├── Cargo.lock
        │   ├── rust-toolchain.toml
        │   └── src/bin/<approach>.rs   # one file = one submission
        └── tools/                # optional: local simulator, test seeds, benchmarks
```

Example: `https://www.codingame.com/multiplayer/optimization/2048` ->
`optimization/2048/`.

## CodinGame environment

Source: [Languages Versions](https://www.codingame.com/playgrounds/40701/help-center/languages-versions)
(checked 2026-11-17; the page says its last update was 2024-09-25, but the Rust
row is clearly newer). Re-check before relying on it for a new language.

- Linux, 64-bit, multi-core.
- Memory limit: 768 MB.
- Code size limit: 100k characters.
- Time limit per turn: see the statement of each game.

| Language | Version | Libraries |
| --- | --- | --- |
| Python 3 | 3.11.5 | NumPy 1.23.2, SciPy 1.9.3, pandas (version unverified) |
| Rust | 1.90.0 | chrono 0.4.41, itertools 0.14.0, libc 0.2.175, rand 0.9.2, regex 1.11.2, time 0.3.43 |

Caveats:

- The official page lists NumPy 1.20.2, pandas 1.2.4, SciPy 1.6.3 for Python,
  which is stale: a
  [forum post from 2025-07-29](https://forum.codingame.com/t/python-numpy-scipy-versions/207086)
  shows the real runtime printing NumPy 1.23.2 and SciPy 1.9.3. Those are the
  versions pinned here. pandas 1.2.4 has no Python 3.11 build, so the real
  pandas version is unknown; print `pandas.__version__` in the CodinGame IDE
  before using it.
- NumPy is 1.x: NumPy 2-only APIs are not available.
- Not stated on the page and unverified: the Rust edition, the Rust
  optimization level and enabled crate features. Check in the CodinGame IDE
  and record the result here.
- Nothing else is installed: no other PyPI packages, no other crates.

## Python (uv)

Always use `uv`; never call `pip` or a system `python` directly.

`python/pyproject.toml`:

```toml
[project]
name = "cg-<game-slug>"
version = "0.0.0"
requires-python = "==3.11.*"
dependencies = [
    "numpy==1.23.2",
    "scipy==1.9.3",
]

[tool.uv]
package = false
```

`.python-version` contains `3.11`. Commit `uv.lock`; `.venv/` is ignored.

```sh
cd <section>/<game-slug>/python
uv sync
uv run <approach>.py < input.txt
```

Local-only tooling (pytest, ruff, ...) goes into a `dev` dependency group, never
into `dependencies`: a solution may import only what CodinGame provides.

## Rust (cargo)

`rust/rust-toolchain.toml` pins `channel = "1.90.0"`. `Cargo.toml` lists only
the crates CodinGame provides, pinned with `=`:

```toml
[dependencies]
chrono = "=0.4.41"
itertools = "=0.14.0"
libc = "=0.2.175"
rand = "=0.9.2"
regex = "=1.11.2"
time = "=0.3.43"
```

Add only the crates a solution actually uses. Each solution is a single file
in `src/bin/`, so it can be pasted into CodinGame as is:

```sh
cd <section>/<game-slug>/rust
cargo run --release --bin <approach> < input.txt
```

## Rules

- **One file per solution.** CodinGame accepts a single source file, so every
  solution must be submittable as is: no local imports or modules, reads stdin,
  writes answers to stdout, debug output to stderr.
- **Naming:** `<approach>.<ext>` in snake_case, e.g. `numpy_beam_search.py`,
  `bitboard_beam_search.rs`. Add a `_v2` style suffix when a variant of the
  same approach is worth keeping.
  - Puzzles usually have one obvious solution per language: `main.<ext>` is
    enough there.
  - Code golf: `golf_<chars>.<ext>` (e.g. `golf_87.py`), where `<chars>` is the
    character count CodinGame reports. Never reformat these files; the repo's
    `.editorconfig` may add a trailing newline, which does not count.
- **Old solutions are history.** Do not edit or "fix" an existing solution in
  place unless asked. For an improvement, add a new file and a new table row;
  record known bugs of old files in the README notes.
- **README per game** contains:
  - the game link,
  - a short protocol summary (input, output, limits, anything needed to
    simulate the game locally),
  - a solutions table: file, year, language, approach, result (score / rank /
    chars, `not recorded` if unknown),
  - notes: findings, known quirks, ideas.
- **Tools** (simulators, seed lists, benchmark scripts) go in `<game>/tools/`,
  never next to solutions. Build artifacts (`.venv/`, `target/`) are not
  committed.
- Rules in a game README that come from memory or from reading a solution,
  not from the statement, must be marked as unverified.

## Local results

A score measured on this machine is not the score on the site and must not be
expected to match it: the hardware is different, so a time-limited search
gets a different amount of work done per turn (and may time out in one place
but not in the other). Only a solution whose moves do not depend on the clock
gives the same score in both places.

- Use local runs to compare solutions with each other, on the same machine
  and the same seeds.
- In a README, always label a local number as local (`local judge: ...`);
  the "Result" column is for what the site reported.
- Local judges live in `<game>/tools/`. Run timing-sensitive solutions one
  game at a time, parallel games slow each other down.


1. Pick the section from the game URL and create `<section>/<game-slug>/README.md`.
2. Create the language subproject (`python/` or `rust/`) as described above.
3. Put the solution in it following the naming rule.
4. Add a row to the README solutions table.
5. Add the game to the list below.

## Games

### Optimization

| Game | Directory |
| --- | --- |
| [2048](https://www.codingame.com/multiplayer/optimization/2048) | [optimization/2048/](optimization/2048/) |
| [Search Race](https://www.codingame.com/multiplayer/optimization/search-race) | [optimization/search-race/](optimization/search-race/) |
