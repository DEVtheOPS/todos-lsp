# Changelog

All notable changes to this project will be documented in this file.

This project follows semantic versioning. The current MVP release candidate is not final-public-release ready until the manual Zed dev-extension validation gate is completed.

## [0.2.0](https://github.com/DEVtheOPS/todos-lsp/compare/todos-lsp-v0.1.0...todos-lsp-v0.2.0) (2026-06-17)


### Features

* bootstrap todos-lsp mvp ([5e12624](https://github.com/DEVtheOPS/todos-lsp/commit/5e12624aa26995ddcf7b47c6ceaa7e413de45542))


### Bug Fixes

* resolve review blockers ([cdf275a](https://github.com/DEVtheOPS/todos-lsp/commit/cdf275aa457aabc17f4f622d9fb9c4ee537582e9))

## [0.1.0-rc.1] - 2026-05-16

### Added
- Initial Zed extension MVP that launches an external Rust `todo-lsp` language server.
- Host-agnostic Rust scanner core for configurable TODO/FIXME-style markers.
- Workspace and open-buffer scanning with include/exclude glob support and maximum file-size guardrails.
- Low-severity LSP diagnostics with source `todo-lsp` for navigation through Zed's built-in Diagnostics UI.
- Default marker support for `@TODO:`, `@FIXME:`, `TODO:`, `FIXME:`, `BUG:`, `HACK:`, `XXX:`, `[ ]`, and `[x]`.
- Markdown task detection for open and completed checklist items.
- User configuration through `lsp.todo-lsp.settings`, including tags, case sensitivity, boundary matching, globs, comment-aware policy, file-size limit, and diagnostic severity.
- Trusted binary-path configuration for `todo-lsp`; the extension fails closed rather than executing a workspace-local fallback binary.
- Documentation for MVP scope, settings, build commands, known limitations, and manual Zed dev-extension validation.

### Fixed
- Cleared code review and security review blockers before release-candidate preparation.
- Passed automated validation reported in session context: Rust tests, clippy, formatting, native build, WASM build, and coverage review.

### Changed
- Re-scoped the original TODO Tree-style request into a supported diagnostics-first MVP because public Zed extension APIs do not currently expose custom side panels/tree views or arbitrary editor decorations.
- Deferred custom TODO side panel, marker colors, gutter icons, status badges, toolbar actions, and TODO-specific filtering/grouping UI until Zed exposes public APIs for those surfaces.

### Removed
- No shipped functionality removed; this is the first MVP release candidate.

### Known Limitations
- [BLOCKER] Manual Zed dev-extension validation remains required before final public release, tagging, deployment, or publication.
- TODOs appear through Zed diagnostics and may be displayed alongside compiler/linter diagnostics.
- Cross-language attachment depends on Zed language-server registration behavior and must be validated on the target Zed channel.
- `comment_aware = "prefer_comments_fallback_to_lines"` currently falls back to line-based matching and may report markers in strings or non-comment text.
