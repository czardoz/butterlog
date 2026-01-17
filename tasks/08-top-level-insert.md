# Task 08 - Insert new lines into top-level partitions

Goal
- Add an O(1) top-level insert path that preserves first-seen order.

Why
- Streaming requires appending lines into existing partitions or creating new ones as new prefixes appear.

Data structures
- `PartitionIndex { top_index: HashMap<String, usize> }` to map prefix -> partition index.

Files to touch
- src/partition.rs (or new src/partition_index.rs)
- src/lib.rs
- tests/partition_insert_top.rs (new)

Test-first steps (TDD)
1. Create `tests/partition_insert_top.rs`:
   - Start with two partitions (prefixes A and B) and a `PartitionIndex`.
   - Insert a new line with prefix A; assert partition A grows.
   - Insert a new line with prefix C; assert a new partition is appended at the end.
2. Run `cargo test` and confirm the new test fails.

Implementation steps
3. Implement `PartitionIndex::from_partitions(&[Partition])` to build the map.
4. Implement `insert_top_level(...)` that:
   - Computes the prefix using the fixed `top_prefix_len`.
   - Uses the index map to find an existing partition, or appends a new one and updates the map.
   - Updates `line_indices` and `line_count`.
5. Export the helper in `src/lib.rs` if needed for tests.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Insert lines into top-level partitions"`

Exit criteria
- `cargo test` passes.
- Top-level insert preserves order and uses O(1) prefix lookup.

Complexity notes
- Insert is O(1) for map lookup plus O(1) for append.
