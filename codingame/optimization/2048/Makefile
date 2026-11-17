# Local judge runner, see README.md ("Local judge").
#
#   make rust   BIN=beam_search                 # judge rust/src/bin/<BIN>.rs
#   make python SOL=numpy_beam_search           # judge python/<SOL>.py
#   make rust   BIN=beam_search ARGS="-s 290797 --stderr"
#   make test                                   # judge self-tests
#   make help                                   # judge options (for ARGS)

BIN  ?= beam_search
SOL  ?= numpy_beam_search
ARGS ?=

JUDGE := cargo run -q --release --manifest-path tools/judge/Cargo.toml --
# cargo may be configured with a shared target directory, so ask it.
RUST_TARGET = $(shell cargo metadata --format-version 1 --no-deps --manifest-path rust/Cargo.toml \
	| sed 's/.*"target_directory":"\([^"]*\)".*/\1/')

.PHONY: rust python test help

rust:
	cargo build -q --release --manifest-path rust/Cargo.toml --bin $(BIN)
	$(JUDGE) $(ARGS) $(RUST_TARGET)/release/$(BIN)

# One game at a time by default: Python solutions use their time budget, and
# parallel games slow each other down. Override with ARGS="-j 4".
python:
	cd python && uv sync -q
	$(JUDGE) -j 1 $(ARGS) python/.venv/bin/python python/$(SOL).py

test:
	cargo test --release --manifest-path tools/judge/Cargo.toml

help:
	@$(JUDGE) --help
