# judge

Local contest judge for this problem. It runs a solution command against all
tests in `test_data`, applies time and memory limits, then checks the produced
output with the built-in checker.

## Basic Usage

Run from the `judge` directory:

```sh
cargo run --release -- -s 'pypy3 ../solutions/f.py'
```

The solution command is passed to `bash -lc`, so shell commands, arguments, and
relative paths work as usual.

## Main Options

- `-s, --solution <COMMAND>`: solution command to run.
- `--test-data <DIR>`: directory with tests, default is `test_data`.
- `--time-limit <SECONDS>`: per-test time limit, default is `1.0`.
- `--memory-limit-mb <MB>`: per-test memory limit, default is `512`.
- `--stdin-file <PATH>`: run the solution once on a single input file without
  checking against answers.

## Examples

Run the Python solution on all tests:

```sh
cargo run --release -- -s 'pypy3 ../solutions/f.py'
```

Run a compiled C++ solution:

```sh
cargo run --release -- -s '../solutions/f_cpp/f'
```

Use custom limits:

```sh
cargo run --release -- -s 'pypy3 ../solutions/f.py' --time-limit 2.0 --memory-limit-mb 1024
```

Use another test directory:

```sh
cargo run --release -- -s 'pypy3 ../solutions/f.py' --test-data test_data
```

Run once on a single input file:

```sh
cargo run --release -- -s 'pypy3 ../solutions/f.py' --stdin-file ../solutions/f.in
```

## Verdicts

- `OK`: accepted by the checker.
- `WA`: wrong answer.
- `TL`: time limit exceeded.
- `ML`: memory limit exceeded.
- `RE`: runtime error.
- `FL`: checker or judge failure.

