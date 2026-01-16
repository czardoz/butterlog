# Task 28 - CLI integration (non-interactive mode for tests)

Goal
- Add a non-interactive mode to run the pipeline and exit successfully.

Why
- Integration tests should verify CLI wiring without hanging in the TUI.

Data structures
- Reuse `AppModel` and pipeline output.

Files to touch
- src/main.rs
- tests/cli_pipeline.rs

Test-first steps (TDD)
1. Write a failing integration test that:
   - Creates a temp log file.
   - Runs `butterlog <file> --no-ui`.
   - Asserts exit code 0 and output contains a summary like "partitions:".
2. Run `cargo test` and confirm the test fails.

Implementation steps
3. Add a `--no-ui` flag to the CLI parser.
4. In `--no-ui` mode, run the pipeline and print a short summary (e.g., number of partitions, estimated lines).
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Add --no-ui CLI integration mode"`

Exit criteria
- `cargo test` passes.
- CLI pipeline integration test passes.
