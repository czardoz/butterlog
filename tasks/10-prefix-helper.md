# Task 10 - Prefix extraction helper

Goal
- Extract a prefix of a given length from a line safely.

Why
- Grouping and partitioning depend on consistent prefix extraction.

Data structures
- `struct PrefixKey { text: String, len: usize }` (optional helper to keep prefix metadata).

Files to touch
- src/prefix.rs (new)
- src/lib.rs
- tests/prefix.rs

Test-first steps (TDD)
1. Write failing tests for `prefix_of(line, len)`:
   - Line longer than len returns the first `len` characters.
   - Line shorter than len returns the full line.
   - Empty line returns empty string.
2. Include a test for a line with UTF-8 characters and decide your behavior (byte-based vs char-based) and lock it in.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `prefix_of(line: &str, len: usize) -> String` using the behavior defined in tests.
5. If using UTF-8 safe slicing, use `char_indices` or `graphemes`; keep it simple for logs.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Add prefix extraction helper"`

Exit criteria
- `cargo test` passes.
- Prefix helper tests pass.
