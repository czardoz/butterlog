# Task 01 - Add partial-load indicator in the UI

Goal
- Show a clear status line that tells the user when the file is only partially loaded.
- The status line must also warn that search results may be incomplete until the file is fully loaded.

Why
- Users need immediate feedback that more data exists and search is not exhaustive yet.

Data structures
- New `LoadStatus` (or similar) struct that tracks whether loading is complete.
- Status line formatting that combines search input and load status.

Files to touch
- src/ui.rs
- src/ui_state.rs or a new src/load_status.rs
- src/main.rs
- src/app.rs (if AppModel stores load status)
- src/lib.rs (exports)
- tests/ui_render.rs

Test-first steps (TDD)
1. Add a new test in `tests/ui_render.rs`:
   - Build a minimal `rows` list with one visible row.
   - Create a `LoadStatus` (or similar) with `is_complete = false`.
   - Render the UI and assert the last line contains both "Partial load" and "search incomplete".
2. Run `cargo test` and confirm the new test fails.

Implementation steps
3. Introduce a `LoadStatus` struct with an `is_complete: bool` field (default false for now).
4. Update `render_rows` to accept `LoadStatus` (alongside `SearchState`), and render a status line:
   - If search mode is active, show `Search: <buffer>` and append a short partial-load warning.
   - If search mode is inactive and `is_complete` is false, show a partial-load warning by itself.
5. Wire `LoadStatus` into `AppModel` (or `UiState`) and pass it from `main` into `render_rows`.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Show partial-load status in UI"`

Exit criteria
- `cargo test` passes.
- The status line clearly indicates partial load and incomplete search when `is_complete` is false.

Complexity notes
- Rendering is still O(rows) per draw; this task does not change that.
