# Task 27 - Event loop and quit handling

Goal
- Add an event loop that renders repeatedly and exits on 'q'.

Why
- The tool is interactive, and must exit cleanly.

Data structures
- `AppModel` plus `UiState { should_quit: bool }` (or similar flag).

Files to touch
- src/ui_state.rs
- src/ui.rs
- src/main.rs
- tests/quit_handling.rs

Test-first steps (TDD)
1. Write a failing unit test for `handle_key_normal` (or a new `handle_key`) that sends a 'q' key event.
2. Assert `should_quit` becomes true.
3. Run `cargo test` and confirm the test fails.

Implementation steps
4. Add `should_quit` flag to UI state.
5. Implement the event loop in `main`:
   - Initialize terminal.
   - Loop: render UI, read events, update app state.
   - Break when `should_quit` is true.
   - Restore terminal on exit (use RAII or a guard).
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add event loop and quit handling"`

Exit criteria
- `cargo test` passes.
- Quit handling test passes.
