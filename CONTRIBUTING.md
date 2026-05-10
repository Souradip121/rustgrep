# Contributing to rustgrep

Thanks for taking the time to contribute! This is a learning project, so all levels of experience are welcome — whether you're fixing a typo or adding a new feature.

## Ways to contribute

- Report a bug by opening an issue
- Suggest a feature via a feature request issue
- Fix a bug or implement a feature and open a pull request
- Improve documentation or examples

## Getting started

1. **Fork** the repository and clone your fork:
   ```bash
   git clone https://github.com/your-username/rustgrep.git
   cd rustgrep
   ```

2. **Build** to confirm everything compiles:
   ```bash
   cargo build
   ```

3. **Run tests** to confirm the baseline is green:
   ```bash
   cargo test
   ```

4. **Create a branch** for your change:
   ```bash
   git checkout -b fix/my-bug-fix
   # or
   git checkout -b feat/my-new-feature
   ```

## Making changes

- Follow the existing code style — run `cargo fmt` before committing.
- Resolve all Clippy warnings: `cargo clippy -- -D warnings`
- Write tests for new behaviour in a `#[cfg(test)]` module in the same file.
- Integration tests go in the `tests/` directory.
- Run `cargo test` before every commit — do not commit failing tests.

## Commit messages

Write in the imperative mood, under 72 characters:

```
Add case-insensitive flag to walker
Fix off-by-one in line number output
Refactor build_matcher to reduce duplication
```

Add a body only when the *why* is non-obvious.

## Submitting a pull request

1. Push your branch to your fork.
2. Open a pull request against the `master` branch of this repo.
3. Fill in the pull request template — describe what changed and how to test it.
4. A maintainer will review and leave feedback. Address comments and push updated commits (do not force-push).

## What to work on

Check the [open issues](../../issues) for ideas. Issues labelled **`good first issue`** are a good starting point if you're new to the codebase.

If you want to work on something that isn't in the issue tracker, open an issue first to discuss the approach before writing code.

## Code of conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to abide by its terms.
