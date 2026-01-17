# Task 12 - Reduce per-frame O(n) work

Goal
- Avoid rebuilding rows and recomputing max width on every idle frame.

Why
- Streaming data and large row sets make per-frame O(n) rebuilds expensive.

Data structures
- `rows_dirty: bool` and `cached_max_width: usize` in `AppModel`.

Files to touch
- src/app.rs
- src/main.rs
- src/ui.rs (if width logic changes)
- tests/app_model.rs or new tests/perf_flags.rs

Test-first steps (TDD)
1. Add a test that constructs `AppModel` and verifies:
   - `rows_dirty` is true on creation.
   - After calling a new `mark_rows_clean()` (or similar), it becomes false.
2. Run `cargo test` and confirm failures.

Implementation steps
3. Add `rows_dirty` and `cached_max_width` fields to `AppModel`.
4. Add helpers like `refresh_rows_if_dirty()` and `recompute_max_width()`.
5. In the main loop:
   - Only rebuild rows when `rows_dirty` is true (input, search changes, load).
   - Cache the max width after rebuild and use it for horizontal scroll clamping.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Reduce per-frame row rebuilds"`

Exit criteria
- `cargo test` passes.
- Row rebuilds happen only when data or selection changes.

Complexity notes
- Avoids O(rows) work on idle frames.
