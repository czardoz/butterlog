# Task 05 - File size utility

Goal
- Add a utility that returns file size in bytes.

Why
- File size is needed to estimate total line count.

Data structures
- None beyond function output types.

Files to touch
- src/file_size.rs (new)
- src/lib.rs
- tests/file_size.rs

Test-first steps (TDD)
1. Create a temp file with known content length.
2. Write a failing test that calls `file_size_bytes(path)` and asserts the size matches the known length.
3. Run `cargo test` and confirm the test fails.

Implementation steps
4. Implement `file_size_bytes(path: &Path) -> AppResult<u64>` using `std::fs::metadata`.
5. Convert I/O errors into `AppError::Io`.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add file size utility"`

Exit criteria
- `cargo test` passes.
- File size test passes.
