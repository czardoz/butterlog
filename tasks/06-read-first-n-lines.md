# Task 06 - Read the first N lines

Goal
- Read the first N lines of a file into memory.

Why
- We only sample the first 5000 lines (or fewer) to estimate line length and build initial partitions.

Data structures
- `struct LineSample { lines: Vec<String> }`.

Files to touch
- src/line_sample.rs (new)
- src/lib.rs
- tests/line_sample_read.rs

Test-first steps (TDD)
1. Create a temp file with multiple lines.
2. Write a failing test for `read_first_n_lines(path, n)` that asserts:
   - Only the first N lines are returned.
   - Lines preserve order and exclude trailing `\n`.
3. Run `cargo test` and confirm the test fails.

Implementation steps
4. Implement `read_first_n_lines(path: &Path, n: usize) -> AppResult<LineSample>` using `BufRead::lines()`.
5. Strip trailing `\n` (the iterator already removes it), and keep line order.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Read first N lines"`

Exit criteria
- `cargo test` passes.
- Line sampling test passes.
