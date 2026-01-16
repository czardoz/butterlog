# Task 08 - Estimate total line count

Goal
- Estimate total line count using file size and average line length.

Why
- Used to choose a target partition size.

Data structures
- None beyond function inputs/outputs.

Files to touch
- src/estimate.rs (new)
- src/lib.rs
- tests/estimate_lines.rs

Test-first steps (TDD)
1. Write failing tests for `estimate_total_lines(file_size_bytes, avg_line_len)` with small numeric examples.
2. Include a test that locks in the rounding rule (floor or ceil).
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement the estimation function and document the rounding rule in code.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Estimate total line count"`

Exit criteria
- `cargo test` passes.
- Estimation tests pass.
