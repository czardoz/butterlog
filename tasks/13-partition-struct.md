# Task 13 - Define the Partition data structure

Goal
- Define the core `Partition` struct used in the tree.

Why
- Partitioning and UI will operate on a shared, explicit data model.

Data structures
- `struct Partition {
    prefix: String,
    line_indices: Vec<usize>,
    children: Vec<Partition>,
    depth: usize,
    expanded: bool,
    matches_self: bool,
    matches_descendants: bool,
  }`

Files to touch
- src/partition.rs (new)
- src/lib.rs
- tests/partition_struct.rs

Test-first steps (TDD)
1. Write a failing unit test that constructs a `Partition` with known values.
2. Assert default flags (`expanded`, `matches_*`) are false and children is empty when created by `Partition::new`.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `Partition::new(prefix, line_indices, depth)` that sets defaults.
5. Add helper methods like `line_count()` to avoid repeated `len()` calls.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add Partition struct"`

Exit criteria
- `cargo test` passes.
- Partition struct tests pass.
