# Task 11 - Group lines by prefix (order-preserving)

Goal
- Group sampled lines by prefix while preserving the first-seen order of prefixes.

Why
- Stable ordering makes UI navigation predictable for users.

Data structures
- `struct Group { prefix: String, line_indices: Vec<usize> }`
- `HashMap<String, usize>` to map prefix -> index in a `Vec<Group>`.

Files to touch
- src/grouping.rs (new)
- src/lib.rs
- tests/grouping.rs

Test-first steps (TDD)
1. Write failing tests for `group_by_prefix(lines, prefix_len)` that assert:
   - Prefix order is the order of first appearance in the input.
   - Each group contains the correct line indices.
2. Include tests for empty lines and mixed prefixes.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `group_by_prefix` using a `HashMap` to find existing groups and a `Vec<Group>` to preserve order.
5. Keep line indices (usize) instead of cloning full lines to reduce memory.
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Group lines by prefix"`

Exit criteria
- `cargo test` passes.
- Grouping tests pass.
