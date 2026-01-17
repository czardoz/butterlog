# Task 07 - Add Partition metadata and child index

Goal
- Store prefix length and line counts in partitions.
- Add a child index map for fast routing during incremental inserts.

Why
- Streaming inserts must route in O(1) per level instead of scanning children.

Data structures
- `Partition { prefix, prefix_len, line_indices, line_count, children, child_index, ... }`.

Files to touch
- src/partition.rs
- src/lib.rs
- tests/partition_struct.rs
- tests/partition_top_level.rs
- tests/partition_recursive.rs (if needed for signature changes)

Test-first steps (TDD)
1. Update `tests/partition_struct.rs` to assert:
   - `prefix_len` is stored on creation.
   - `line_count` equals `line_indices.len()`.
   - `child_index` starts empty.
2. Run `cargo test` and confirm failures due to new fields.

Implementation steps
3. Add new fields to `Partition`.
4. Update `Partition::new` to accept `prefix_len` and set `line_count` and `child_index`.
5. Add a helper like `fn rebuild_child_index(&mut self)` for later tasks.
6. Update constructors and helpers (`build_top_level_partitions`, tests, and any call sites) to pass `prefix_len`.
7. Run `cargo test` and ensure all tests pass.

Commit
8. `git add -A`
9. `git commit -m "Add partition metadata and child index"`

Exit criteria
- `cargo test` passes.
- Partitions carry prefix length, line counts, and an empty child index at creation.

Complexity notes
- `child_index` enables O(1) child lookup per level.
