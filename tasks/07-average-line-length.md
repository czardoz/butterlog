# Task 07 - Compute average line length

Goal
- Compute the average length of sampled lines.

Why
- Average length is used to estimate total line count.

Data structures
- Reuse `LineSample` with `lines: Vec<String>`.

Files to touch
- src/line_sample.rs
- tests/line_sample_avg.rs

Test-first steps (TDD)
1. Write a failing unit test that constructs `LineSample` with known lines.
2. Assert `average_line_len(&sample)` returns the expected value (choose integer rounding rule and test it).
3. Run `cargo test` and confirm the test fails.

Implementation steps
4. Implement `average_line_len(sample: &LineSample) -> usize` (or `f64` if you prefer; test should specify).
5. Document the rounding rule in code comments and keep it stable for tests.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Compute average line length"`

Exit criteria
- `cargo test` passes.
- Average line length test passes.
