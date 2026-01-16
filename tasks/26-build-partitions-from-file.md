# Task 26 - Build partitions from a file (pipeline)

Goal
- Wire together file size, sampling, estimation, heuristics, and partitioning.

Why
- This is the core non-UI pipeline described in the README.

Data structures
- `LineSample`, `LineStore`, `Partition`, and heuristic outputs.

Files to touch
- src/pipeline.rs (new)
- src/lib.rs
- tests/pipeline.rs

Test-first steps (TDD)
1. Create a temp file with a handful of lines.
2. Write a failing test for `build_partitions_from_file(path)` that asserts:
   - It returns a non-empty `Vec<Partition>`.
   - The `LineStore` contains the sampled lines.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `build_partitions_from_file(path) -> AppResult<(LineStore, Vec<Partition>)>`:
   - Read first 5000 lines into `LineSample`.
   - Compute average line length.
   - Get file size and estimate total lines.
   - Compute `target_partition_size` and `initial_prefix_len` via heuristics.
   - Group by prefix and build top-level partitions.
   - Recursively split partitions with `split_partition`.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Wire file-to-partition pipeline"`

Exit criteria
- `cargo test` passes.
- Pipeline test passes.
