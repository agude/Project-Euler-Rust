# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rust solutions to Project Euler problems. Each problem is a standalone binary in `src/bin/` named by problem number (e.g., `001.rs`, `045.rs`).

## Commands

```bash
# Build all solutions
cargo build

# Run a specific problem
cargo run --bin 001

# Run all tests
cargo test

# Run tests for a specific problem
cargo test --bin 001

# Lint
cargo clippy

# Format
cargo fmt
```

## Architecture

- **`src/bin/*.rs`** - Individual problem solutions. Each contains:
  - Problem description in a comment
  - `euler_XXX()` function with the solution logic
  - `main()` that prints the answer
  - Tests verifying example cases and final answer

- **`src/utils/`** - Shared mathematical utilities:
  - `primes.rs` - Prime sieve and iterators
  - `fibonacci.rs` - Fibonacci sequence generator
  - `factorization.rs` - Integer factorization
  - `combinatorics.rs` - Combinations, permutations
  - `palindromic.rs` - Palindrome checking
  - `polygonal.rs` - Triangular, pentagonal, hexagonal numbers

Import utils via: `use euler_rust::utils::primes;`
