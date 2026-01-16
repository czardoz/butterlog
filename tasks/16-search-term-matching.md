# Task 16 - Search term normalization and line matching

Goal
- Normalize a search term and match it against normalized lines.

Why
- We need consistent, case-insensitive search behavior.

Data structures
- `struct SearchTerm { raw: String, normalized: String }`

Files to touch
- src/search.rs (new)
- src/lib.rs
- tests/search_term.rs

Test-first steps (TDD)
1. Write failing tests that:
   - Build a `SearchTerm` from mixed-case input.
   - Assert `normalized` is lowercase.
   - Assert `matches_line` returns true for a line containing the term (case-insensitive).
2. Run `cargo test` and confirm tests fail.

Implementation steps
3. Implement `SearchTerm::new(raw: &str) -> Self` and `matches_line(&self, normalized_line: &str) -> bool` using `contains`.
4. Keep matching simple and documented (substring, not regex).
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Add search term normalization"`

Exit criteria
- `cargo test` passes.
- Search term tests pass.
