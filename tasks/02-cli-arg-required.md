# Task 02 - Require a log file argument

Goal
- Make the CLI require a single log file path argument.

Why
- The tool operates on a log file, so we need a strict, testable contract.

Data structures
- None yet.

Files to touch
- src/main.rs
- tests/cli_args_missing.rs

Test-first steps (TDD)
1. Write a failing integration test that runs `butterlog` with no args.
2. Assert it exits non-zero and prints a stable error message such as "missing log file argument".
3. Run `cargo test` and confirm the test fails.

Implementation steps
4. Update argument parsing to require one positional argument: the log file path.
5. On missing arg, print the stable error message and return a non-zero exit code.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Require log file argument"`

Exit criteria
- `cargo test` passes.
- Missing-argument CLI test passes.
