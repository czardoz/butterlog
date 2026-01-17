# Task 02 - Add a streaming LineLoader that reads batches

Goal
- Introduce a `LineLoader` that reads log lines in fixed-size batches from a file.
- Return owned data so this logic can later move to a background thread.

Why
- Streaming input is required to load more data as the user scrolls.

Data structures
- `LineLoader` (owns `BufReader<File>` and an `is_complete` flag).
- `LoadBatch` containing `Vec<String>` and `is_complete`.

Files to touch
- src/loader.rs (new)
- src/lib.rs
- tests/loader.rs (new)

Test-first steps (TDD)
1. Create `tests/loader.rs` with a test that:
   - Writes a temp file with 3 lines.
   - Calls `load_next(2)` and asserts two lines returned and `is_complete == false`.
   - Calls `load_next(2)` and asserts one line returned and `is_complete == true`.
2. Run `cargo test` and confirm the new test fails.

Implementation steps
3. Implement `LineLoader::open(path)` to open the file and wrap it in `BufReader`.
4. Implement `LineLoader::load_next(batch_size)` using `read_line` in a loop:
   - Trim trailing newlines.
   - Stop when `batch_size` is reached or EOF.
   - Set `is_complete` on EOF.
5. Return a `LoadBatch` with owned `Vec<String>` and `is_complete`.
6. Export `LineLoader` and `LoadBatch` in `src/lib.rs`.
7. Run `cargo test` and ensure all tests pass.

Commit
8. `git add -A`
9. `git commit -m "Add line loader for batch reads"`

Exit criteria
- `cargo test` passes.
- Batch reads preserve order and set `is_complete` at EOF.

Complexity notes
- Each batch read is O(batch_size).
