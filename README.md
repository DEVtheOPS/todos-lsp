# todos-lsp

`todos-lsp` is a clean-room Rust TODO scanner with two supported operator surfaces:

- a CLI that targets MVP behavioral parity with `ianlewis/todos` v0.14.0
- an additive Language Server Protocol entrypoint exposed as `todos-lsp serve`

The MVP parity goal is intentionally version-pinned and finite. After the bootstrap release, this project evolves independently rather than tracking upstream feature-for-feature.

## Installation

Install from crates.io once published:

```sh
cargo install todos-lsp
```

For local development or release validation, build from the repository:

```sh
cargo build --release
./target/release/todos-lsp --help
```

## Usage

Scan the current directory:

```sh
todos-lsp
```

Scan one or more paths and emit JSON lines:

```sh
todos-lsp -o json src tests
```

Common parity-oriented flags include:

```sh
todos-lsp --label owner -o json .
todos-lsp --charset detect .
todos-lsp --follow .
todos-lsp --blame .
todos-lsp --no-error-on-unsupported path/to/file.txt
```

TODO comments are reported through one shared internal finding model used by both the CLI and LSP paths.

## CLI parity scope

The bootstrap CLI parity target is `ianlewis/todos` v0.14.0 only. The parity harness compares observable behavior against a pinned upstream binary for the documented MVP rows, including recursive scans, unsupported input handling, JSON shape, labels, charset fallback, symlink traversal, representative language support, blame output, and multi-line block comment `comment_line` behavior.

CI and release validation require the pinned upstream binary and run the parity suite fail-closed. Missing parity tooling must block validation rather than silently skipping evidence.

## Language server

Start the LSP server over stdio with:

```sh
todos-lsp serve
```

The server supports diagnostics for open buffers, overlay-aware workspace symbols, and the `todos-lsp.listTodos` execute-command surface. Open-buffer scans resolve paths relative to the initialized workspace root so CLI and LSP relative paths remain equivalent.

## Release and publishing

Releases are managed with release-please. The publish workflow validates before publishing:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo package
cargo publish --dry-run
```

Publishing uses Cargo's `CARGO_REGISTRY_TOKEN` environment variable. Tokens are not passed on the command line.

## Clean-room and MVP-only parity

This repository is a clean-room Rust rewrite. `ianlewis/todos` v0.14.0 is used as a black-box behavioral reference for MVP parity only. Do not copy upstream source, tests, fixtures, prose, regexes, or language tables. Record parity fixture provenance in `docs/parity/` and recreate any tainted work rather than editing around contamination.

## Development checks

Recommended local verification:

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo llvm-cov --all-targets --summary-only
```

When running parity tests locally, provide `.tmp/todos-upstream` or set `UPSTREAM_TODOS_BIN` to the pinned upstream v0.14.0 binary.
