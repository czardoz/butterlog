# Task 01 - Initialize crate and CLI help

Goal
- Create the Rust binary crate and ensure `butterlog --help` works.

Why
- A runnable binary is needed so later tests can exercise the CLI.

Data structures
- None yet.

Files to touch
- Cargo.toml
- src/main.rs
- tests/cli_help.rs

Test-first steps (TDD)
1. Add `assert_cmd` and `predicates` as dev-dependencies in `Cargo.toml`.
2. Write a failing integration test in `tests/cli_help.rs`:
   - Run `butterlog --help`.
   - Assert the output contains "butterlog" and a usage header ("USAGE" or "Usage").
3. Run `cargo test` and confirm the new test fails.

Implementation steps
4. If the crate does not exist, run `cargo init --bin`.
5. Implement a minimal CLI (use `clap` or basic `std::env` parsing) that supports `--help`.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Initialize crate and CLI help"`

Exit criteria
- `cargo test` passes.
- The `butterlog --help` test passes.
