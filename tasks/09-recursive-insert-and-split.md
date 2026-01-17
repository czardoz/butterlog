# Task 09 - Recursive insert and split for growing partitions

Goal
- Insert lines into the correct child partition using the stored prefix lengths.
- Split a leaf partition when it exceeds `target_size`.

Why
- As new lines arrive, partitions must expand without rebuilding everything.

Data structures
- `Partition::insert_line(...)` that updates counts, indices, and children.

Files to touch
- src/partition.rs
- src/pipeline.rs (if split helpers are moved or reused)
- tests/partition_recursive.rs
- tests/partition_split_incremental.rs (new)

Test-first steps (TDD)
1. Add a new test `tests/partition_split_incremental.rs`:
   - Build a partition with a small `target_size`.
   - Insert enough lines to trigger a split.
   - Assert children are created and `child_index` is populated.
2. Add/extend a test in `tests/partition_recursive.rs` to ensure insert routes into an existing child by prefix.
3. Run `cargo test` and confirm failures.

Implementation steps
4. Add `Partition::insert_line(line_idx, line_text, target_size, term_opt)`:
   - Increment `line_count` and push `line_idx` to `line_indices`.
   - If children exist, route using `child_index` and the child `prefix_len`.
   - If children are absent and `line_count > target_size`, split the partition.
5. Update `split_partition` to:
   - Use `partition.prefix_len` as the starting point.
   - Assign `prefix_len` on children.
   - Rebuild `child_index` after splitting.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Insert lines recursively and split partitions"`

Exit criteria
- `cargo test` passes.
- Inserting lines routes to the correct child and splits when oversized.

Complexity notes
- Insert is O(depth) with O(1) child lookup per level.
- Splitting is O(k) for k line indices in the partition being split.
