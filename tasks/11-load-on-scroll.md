# Task 11 - Load more data when selection nears the end

Goal
- Trigger `load_more` when the selection is within 20 rows of the end.
- Append new lines into `LineStore` and partitions, then refresh rows.

Why
- This is the streaming behavior the user requested.

Data structures
- `LoadConfig.near_end_threshold` (20).
- Helper `should_load_more(selected, total_rows, threshold)`.

Files to touch
- src/main.rs
- src/app.rs or src/ui_state.rs (where AppModel lives)
- src/load_state.rs
- tests/load_trigger.rs (new)

Test-first steps (TDD)
1. Create `tests/load_trigger.rs` with a pure helper test:
   - `should_load_more(80, 100, 20)` returns true.
   - `should_load_more(10, 100, 20)` returns false.
2. Run `cargo test` and confirm failures.

Implementation steps
3. Add `should_load_more` to `load_state.rs` (or a small helper module).
4. In the main loop:
   - After handling input, check `should_load_more` and `load_state.has_more()`.
   - Call `load_state.load_more()` to get a `LoadBatch`.
   - Append lines to `LineStore`, and insert each new line into partitions.
   - Update `load_status.is_complete` based on `LoadBatch.is_complete`.
   - Refresh rows and re-run `ensure_visible`.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Load more lines when nearing the end"`

Exit criteria
- `cargo test` passes.
- Scrolling near the end loads more data and updates the UI.

Complexity notes
- Per load: O(batch_size * depth) for inserts.
- Row rebuild is still O(rows) after each load.
