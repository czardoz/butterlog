# Task 06 - Extend LineStore for incremental appends

Goal
- Add a fast append path for newly loaded lines.
- Track per-line length for quick width computations.

Why
- Streaming needs a way to add lines without rebuilding the store.

Data structures
- `LineStore { lines, normalized, line_lengths }`.

Files to touch
- src/line_store.rs
- src/lib.rs
- tests/line_store.rs

Test-first steps (TDD)
1. Extend `tests/line_store.rs`:
   - Build a store with 1 line.
   - Call `append_lines` with 2 more lines.
   - Assert `lines`, `normalized`, and `line_lengths` include all 3 lines and correct lengths.
2. Run `cargo test` and confirm the new test fails.

Implementation steps
3. Add `line_lengths: Vec<usize>` to `LineStore`.
4. Update `LineStore::new` to populate `line_lengths`.
5. Add `append_lines(&mut self, new_lines: Vec<String>) -> std::ops::Range<usize>` (or similar) that:
   - Appends to `lines` and `normalized`.
   - Appends lengths to `line_lengths`.
   - Returns the index range for the newly appended lines.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add LineStore append support"`

Exit criteria
- `cargo test` passes.
- Appending lines updates normalized text and lengths correctly.

Complexity notes
- Append is O(new_lines).
