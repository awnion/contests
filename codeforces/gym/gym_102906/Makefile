SHELL := /bin/bash

BUILD_DIR := build
SOLUTIONS_DIR := solutions
JUDGE_DIR := judge
TEST_DATA := $(JUDGE_DIR)/test_data

RUSTC ?= rustc
CXX ?= clang++
KOTLINC ?= kotlinc
KOTLINC_NATIVE ?= kotlinc-native
PYPY ?= pypy3
JAVA ?= java
CARGO ?= cargo

RUST_FLAGS := -C opt-level=3 -C target-cpu=native -C lto=fat -C codegen-units=1
CXX_FLAGS := -std=c++17 -O3 -march=native -flto -pipe
KOTLIN_NATIVE_FLAGS := -opt
JAVA_RUN_FLAGS := -server -Xbatch

JUDGE := $(BUILD_DIR)/judge
JUDGE_CARGO_TARGET := $(abspath $(BUILD_DIR)/cargo-target)
JUDGE_CARGO_BIN := $(JUDGE_CARGO_TARGET)/release/judge

CPP_MAIN := $(BUILD_DIR)/f-cpp
CPP_DFS := $(BUILD_DIR)/f-dfs-cpp
RUST_MAIN := $(BUILD_DIR)/f-rs
RUST_DFS := $(BUILD_DIR)/f-dfs-rs
KOTLIN_JAR := $(BUILD_DIR)/f-kotlin.jar
KOTLIN_NATIVE_SRC := $(BUILD_DIR)/f-kotlin-native.kt
KOTLIN_NATIVE_BIN := $(BUILD_DIR)/f-kotlin-native.kexe

# Extra variants: rust/<name>.rs -> build/rs-<name>, cpp/<name>.cpp -> build/cpp-<name>
RUST_EXTRA := $(basename $(notdir $(wildcard rust/*.rs)))
CPP_EXTRA := $(basename $(notdir $(wildcard cpp/*.cpp)))
RUST_EXTRA_BINS := $(addprefix $(BUILD_DIR)/rs-,$(RUST_EXTRA))
CPP_EXTRA_BINS := $(addprefix $(BUILD_DIR)/cpp-,$(CPP_EXTRA))
EXTRA_JUDGE := $(addprefix judge-rs-,$(RUST_EXTRA)) $(addprefix judge-cpp-,$(CPP_EXTRA))

.PHONY: help all build-all build-judge build-cpp build-rust build-kotlin build-native \
	judge-cpp judge-cpp-dfs judge-rs judge-rs-dfs judge-kotlin \
	judge-kotlin-native judge-py judge-py-dfs judge-py-bfs judge-all clean \
	build-extra judge-extra $(EXTRA_JUDGE)

help:
	@echo "Build targets:"
	@echo "  make all                   build judge and compiled solutions"
	@echo "  make build-all             build judge and compiled solutions"
	@echo "  make build-judge           build judge into $(JUDGE)"
	@echo "  make build-cpp             build C++ solutions"
	@echo "  make build-rust            build Rust solutions"
	@echo "  make build-kotlin          build Kotlin/JVM jar"
	@echo "  make build-native          build Kotlin/Native binary"
	@echo
	@echo "Judge targets:"
	@echo "  make judge-cpp             run $(CPP_MAIN)"
	@echo "  make judge-cpp-dfs         run $(CPP_DFS)"
	@echo "  make judge-rs              run $(RUST_MAIN)"
	@echo "  make judge-rs-dfs          run $(RUST_DFS)"
	@echo "  make judge-kotlin          run $(KOTLIN_JAR) with java"
	@echo "  make judge-kotlin-native   run $(KOTLIN_NATIVE_BIN)"
	@echo "  make judge-py              run $(SOLUTIONS_DIR)/f.py with pypy3"
	@echo "  make judge-py-dfs          run $(SOLUTIONS_DIR)/f_dfs.py with pypy3"
	@echo "  make judge-py-bfs          run $(SOLUTIONS_DIR)/f_bfs.py with pypy3"
	@echo "  make judge-all             run all judge targets above"
	@echo
	@echo "Extra variants from rust/ and cpp/:"
	@echo "  make build-extra           build all of them"
	@echo "  make judge-extra           run all of them"
	@for t in $(EXTRA_JUDGE); do echo "  make $$t"; done
	@echo
	@echo "Override tools if needed, for example:"
	@echo "  make CXX=g++ judge-cpp"

all: build-all

build-all: build-judge build-cpp build-rust build-kotlin build-native

build-judge: $(JUDGE)
build-cpp: $(CPP_MAIN) $(CPP_DFS)
build-rust: $(RUST_MAIN) $(RUST_DFS)
build-kotlin: $(KOTLIN_JAR)
build-native: $(KOTLIN_NATIVE_BIN)
build-extra: $(RUST_EXTRA_BINS) $(CPP_EXTRA_BINS)

$(BUILD_DIR)/.dir:
	mkdir -p $(BUILD_DIR)
	touch $@

$(JUDGE): $(shell find $(JUDGE_DIR)/src -type f 2>/dev/null) $(JUDGE_DIR)/Cargo.toml $(JUDGE_DIR)/Cargo.lock | $(BUILD_DIR)/.dir
	cd $(JUDGE_DIR) && CARGO_TARGET_DIR=$(JUDGE_CARGO_TARGET) $(CARGO) build --release
	install -m 755 $(JUDGE_CARGO_BIN) $(JUDGE)

$(CPP_MAIN): $(SOLUTIONS_DIR)/f_cpp/f.cpp | $(BUILD_DIR)/.dir
	$(CXX) $(CXX_FLAGS) $< -o $@

$(CPP_DFS): $(SOLUTIONS_DIR)/f_cpp/f_dfs.cpp | $(BUILD_DIR)/.dir
	$(CXX) $(CXX_FLAGS) $< -o $@

$(RUST_MAIN): $(SOLUTIONS_DIR)/f.rs | $(BUILD_DIR)/.dir
	$(RUSTC) $(RUST_FLAGS) $< -o $@

$(RUST_DFS): $(SOLUTIONS_DIR)/f_dfs.rs | $(BUILD_DIR)/.dir
	$(RUSTC) $(RUST_FLAGS) $< -o $@

$(KOTLIN_JAR): $(SOLUTIONS_DIR)/f.kt | $(BUILD_DIR)/.dir
	$(KOTLINC) $< -include-runtime -d $@

$(KOTLIN_NATIVE_SRC): $(SOLUTIONS_DIR)/f.kt | $(BUILD_DIR)/.dir
	perl -pe 's/readLine\(\)!!/readln()/g; s/readLine\(\)/readln()/g' $< > $@

$(KOTLIN_NATIVE_BIN): $(KOTLIN_NATIVE_SRC) | $(BUILD_DIR)/.dir
	$(KOTLINC_NATIVE) $< $(KOTLIN_NATIVE_FLAGS) -o $(BUILD_DIR)/f-kotlin-native

# Local-only variants may use flags a contest judge would not.
$(BUILD_DIR)/rs-%_do_not_submit: RUST_FLAGS += -C panic=abort

$(BUILD_DIR)/rs-%: rust/%.rs | $(BUILD_DIR)/.dir
	$(RUSTC) $(RUST_FLAGS) $< -o $@

$(BUILD_DIR)/cpp-%: cpp/%.cpp | $(BUILD_DIR)/.dir
	$(CXX) $(CXX_FLAGS) $< -o $@

$(addprefix judge-rs-,$(RUST_EXTRA)): judge-rs-%: $(JUDGE) $(BUILD_DIR)/rs-%
	$(JUDGE) --test-data $(TEST_DATA) -s './$(BUILD_DIR)/rs-$*'

$(addprefix judge-cpp-,$(CPP_EXTRA)): judge-cpp-%: $(JUDGE) $(BUILD_DIR)/cpp-%
	$(JUDGE) --test-data $(TEST_DATA) -s './$(BUILD_DIR)/cpp-$*'

judge-extra: $(EXTRA_JUDGE)

judge-cpp: $(JUDGE) $(CPP_MAIN)
	$(JUDGE) --test-data $(TEST_DATA) -s './$(CPP_MAIN)'

judge-cpp-dfs: $(JUDGE) $(CPP_DFS)
	$(JUDGE) --test-data $(TEST_DATA) -s './$(CPP_DFS)'

judge-rs: $(JUDGE) $(RUST_MAIN)
	$(JUDGE) --test-data $(TEST_DATA) -s './$(RUST_MAIN)'

judge-rs-dfs: $(JUDGE) $(RUST_DFS)
	$(JUDGE) --test-data $(TEST_DATA) -s './$(RUST_DFS)'

judge-kotlin: $(JUDGE) $(KOTLIN_JAR)
	$(JUDGE) --test-data $(TEST_DATA) -s '$(JAVA) $(JAVA_RUN_FLAGS) -jar ./$(KOTLIN_JAR)'

judge-kotlin-native: $(JUDGE) $(KOTLIN_NATIVE_BIN)
	$(JUDGE) --test-data $(TEST_DATA) -s './$(KOTLIN_NATIVE_BIN)'

judge-py: $(JUDGE)
	$(JUDGE) --test-data $(TEST_DATA) -s '$(PYPY) $(SOLUTIONS_DIR)/f.py'

judge-py-dfs: $(JUDGE)
	$(JUDGE) --test-data $(TEST_DATA) -s '$(PYPY) $(SOLUTIONS_DIR)/f_dfs.py'

judge-py-bfs: $(JUDGE)
	$(JUDGE) --test-data $(TEST_DATA) -s '$(PYPY) $(SOLUTIONS_DIR)/f_bfs.py'

judge-all: judge-cpp judge-cpp-dfs judge-rs judge-rs-dfs judge-kotlin judge-kotlin-native judge-py judge-py-dfs judge-py-bfs

clean:
	rm -rf $(BUILD_DIR)
