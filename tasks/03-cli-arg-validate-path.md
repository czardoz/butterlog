# Task 03 - Validate log file path

Goal
- Validate that the provided path exists and is a file.

Why
- We need predictable error handling before touching the file.

Data structures
- None yet.

Files to touch
- src/main.rs
- tests/cli_args_invalid_path.rs

Test-first steps (TDD)
1. Write a failing test that runs `butterlog /no/such/file.log`.
2. Assert non-zero exit and a stable error message like "log file not found".
3. Write a second failing test for a directory path (create a temp dir) and assert error "path is not a file".
4. Run `cargo test` and confirm both tests fail.

Implementation steps
5. Add path validation using `std::fs::metadata`.
6. Map OS errors to the stable error messages expected by tests.
7. Run `cargo test` and ensure all tests pass.

Commit
8. `git add -A`
9. `git commit -m "Validate log file path"`

Exit criteria
- `cargo test` passes.
- Invalid-path and directory-path CLI tests pass.
