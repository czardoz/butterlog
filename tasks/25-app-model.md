# Task 25 - App model struct

Goal
- Define a top-level `AppModel` that holds the data needed by the UI loop.

Why
- Centralized state makes the event loop easier to reason about for new engineers.

Data structures
- `struct AppModel {
    line_store: LineStore,
    partitions: Vec<Partition>,
    rows: Vec<VisibleRow>,
    ui: UiState,
    search: SearchState,
  }`

Files to touch
- src/app.rs (new)
- src/lib.rs
- tests/app_model.rs

Test-first steps (TDD)
1. Write a failing unit test that constructs an `AppModel` with minimal data.
2. Assert `rows` and `ui.selected` initialize to sensible defaults.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `AppModel::new(line_store, partitions)` that:
   - Builds visible rows.
   - Initializes `UiState` and `SearchState`.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Add AppModel struct"`

Exit criteria
- `cargo test` passes.
- App model tests pass.
