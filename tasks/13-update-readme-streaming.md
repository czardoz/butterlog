# Task 13 - Update README for streaming behavior

Goal
- Document streaming loading behavior and the partial-load indicator.

Why
- Users should know that data loads progressively and search may be incomplete until fully loaded.

Files to touch
- README.md

Test-first steps (TDD)
1. Add or update a README test if one exists (skip if none).
2. If no tests exist for README, note in the task that this is a documentation-only change.

Implementation steps
3. Update the README to include:
   - Streaming load behavior (loads more lines as you scroll near the end).
   - The partial-load indicator and its meaning.
   - Clarify that search results may be incomplete until fully loaded.
4. Run `cargo test` (optional for docs-only change, but keep the habit).

Commit
5. `git add README.md`
6. `git commit -m "Document streaming load behavior"`

Exit criteria
- README reflects streaming behavior and the partial-load indicator.

Complexity notes
- Docs only.
