# Task 21 - Render visible rows in TUI

Goal
- Render visible rows with indentation, arrows, and match highlighting.

Why
- The UI is the core experience: users scan partitions visually.

Data structures
- `VisibleRow` list from Task 18.

Files to touch
- Cargo.toml
- src/ui.rs (new)
- src/lib.rs
- tests/ui_render.rs

Test-first steps (TDD)
1. Add TUI dependencies (`ratatui` + `crossterm`) to `Cargo.toml`.
2. Write a failing test using `ratatui::backend::TestBackend`:
   - Render two rows: one collapsed, one expanded.
   - Assert the buffer contains arrow indicators (e.g., ">" for collapsed, "v" for expanded).
   - Assert matched rows use a highlight style (e.g., bold or color).
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `render_rows(rows: &[VisibleRow], frame: &mut Frame)` that:
   - Indents by `depth` (e.g., two spaces per level).
   - Prefixes each line with arrow and prefix text.
   - Applies a highlight style if `matches_self` or `matches_descendants` is true.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Render visible rows in TUI"`

Exit criteria
- `cargo test` passes.
- TUI render tests pass.
