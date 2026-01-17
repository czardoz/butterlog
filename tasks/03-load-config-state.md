# Task 03 - Add LoadConfig and LoadState for streaming control

Goal
- Centralize streaming parameters and state, with a clear API that can later be moved to a background thread.

Why
- The UI should not know file IO details; it should ask for batches based on a simple state object.

Data structures
- `LoadConfig { batch_size: usize, near_end_threshold: usize }` with defaults (5000 and 20).
- `LoadState { loader: LineLoader, config: LoadConfig, is_complete: bool }`.

Files to touch
- src/load_state.rs (new)
- src/lib.rs
- tests/load_state.rs (new)

Test-first steps (TDD)
1. Create `tests/load_state.rs` that:
   - Asserts `LoadConfig::default()` uses batch_size 5000 and near_end_threshold 20.
   - Builds a `LoadState` with a small temp file and calls `load_more()` until EOF.
   - Asserts `is_complete` flips to true when EOF is reached.
2. Run `cargo test` and confirm the new tests fail.

Implementation steps
3. Implement `LoadConfig` with `Default`.
4. Implement `LoadState::new(loader, config)` and `load_more()`:
   - Call `loader.load_next(config.batch_size)`.
   - Update `is_complete` from the batch result.
   - Return the `LoadBatch` (owned data).
5. Export `LoadConfig` and `LoadState` in `src/lib.rs`.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add load config and state"`

Exit criteria
- `cargo test` passes.
- LoadState reports completion correctly and uses default batch/threshold values.

Complexity notes
- `load_more()` is O(batch_size).
