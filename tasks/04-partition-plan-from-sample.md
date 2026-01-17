# Task 04 - Compute a PartitionPlan from the initial sample

Goal
- Capture the initial partitioning strategy (top-level prefix length and target size) and keep it stable as more lines load.

Why
- The user explicitly wants to keep the prefix length chosen from the initial sample.

Data structures
- `PartitionPlan { top_prefix_len: usize, target_size: usize }`.

Files to touch
- src/pipeline.rs (or new src/partition_plan.rs)
- src/lib.rs
- tests/partition_plan.rs (new)

Test-first steps (TDD)
1. Create `tests/partition_plan.rs`:
   - Use a small set of lines with distinct prefixes.
   - Provide a fake `estimated_lines` and `screen_height`.
   - Assert `top_prefix_len` matches the expected prefix length chosen by `choose_prefix_len`.
   - Assert `target_size` matches `target_partition_size_for_screen` logic.
2. Run `cargo test` and confirm the new test fails.

Implementation steps
3. Add `PartitionPlan` and a constructor like `PartitionPlan::from_sample(lines, estimated_lines, screen_height)`.
4. Ensure it uses existing logic (`choose_prefix_len`, `target_partition_size_for_screen`).
5. Export `PartitionPlan` in `src/lib.rs`.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add partition plan from initial sample"`

Exit criteria
- `cargo test` passes.
- PartitionPlan stores stable `top_prefix_len` and `target_size`.

Complexity notes
- `choose_prefix_len` is O(n * max_prefix_len); this task does not change that.
