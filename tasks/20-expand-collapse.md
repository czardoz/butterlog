# Task 20 - Expand/collapse partitions by path

Goal
- Toggle a partition's `expanded` state using a row path.

Why
- The UI needs to expand the selected row to reveal children.

Data structures
- `RowPath(Vec<usize>)` from Task 18.
- `Partition { expanded: bool }`.

Files to touch
- src/partition.rs
- src/view.rs
- tests/expand_collapse.rs

Test-first steps (TDD)
1. Build a small partition tree and a `RowPath` pointing at a child.
2. Write a failing test for `toggle_expanded` that flips `expanded` from false to true.
3. Add a second test to toggle back to false.
4. Run `cargo test` and confirm tests fail.

Implementation steps
5. Implement `toggle_expanded(partitions: &mut [Partition], path: &RowPath)` by walking indices.
6. If the path is invalid, return an error or do nothing (decide and test it).
7. Run `cargo test` and ensure all tests pass.

Commit
8. `git add -A`
9. `git commit -m "Toggle partition expansion by path"`

Exit criteria
- `cargo test` passes.
- Expand/collapse tests pass.
