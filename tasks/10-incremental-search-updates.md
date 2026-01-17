# Task 10 - Update search matches for newly loaded lines

Goal
- When a search term is active, update match flags immediately for newly loaded lines.

Why
- The UI should reflect matches as soon as new data arrives.

Data structures
- Optional `term: &SearchTerm` parameter in insertion helpers.

Files to touch
- src/search.rs
- src/partition.rs
- src/ui_state.rs or src/app.rs (call sites)
- tests/partition_search.rs
- tests/search_incremental.rs (new)

Test-first steps (TDD)
1. Create `tests/search_incremental.rs`:
   - Build a small partition tree and a `SearchTerm`.
   - Insert a new line that matches the term.
   - Assert `matches_self` or `matches_descendants` is set on the correct partitions.
2. Run `cargo test` and confirm failures.

Implementation steps
3. Update insertion helpers to accept `Option<&SearchTerm>` and:
   - Check the new line only (not a full scan).
   - Set `matches_self` on the partition that owns the line.
   - Bubble `matches_descendants` to parents along the path.
4. Keep the full-scan path for new search terms (existing `mark_search_matches`).
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Update search matches incrementally"`

Exit criteria
- `cargo test` passes.
- New lines update search flags immediately without a full re-scan.

Complexity notes
- Incremental updates are O(depth) per new line.
