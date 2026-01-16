# Task 18 - Flatten partitions into visible rows

Goal
- Convert the partition tree into a flat list for rendering and navigation.

Why
- UI rendering and selection are easier with a linear list of visible rows.

Data structures
- `struct RowPath(Vec<usize>)` (path of child indices from root).
- `struct VisibleRow {
    path: RowPath,
    depth: usize,
    prefix: String,
    line_count: usize,
    expanded: bool,
    matches_self: bool,
    matches_descendants: bool,
  }`

Files to touch
- src/view.rs (new)
- src/lib.rs
- tests/visible_rows.rs

Test-first steps (TDD)
1. Create a small partition tree with one expanded node.
2. Write a failing test for `flatten_partitions` that asserts:
   - Only expanded children are included.
   - Depth and path values are correct.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `flatten_partitions(partitions: &[Partition]) -> Vec<VisibleRow>` using recursion.
5. Clone only the data needed for rendering (prefix, counts, flags).
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Flatten partitions into visible rows"`

Exit criteria
- `cargo test` passes.
- Visible row tests pass.
