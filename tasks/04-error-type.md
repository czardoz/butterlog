# Task 04 - Define a stable error type

Goal
- Introduce a dedicated error enum for CLI and pipeline errors with stable messages.

Why
- Centralized error messages make tests reliable and user output consistent.

Data structures
- `enum AppError` with variants like `MissingArg`, `PathNotFound(PathBuf)`, `PathNotFile(PathBuf)`, `Io(io::Error)`.

Files to touch
- src/error.rs (new)
- src/lib.rs (export `AppError`)
- tests/error_display.rs

Test-first steps (TDD)
1. Write failing tests that construct `AppError` variants and assert their `Display` strings match the expected stable messages.
2. Run `cargo test` and confirm tests fail.

Implementation steps
3. Implement `AppError` (use `thiserror` if desired) and `impl Display` with the exact strings used in tests.
4. Add a `type AppResult<T> = Result<T, AppError>` alias if helpful.
5. Run `cargo test` and ensure all tests pass.

Commit
6. `git add -A`
7. `git commit -m "Add AppError with stable messages"`

Exit criteria
- `cargo test` passes.
- Error display tests pass.
