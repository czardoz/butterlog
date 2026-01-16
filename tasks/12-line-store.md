# Task 12 - LineStore for original and normalized lines

Goal
- Store both original and normalized (lowercased) lines for fast search.

Why
- Search should be case-insensitive and avoid repeated normalization work.

Data structures
- `struct LineStore { lines: Vec<String>, normalized: Vec<String> }`

Files to touch
- src/line_store.rs (new)
- src/lib.rs
- tests/line_store.rs

Test-first steps (TDD)
1. Write a failing test that builds a `LineStore` from lines with mixed case.
2. Assert `lines` keeps original content and `normalized` is lowercased.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `LineStore::new(lines: Vec<String>) -> Self`.
5. Add a helper `fn get(&self, idx: usize) -> (&str, &str)` to fetch original and normalized views.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add LineStore for normalized search"`

Exit criteria
- `cargo test` passes.
- LineStore tests pass.
