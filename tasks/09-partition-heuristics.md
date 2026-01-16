# Task 09 - Partition sizing heuristics

Goal
- Convert the estimated line count into a target partition size and an initial prefix length.

Why
- The README says large partitions use shorter prefixes; we need a clear, testable rule.

Data structures
- None beyond function inputs/outputs.

Files to touch
- src/heuristics.rs (new)
- src/lib.rs
- tests/heuristics.rs

Test-first steps (TDD)
1. Write failing tests for `target_partition_size(estimated_lines)` using a few thresholds, e.g.:
   - 100 lines => target 10
   - 10_000 lines => target 500
   (Pick the thresholds you want and lock them in with tests.)
2. Write failing tests for `initial_prefix_len(target_size)` such as:
   - target >= 10_000 => prefix 1
   - target >= 1_000 => prefix 2
   - target >= 100 => prefix 3
   - else => prefix 4
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement both functions with the exact thresholds used in tests.
5. Add brief comments explaining why large targets get shorter prefixes.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add partition sizing heuristics"`

Exit criteria
- `cargo test` passes.
- Heuristic tests pass.
