# Changelog

All notable changes to rustgrep are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [Unreleased]

### Added
- Parallel multi-file search via `rayon`
- Colored output — filenames in cyan, line numbers in green (`owo-colors`)
- Full regex matching (`-E` / `--regex`) via the `regex` crate
- Case-insensitive matching (`-i` / `--ignore-case`)
- Recursive directory traversal (`-r` / `--recursive`) via `walkdir`
- Line number display (`-n` / `--line-numbers`)
- Count-only mode (`-c` / `--count`)
- Invert match (`-v` / `--invert`)
- Unified `AppError` enum with `thiserror`
- `Matcher` trait with strategy pattern — `LiteralMatcher`, `CaseInsensitiveMatcher`, `RegexMatcher`
- `build_matcher` factory that selects the right strategy from CLI flags
- grep-compatible exit codes (exit 1 when no matches found)
