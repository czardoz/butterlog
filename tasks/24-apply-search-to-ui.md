# Task 24 - Apply search to UI state

Goal
- Wire search term changes to partition match flags and rendering data.

Why
- Search should immediately highlight matching partitions in the UI.

Data structures
- `SearchState` and `SearchTerm`.
- `Partition` match flags.
- `VisibleRow` list.

Files to touch
- src/ui_state.rs
- src/search.rs
- src/view.rs
- tests/ui_search_apply.rs

Test-first steps (TDD)
1. Create a small partition tree and `LineStore` with known content.
2. Write a failing test that sets a search term and asserts:
   - `mark_search_matches` is applied.
   - `VisibleRow` items are updated with match flags.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Add a `apply_search(term, partitions, line_store)` method in UI or app state.
5. Rebuild visible rows after applying the search so highlighting matches.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Apply search to UI state"`

Exit criteria
- `cargo test` passes.
- UI search application tests pass.
