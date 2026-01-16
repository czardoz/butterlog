# Task 23 - Search input mode and key handling

Goal
- Add a search input mode with a text buffer and apply search on Enter.

Why
- Users must be able to search and highlight partitions containing a term.

Data structures
- `enum InputMode { Normal, Search }`
- `struct SearchState { mode: InputMode, buffer: String, term: Option<SearchTerm> }`

Files to touch
- src/ui_state.rs
- src/search.rs
- tests/search_input_mode.rs

Test-first steps (TDD)
1. Write failing tests that simulate key sequences:
   - `/` enters search mode.
   - Typing characters appends to the buffer.
   - `Backspace` removes a character.
   - `Enter` converts the buffer into `SearchTerm` and returns to normal mode.
   - `Esc` clears the buffer and returns to normal mode without changing term.
2. Run `cargo test` and confirm tests fail.

Implementation steps
3. Implement search-mode key handling in `ui_state`.
4. Ensure `SearchTerm` is created only when buffer is non-empty.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Add search input mode"`

Exit criteria
- `cargo test` passes.
- Search input mode tests pass.
