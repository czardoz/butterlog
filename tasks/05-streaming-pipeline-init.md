# Task 05 - Initialize streaming pipeline from LineLoader

Goal
- Replace `read_first_n_lines` with `LineLoader` + `LoadState`.
- Return the initial sample, partitions, load state, and partition plan from a single build function.

Why
- Streaming requires keeping the file handle open and returning state for future loads.

Data structures
- `BuildOutput { line_store, partitions, load_state, plan }` (or similar).

Files to touch
- src/pipeline.rs
- src/app.rs
- src/main.rs
- src/lib.rs
- tests/pipeline.rs
- tests/cli_pipeline.rs (if needed for return signature changes)

Test-first steps (TDD)
1. Update `tests/pipeline.rs` to expect the new return type:
   - Assert the initial sample size is the batch size (or smaller on EOF).
   - Assert the `PartitionPlan` is present.
   - Assert `load_state.is_complete` is false when more data exists.
2. Run `cargo test` and confirm failures due to signature mismatches.

Implementation steps
3. Add a `BuildOutput` struct in `pipeline.rs`.
4. Change `build_partitions_from_file` to:
   - Open a `LineLoader`.
   - Read the first batch with `LoadState`.
   - Build `LineStore` and initial partitions from that batch.
   - Compute `PartitionPlan` from the sample.
   - Return `BuildOutput`.
5. Update `build_partitions_from_file_default` and `--no-ui` code paths.
6. Update `AppModel::new` and callers to use the new output.
7. Run `cargo test` and ensure all tests pass.

Commit
8. `git add -A`
9. `git commit -m "Initialize streaming pipeline with loader"`

Exit criteria
- `cargo test` passes.
- The pipeline returns a loader-backed state and a stable partition plan.

Complexity notes
- Initial build is still O(sample_size).
