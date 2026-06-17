# Changelog

All notable changes to this project will be documented in this file.

This project follows semantic versioning. Release automation is managed by release-please.

## [Unreleased]

### Added
- Standalone Rust `todos-lsp` binary with parity-oriented CLI scanning.
- `todos-lsp serve` Language Server Protocol entrypoint over stdio.
- Shared internal finding model used by the CLI and LSP adapters.
- LSP diagnostics for open buffers, overlay-aware workspace symbols, and the `todos-lsp.listTodos` execute-command surface.
- Release-please-based release flow for the single Cargo package.
- Cargo publishing validation through `cargo package` and `cargo publish --dry-run`.

### Fixed
- Closed the documented MVP parity matrix with zero blocked rows against the pinned `ianlewis/todos` v0.14.0 binary.
- Added fail-closed handling for missing parity tooling and required runtime dependencies.
- Aligned open-buffer LSP scans with workspace-relative CLI path behavior.

### Changed
- Reframed upstream compatibility as an MVP-only black-box behavioral reference.
- Kept implementation and release packaging as one Rust package with internal module boundaries.
- Documented that post-MVP development is independent and is not bound to ongoing upstream lockstep.

### Removed
- Removed stale Zed-extension release-candidate notes from the project changelog.

### Release notes
- This release is not published until maintainers merge the release-please release PR and the tagged publish workflow succeeds.
- Install from crates.io only after publication with `cargo install todos-lsp`.
- For local validation before publication, build from the repository with `cargo build --release` and run `./target/release/todos-lsp --help`.
