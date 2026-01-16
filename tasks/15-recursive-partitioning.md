# Task 15 - Recursive partition splitting

Goal
- Split large partitions into children using longer prefixes.

Why
- Large partitions need shorter prefixes at the top, but more detail when expanded.

Data structures
- `Partition` tree with `children: Vec<Partition>`.
- `Vec<usize>` line indices to avoid copying strings.

Files to touch
- src/partition.rs
- tests/partition_recursive.rs

Test-first steps (TDD)
1. Build a sample set of lines where one prefix group is larger than the target size.
2. Write a failing test for `split_partition` or `split_partitions` that asserts:
   - A large partition gains children.
   - Children prefixes are one character longer (or whatever increment you choose).
   - Children line indices partition the parent line indices without loss.
3. Run `cargo test` and confirm the test fails.

Implementation steps
4. Implement `split_partition(partition, lines, prefix_len, target_size)`:
   - If `partition.line_count() <= target_size`, stop.
   - Otherwise group that partition's lines by `prefix_len + 1` and create children.
   - Recurse on each child with the longer prefix length.
5. Add a guard to stop recursion when all lines are shorter than the next prefix length (i.e., no further split is possible).
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add recursive partition splitting"`

Exit criteria
- `cargo test` passes.
- Recursive partition tests pass.
