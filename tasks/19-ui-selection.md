# Task 19 - UI selection state

Goal
- Track and update the selected row index.

Why
- Navigation (up/down) needs deterministic selection behavior.

Data structures
- `struct UiState { selected: usize }`

Files to touch
- src/ui_state.rs (new)
- src/lib.rs
- tests/ui_selection.rs

Test-first steps (TDD)
1. Write failing tests for `UiState::move_up` and `UiState::move_down`:
   - Selection stays at 0 when moving up at the top.
   - Selection stays at last index when moving down at the bottom.
2. Run `cargo test` and confirm tests fail.

Implementation steps
3. Implement `UiState` with `move_up(max)` and `move_down(max)` methods.
4. Keep logic simple and bounds-checked.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Add UI selection state"`

Exit criteria
- `cargo test` passes.
- UI selection tests pass.
