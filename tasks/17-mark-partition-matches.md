# Task 17 - Mark search matches on partitions

Goal
- Flag partitions that contain the search term and bubble match info to parents.

Why
- The UI needs to highlight partitions that match or contain matching children.

Data structures
- `Partition { matches_self, matches_descendants }`.
- `LineStore` for normalized lines.
- `SearchTerm` for normalized search term.

Files to touch
- src/search.rs
- src/partition.rs
- tests/partition_search.rs

Test-first steps (TDD)
1. Build a small partition tree with line indices.
2. Write a failing test that applies a search term and asserts:
   - A partition with matching lines has `matches_self = true`.
   - A parent of a matching child has `matches_descendants = true`.
3. Run `cargo test` and confirm tests fail.

Implementation steps
4. Implement `mark_search_matches(partitions, line_store, term)` using recursion:
   - Compute `matches_self` by scanning the partition's line indices.
   - Recurse into children and set `matches_descendants` if any child matches.
5. Ensure flags are reset to false before recomputing (so repeated searches work).
6. Run `cargo test` and ensure all tests pass.

Commit
7. `git add -A`
8. `git commit -m "Mark search matches on partitions"`

Exit criteria
- `cargo test` passes.
- Partition search tests pass.
