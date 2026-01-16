# Task 22 - Key handling (normal mode)

Goal
- Handle Up/Down and Expand behavior in normal mode.

Why
- Users need to move through partitions and expand them using the keyboard.

Data structures
- `UiState { selected: usize }`
- `VisibleRow` list
- `RowPath` to toggle expansion

Files to touch
- src/ui_state.rs
- src/partition.rs
- tests/key_handling_normal.rs

Test-first steps (TDD)
1. Write failing tests for `handle_key_normal`:
   - Up arrow moves selection up (bounded at 0).
   - Down arrow expands the selected row if it is collapsed.
   - If already expanded, Down arrow moves selection down.
2. Run `cargo test` and confirm tests fail.

Implementation steps
3. Implement `handle_key_normal(key, rows, partitions, state)` using:
   - `toggle_expanded` from Task 20.
   - `UiState::move_up/move_down` from Task 19.
4. Decide and document the exact behavior when there are no children.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Handle normal-mode keys"`

Exit criteria
- `cargo test` passes.
- Normal key handling tests pass.
