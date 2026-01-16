# Task 14 - Build top-level partitions

Goal
- Convert prefix groups into top-level `Partition` nodes.

Why
- This is the entry point for the partition tree displayed in the UI.

Data structures
- `Vec<Group>` from Task 11 mapped into `Vec<Partition>`.

Files to touch
- src/partition.rs
- tests/partition_top_level.rs

Test-first steps (TDD)
1. Create a small set of lines (e.g., "ERR: a", "ERR: b", "INFO: c").
2. Write a failing test that groups them and builds top-level partitions.
3. Assert the result has two partitions, each with the correct prefix and line indices.
4. Run `cargo test` and confirm the test fails.

Implementation steps
5. Implement `build_top_level_partitions(groups: Vec<Group>, depth: usize) -> Vec<Partition>`.
6. Ensure partition order matches group order.
7. Run `cargo test` and ensure all tests pass.

Commit
8. `git add -A`
9. `git commit -m "Build top-level partitions"`

Exit criteria
- `cargo test` passes.
- Top-level partition tests pass.
