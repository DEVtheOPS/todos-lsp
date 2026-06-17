# Implementation Plan: todos-lsp Bootstrap MVP

## Overview
Build `todos-lsp` as one publishable Rust package with internal `app`, `core`, `cli`, and `lsp` modules, one installed binary, MVP-only CLI behavioral parity against `ianlewis/todos` v0.14.0, and additive LSP support behind `todos-lsp serve`. The plan below is file-oriented, clean-room safe, and sequenced so parity evidence, shared-core runtime behavior, LSP overlay behavior, and release automation are all proven before the first publish gate.

## Regression Recovery Mode / Batch Milestone Gate
Because ADR-003 explicitly treats parity drift, repeated edge-case churn, and runtime contradictions as release blockers, implementation starts in **Batch Milestone Gate mode**:

- **Gate G0.1 — Provenance First:** no parser, discovery, CLI, or LSP runtime file moves to `VERIFIED` until `docs/parity/matrix.md` and `docs/parity/provenance.md` exist and the engineer has recorded the observed behavior category being implemented.
- **Gate G0.2 — Shared Model First:** no CLI-only or LSP-only finding shape is allowed; if a task needs a new finding field, it lands in `src/core/model.rs` first.
- **Gate G0.3 — Fail Closed First:** any blame/config/runtime dependency path that cannot be fully supported must error explicitly; do not ship best-effort partial results.
- **Gate G0.4 — Reset Rule:** if any prototype work reintroduces workspace/multi-crate MVP structure, separate LSP binary assumptions, shell-out-primary scanning, or derivative upstream artifacts, stop and reset before proceeding. Route reset impact to lead for approval.
- **Gate G0.5 — MVP Parity Boundary:** do not invent extra CLI behavior before the parity matrix closes the v0.14.0 surface. `serve` is the one additive command approved for MVP.

### Shared Runtime Constraint Codes
- **AC-R1:** single publishable `todos-lsp` package, single `todos-lsp` binary, `todos-lsp serve` entrypoint.
- **AC-R2:** CLI and LSP must share the same core finding model and scan/config behavior.
- **AC-R3:** fail-closed behavior for config/runtime dependency errors; no silent degradation.
- **AC-R4:** clean-room only; no copied upstream code/tests/fixtures/regexes/prose/tables.
- **AC-R5:** no shell-out-first or shell-out-primary architecture.
- **AC-R6:** release flow targets only the `todos-lsp` package.

### Completion Label Semantics
- **VERIFIED:** implementation landed, companion test/evidence landed, listed verification command passed, and required runtime evidence is attached.
- **PARTIAL:** file landed but a dependent runtime proof or paired evidence file is still pending.
- **UNVERIFIED:** scaffold or draft only; cannot be used to claim parity/runtime completion.

## Dependency Map

```text
Batch 1 Foundation ───────────────┐
                                  ├──> Batch 2 Provenance + Harness ───────┐
                                  │                                          ├──> Batch 4 Core Runtime ──┐
                                  │                                          │                           ├──> Batch 5 CLI Parity
                                  │                                          │                           └──> Batch 6 LSP Runtime
                                  └──> Batch 3 Release Skeleton ─────────────┘                                        │
                                                                                                                       └──> Batch 7 Release Validation + Docs
```

## Integration Checkpoints
- **CP-1 / Foundation:** single-package Cargo app builds; `todos-lsp --help` and `todos-lsp serve --help` route through one binary.
- **CP-2 / Provenance Gate:** parity matrix, provenance rules, fixture loader, upstream harness, and disposition logging exist before parser/discovery logic expands.
- **CP-3 / Shared Core Gate:** disk scan, overlay scan, config resolution, ignore/discovery, blame gating, and finding identity are all served from `src/core/*`.
- **CP-4 / CLI Gate:** parity harness proves output/exit behavior against the pinned upstream binary on the clean-room corpus.
- **CP-5 / LSP Gate:** `serve` publishes diagnostics and `workspace/symbol` from the same findings, with overlay content preferred over disk.
- **CP-6 / Release Gate:** CI, release-please, dry-run publish, packaged install smoke test, and crates.io release flow are all green for the single package only.

## Parallel Batch 1 (independent foundation)

### Task 1.1: Package manifest and binary boundary
**File:** `Cargo.toml`
**Test:** `tests/release_smoke.rs`
**Depends:** none

```toml
[package]
name = "todos-lsp"
version = "0.1.0"
edition = "2021"
license = "MPL-2.0"
description = "Standalone TODO scanner with CLI parity and LSP support"
repository = "https://github.com/devtheops/todos-lsp"
readme = "README.md"
keywords = ["todo", "lsp", "cli"]
categories = ["command-line-utilities", "development-tools"]

[[bin]]
name = "todos-lsp"
path = "src/main.rs"
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R6  
**Required verification evidence:** `cargo metadata`, `cargo build`, packaged binary named `todos-lsp`.  
**Do Not Regress checks:** no workspace section, no additional binary target, no unpublished path-crate dependency.  
**Completion:** VERIFIED only after `cargo build && cargo test --test release_smoke`.

### Task 1.2: Toolchain pin
**File:** `rust-toolchain.toml`
**Test:** `tests/release_smoke.rs`
**Depends:** none

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

**Verify:** `rustup show active-toolchain`

### Task 1.3: Binary entrypoint
**File:** `src/main.rs`
**Test:** `tests/cli_args.rs`
**Depends:** Task 1.1

```rust
mod app;
mod cli;
mod core;
mod lsp;

fn main() -> std::process::ExitCode {
    app::run(std::env::args_os())
}
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R2, AC-R5  
**Required verification evidence:** binary starts, routes `serve`, routes parity CLI path through app layer.  
**Do Not Regress checks:** no adapter logic in `main.rs`; no second executable path.  
**Completion:** VERIFIED after `cargo test --test cli_args`.

### Task 1.4: Thin app composition module
**File:** `src/app/mod.rs`
**Test:** `tests/cli_args.rs`
**Depends:** Task 1.3

```rust
pub fn run<I>(_args: I) -> std::process::ExitCode
where
    I: IntoIterator,
    I::Item: Into<std::ffi::OsString>,
{
    todo!("wire CLI and LSP dispatch only")
}
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R2  
**Required verification evidence:** command routing proof for default CLI path and `serve`.  
**Do Not Regress checks:** no parser/scanner logic in app layer.  
**Completion:** VERIFIED after route smoke tests pass.

### Task 1.5: Clean-room contribution rules
**File:** `CONTRIBUTING.md`
**Test:** `docs/parity/provenance.md`
**Depends:** none

```md
# Contributing
- Upstream `ianlewis/todos` v0.14.0 is a behavioral reference only.
- Do not copy upstream code, tests, fixtures, regexes, prose, or tables.
- Record provenance for parity fixtures, parser rules, and support tables.
- Reset tainted work rather than editing around contamination.
```

**Verify:** manual review against ADR-002

## Parallel Batch 2 (parity provenance and harness)

### Task 2.1: MVP parity matrix
**File:** `docs/parity/matrix.md`
**Test:** `tests/parity_cli.rs`
**Depends:** Task 1.5

```md
# todos-lsp MVP Parity Matrix
| ID | Upstream capability | todos-lsp command mapping | Dimensions | Fixture IDs | Evidence | Status | Notes |
|----|---------------------|---------------------------|------------|-------------|----------|--------|-------|
| CLI-001 | default recursive scan | `todos-lsp [PATH...]` | output, exit, discovery | ... | ... | Blocked | ... |
```

**Verify:** matrix covers commands, flags, output, exit, discovery, ignore, symlink, unsupported-file, blame, parsing, config.

### Task 2.2: Provenance workflow and disposition log rules
**File:** `docs/parity/provenance.md`
**Test:** `tests/parity_cli.rs`
**Depends:** Task 1.5

```md
# Provenance Workflow
1. Observe upstream only through allowed black-box/documented channels.
2. Author original fixture text and metadata.
3. Store ambiguity/disposition notes before claiming parity.
4. Escalate undocumented quirk mirroring for lead approval.
```

**Verify:** manual review against ADR-002 and ADR-003

### Task 2.3: Shared test support root
**File:** `tests/support/mod.rs`
**Test:** `tests/parity_cli.rs`
**Depends:** Task 1.1

```rust
pub mod fixtures;
pub mod upstream;
```

**Verify:** `cargo test --test parity_cli`

### Task 2.4: Fixture inventory and corpus loader
**File:** `tests/support/fixtures.rs`
**Test:** `tests/parity_cli.rs`
**Depends:** Task 2.3

```rust
pub struct FixtureCase {
    pub id: &'static str,
    pub root: &'static str,
    pub description: &'static str,
}
```

**Active Constraints / Do Not Regress:** AC-R4  
**Required verification evidence:** fixture metadata includes provenance hook, matrix IDs, env assumptions.  
**Do Not Regress checks:** no embedded copied upstream sample output.  
**Completion:** VERIFIED after harness loads authored corpus.

### Task 2.5: Upstream black-box harness adapter
**File:** `tests/support/upstream.rs`
**Test:** `tests/parity_cli.rs`
**Depends:** Task 2.3, Task 2.2

```rust
pub struct CommandObservation {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}
```

**Active Constraints / Do Not Regress:** AC-R3, AC-R4, AC-R5  
**Required verification evidence:** harness records exact stdout/stderr/exit triples for upstream and local runs.  
**Do Not Regress checks:** no shell-out-first product architecture; harness shell-out is test-only and isolated.  
**Completion:** VERIFIED after sample comparison run is stored.

### Task 2.6: End-to-end parity comparator
**File:** `tests/parity_cli.rs`
**Test:** `tests/parity_cli.rs`
**Depends:** Task 2.1, Task 2.4, Task 2.5

```rust
#[test]
fn compares_todos_lsp_to_pinned_upstream_for_mvp_rows() {
    todo!("iterate matrix-backed fixture invocations and compare behavior");
}
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R3, AC-R4  
**Required verification evidence:** diff output for mismatches, disposition logging, row status update path.  
**Do Not Regress checks:** parity rows may not close as “close enough”.  
**Completion:** VERIFIED after `cargo test --test parity_cli` passes against local and upstream binaries.

## Parallel Batch 3 (release skeleton)

### Task 3.1: Release-please configuration
**File:** `release-please-config.json`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 1.1

```json
{
  "$schema": "https://raw.githubusercontent.com/googleapis/release-please/main/schemas/config.json",
  "release-type": "rust",
  "packages": {
    ".": {
      "package-name": "todos-lsp"
    }
  }
}
```

**Verify:** release-please dry-run uses one package only.

### Task 3.2: Release manifest seed
**File:** `.release-please-manifest.json`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 3.1

```json
{
  ".": "0.1.0"
}
```

**Verify:** release-please dry-run resolves a single tag stream.

### Task 3.3: CI workflow baseline
**File:** `.github/workflows/ci.yml`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 1.1

```yaml
name: ci
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: cargo test --all-targets
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R6  
**Required verification evidence:** workflow run shows build, lint, unit, integration, parity job hooks.  
**Do Not Regress checks:** do not target workspace-wide publish behavior.  
**Completion:** VERIFIED after workflow validates on branch.

## Parallel Batch 4 (shared core runtime)

### Task 4.1: Core module boundary
**File:** `src/core/mod.rs`
**Test:** `tests/core_scan.rs`
**Depends:** Task 1.4, Task 2.1

```rust
pub mod blame;
pub mod config;
pub mod discovery;
pub mod errors;
pub mod index;
pub mod model;
pub mod parser;
pub mod scan;
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R2  
**Required verification evidence:** all runtime semantics route through core submodules only.  
**Do Not Regress checks:** no CLI/LSP imports in core.  
**Completion:** VERIFIED after core tests compile and pass.

### Task 4.2: Canonical finding and request model
**File:** `src/core/model.rs`
**Test:** `tests/core_scan.rs`
**Depends:** Task 4.1

```rust
pub struct Finding { /* id, uri/path, range, raw_text, todo_type, label, message, source */ }
pub struct DiskScanRequest { /* root, paths, config */ }
pub struct TextScanRequest { /* uri, text, language_hint, config */ }
```

**Active Constraints / Do Not Regress:** AC-R2  
**Required verification evidence:** same structs consumed by CLI rendering and LSP translation.  
**Do Not Regress checks:** no adapter-only fields in model.  
**Completion:** VERIFIED after dual-adapter compile usage exists.

### Task 4.3: Runtime error taxonomy
**File:** `src/core/errors.rs`
**Test:** `tests/config_fail_closed.rs`
**Depends:** Task 4.1

```rust
pub enum TodoError {
    InvalidConfig(String),
    MissingRuntimeDependency(String),
    UnsupportedInput(String),
    Io(String),
}
```

**Active Constraints / Do Not Regress:** AC-R3  
**Required verification evidence:** explicit error mapping for config, blame dependency, unsupported file, IO.  
**Do Not Regress checks:** no silent warning-only downgrade for required failures.  
**Completion:** VERIFIED after fail-closed tests pass.

### Task 4.4: Shared config resolution and precedence
**File:** `src/core/config.rs`
**Test:** `tests/config_fail_closed.rs`
**Depends:** Task 4.2, Task 4.3, Task 2.1

```rust
pub struct RuntimeConfig { /* todo_types, output, blame, ignores, config_path, ... */ }
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R3, AC-R4  
**Required verification evidence:** invalid config fails, flag-overrides-config precedence is tested, CLI and LSP consume same resolved config shape.  
**Do Not Regress checks:** no adapter-local default set.  
**Completion:** VERIFIED after config precedence cases pass.

### Task 4.5: File discovery and ignore traversal
**File:** `src/core/discovery.rs`
**Test:** `tests/core_scan.rs`
**Depends:** Task 4.2, Task 4.4, Task 2.1

```rust
pub fn discover_inputs(/* root, explicit_paths, config */) -> Result<Vec<std::path::PathBuf>, crate::core::errors::TodoError> {
    todo!()
}
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R5  
**Required verification evidence:** direct paths, recursion, ignore rules, symlink handling, unsupported-file semantics.  
**Do Not Regress checks:** discovery behavior must be parity-driven, not convenience-driven.  
**Completion:** VERIFIED after discovery matrix rows close.

### Task 4.6: Parser and TODO normalization
**File:** `src/core/parser.rs`
**Test:** `tests/core_scan.rs`
**Depends:** Task 4.2, Task 2.2

```rust
pub fn parse_text(/* request */) -> Result<Vec<crate::core::model::Finding>, crate::core::errors::TodoError> {
    todo!()
}
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R4  
**Required verification evidence:** comment-aware parsing, default TODO type set, label/message extraction, false-positive coverage, multi-language behavior from original fixtures.  
**Do Not Regress checks:** no copied regexes or language tables.  
**Completion:** VERIFIED after parser fixture rows and provenance notes are attached.

### Task 4.7: Blame execution boundary
**File:** `src/core/blame.rs`
**Test:** `tests/config_fail_closed.rs`
**Depends:** Task 4.2, Task 4.3, Task 2.1

```rust
pub fn enrich_with_blame(/* findings, repo */) -> Result<Vec<crate::core::model::Finding>, crate::core::errors::TodoError> {
    todo!()
}
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R3, AC-R5  
**Required verification evidence:** dependency-present success path, dependency-missing failure path, explicit git-repo assumptions.  
**Do Not Regress checks:** no partial blame output when dependency lookup fails.  
**Completion:** VERIFIED after blame success and failure evidence pass.

### Task 4.8: Scan orchestrator
**File:** `src/core/scan.rs`
**Test:** `tests/core_scan.rs`
**Depends:** Task 4.4, Task 4.5, Task 4.6, Task 4.7

```rust
pub fn scan_disk(/* request */) -> Result<Vec<crate::core::model::Finding>, crate::core::errors::TodoError> { todo!() }
pub fn scan_text(/* request */) -> Result<Vec<crate::core::model::Finding>, crate::core::errors::TodoError> { todo!() }
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R3  
**Required verification evidence:** disk/text scan parity on same content, stable finding ordering/identity, error propagation.  
**Do Not Regress checks:** no duplicate scan logic outside orchestrator.  
**Completion:** VERIFIED after disk-vs-overlay equivalence tests pass.

### Task 4.9: Workspace finding index
**File:** `src/core/index.rs`
**Test:** `tests/open_buffer_overlay.rs`
**Depends:** Task 4.2, Task 4.8

```rust
pub struct FindingIndex { /* disk findings, overlay findings, merge helpers */ }
```

**Active Constraints / Do Not Regress:** AC-R2  
**Required verification evidence:** stable per-document lookup for diagnostics, symbols, and CLI/LSP consistency checks.  
**Do Not Regress checks:** overlay merge rules live here or in dedicated overlay module, not scattered across handlers.  
**Completion:** VERIFIED after merged-view tests pass.

### Task 4.10: Core scan integration tests
**File:** `tests/core_scan.rs`
**Test:** `tests/core_scan.rs`
**Depends:** Task 4.2, Task 4.5, Task 4.6, Task 4.8

```rust
#[test]
fn scans_disk_and_text_through_same_model() {
    todo!()
}
```

**Verify:** `cargo test --test core_scan`

### Task 4.11: Config and fail-closed integration tests
**File:** `tests/config_fail_closed.rs`
**Test:** `tests/config_fail_closed.rs`
**Depends:** Task 4.3, Task 4.4, Task 4.7

```rust
#[test]
fn missing_required_runtime_dependency_fails_closed() {
    todo!()
}
```

**Verify:** `cargo test --test config_fail_closed`

## Parallel Batch 5 (CLI parity surface)

### Task 5.1: CLI module root
**File:** `src/cli/mod.rs`
**Test:** `tests/cli_args.rs`
**Depends:** Task 4.1

```rust
pub mod args;
pub mod commands;
pub mod render;
```

**Verify:** `cargo test --test cli_args`

### Task 5.2: Argument grammar and command mapping
**File:** `src/cli/args.rs`
**Test:** `tests/cli_args.rs`
**Depends:** Task 5.1, Task 2.1

```rust
pub enum Command {
    Scan,
    Serve,
}
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R3, AC-R4  
**Required verification evidence:** root scan invocation, `serve` subcommand, and only matrix-approved parity flags/aliases are accepted.  
**Do Not Regress checks:** do not speculate extra subcommands before matrix approval; help wording need not be copied, semantics must be.  
**Completion:** VERIFIED after CLI grammar tests and parity harness rows pass.

### Task 5.3: CLI execution adapter
**File:** `src/cli/commands.rs`
**Test:** `tests/parity_cli.rs`
**Depends:** Task 4.8, Task 5.2

```rust
pub fn run(/* parsed args */) -> Result<i32, crate::core::errors::TodoError> {
    todo!()
}
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R2, AC-R3  
**Required verification evidence:** core scan requests drive all CLI behavior, including config, blame, unsupported-file, and exit mapping.  
**Do Not Regress checks:** no direct parser/discovery logic inside CLI adapter.  
**Completion:** VERIFIED after parity comparator passes for implemented rows.

### Task 5.4: CLI output renderer
**File:** `src/cli/render.rs`
**Test:** `tests/cli_render.rs`
**Depends:** Task 4.2, Task 5.3, Task 2.1

```rust
pub fn render_plain(/* findings */) -> String { todo!() }
pub fn render_json(/* findings */) -> String { todo!() }
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R4  
**Required verification evidence:** stdout/stderr separation, JSON shape, ordering, newline behavior, blame rendering, unsupported-file messages.  
**Do Not Regress checks:** no copied upstream sample output templates.  
**Completion:** VERIFIED after renderer tests and parity rows close.

### Task 5.5: CLI grammar tests
**File:** `tests/cli_args.rs`
**Test:** `tests/cli_args.rs`
**Depends:** Task 1.3, Task 5.2

```rust
#[test]
fn parses_root_scan_and_serve_paths() {
    todo!()
}
```

**Verify:** `cargo test --test cli_args`

### Task 5.6: CLI renderer tests
**File:** `tests/cli_render.rs`
**Test:** `tests/cli_render.rs`
**Depends:** Task 5.4

```rust
#[test]
fn renders_plain_and_json_outputs_from_shared_findings() {
    todo!()
}
```

**Verify:** `cargo test --test cli_render`

## Parallel Batch 6 (LSP runtime)

### Task 6.1: LSP module root
**File:** `src/lsp/mod.rs`
**Test:** `tests/lsp_diagnostics.rs`
**Depends:** Task 4.1

```rust
pub mod commands;
pub mod overlay;
pub mod server;
pub mod session;
pub mod translate;
```

**Verify:** `cargo test --test lsp_diagnostics`

### Task 6.2: Open-buffer overlay store
**File:** `src/lsp/overlay.rs`
**Test:** `tests/open_buffer_overlay.rs`
**Depends:** Task 4.8, Task 4.9

```rust
pub struct OverlayStore { /* uri -> unsaved text + version */ }
```

**Active Constraints / Do Not Regress:** AC-R2  
**Required verification evidence:** open document content overrides disk content, close/save lifecycle returns to disk-backed index, no temp-file shelling.  
**Do Not Regress checks:** unsaved text must remain first-class.  
**Completion:** VERIFIED after overlay lifecycle tests pass.

### Task 6.3: Finding-to-LSP translation
**File:** `src/lsp/translate.rs`
**Test:** `tests/lsp_diagnostics.rs`
**Depends:** Task 4.2, Task 6.1

```rust
pub fn to_diagnostic(/* finding */) -> lsp_types::Diagnostic { todo!() }
pub fn to_workspace_symbol(/* finding */) -> lsp_types::SymbolInformation { todo!() }
```

**Active Constraints / Do Not Regress:** AC-R2  
**Required verification evidence:** diagnostics and symbols derive from the same finding fields and IDs.  
**Do Not Regress checks:** no LSP-only parsing model.  
**Completion:** VERIFIED after translation tests pass.

### Task 6.4: LSP session state
**File:** `src/lsp/session.rs`
**Test:** `tests/open_buffer_overlay.rs`
**Depends:** Task 4.4, Task 4.9, Task 6.2, Task 6.3

```rust
pub struct SessionState { /* config, workspace roots, index, overlays */ }
```

**Active Constraints / Do Not Regress:** AC-R2, AC-R3  
**Required verification evidence:** initialize validates config/dependencies, request handlers resolve merged view, overlay authority is centralized.  
**Do Not Regress checks:** no duplicated merge rules across request handlers.  
**Completion:** VERIFIED after session merge tests pass.

### Task 6.5: Additive server command surface
**File:** `src/lsp/commands.rs`
**Test:** `tests/lsp_workspace_symbols.rs`
**Depends:** Task 6.4

```rust
pub const LIST_TODOS_COMMAND: &str = "todos-lsp.listTodos";
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R2, AC-R3  
**Required verification evidence:** command is documented, deterministic, and reuses indexed findings rather than ad hoc editor logic.  
**Do Not Regress checks:** additive command must not weaken CLI parity boundaries.  
**Completion:** VERIFIED after command transcript evidence exists.  
**[CROSS-TEAM IMPACT]:** client command name/args become downstream integration contract; route final schema to lead.

### Task 6.6: LSP stdio server entrypoint
**File:** `src/lsp/server.rs`
**Test:** `tests/lsp_diagnostics.rs`
**Depends:** Task 6.4, Task 6.5

```rust
pub fn serve_stdio() -> Result<(), crate::core::errors::TodoError> {
    todo!()
}
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R2, AC-R3, AC-R5  
**Required verification evidence:** `todos-lsp serve` initializes, publishes diagnostics, supports `workspace/symbol`, and fails closed on invalid startup state.  
**Do Not Regress checks:** no separate binary path; no shell-out request flow.  
**Completion:** VERIFIED after LSP integration transcripts pass.

### Task 6.7: Overlay behavior tests
**File:** `tests/open_buffer_overlay.rs`
**Test:** `tests/open_buffer_overlay.rs`
**Depends:** Task 4.9, Task 6.2, Task 6.4

```rust
#[test]
fn open_buffer_overrides_disk_and_reverts_on_close() {
    todo!()
}
```

**Verify:** `cargo test --test open_buffer_overlay`

### Task 6.8: Diagnostics integration tests
**File:** `tests/lsp_diagnostics.rs`
**Test:** `tests/lsp_diagnostics.rs`
**Depends:** Task 6.3, Task 6.6

```rust
#[test]
fn publish_diagnostics_from_shared_core_findings() {
    todo!()
}
```

**Verify:** `cargo test --test lsp_diagnostics`

### Task 6.9: Workspace symbol and command integration tests
**File:** `tests/lsp_workspace_symbols.rs`
**Test:** `tests/lsp_workspace_symbols.rs`
**Depends:** Task 6.3, Task 6.5, Task 6.6

```rust
#[test]
fn workspace_symbol_uses_overlay_aware_index() {
    todo!()
}
```

**Verify:** `cargo test --test lsp_workspace_symbols`

## Parallel Batch 7 (release validation and operator docs)

### Task 7.1: Release-please workflow
**File:** `.github/workflows/release-please.yml`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 3.1, Task 3.2

```yaml
name: release-please
on:
  push:
    branches: [main]
```

**Active Constraints / Do Not Regress:** AC-R6  
**Required verification evidence:** dry-run or branch run shows one release PR stream for `todos-lsp`.  
**Do Not Regress checks:** no multi-package manifest publishing.  
**Completion:** VERIFIED after release-please dry-run output is captured.

### Task 7.2: Release validation workflow
**File:** `.github/workflows/release-validate.yml`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 3.3, Task 7.1

```yaml
name: release-validate
on: [workflow_dispatch, pull_request]
```

**Active Constraints / Do Not Regress:** AC-R6  
**Required verification evidence:** `cargo package`, `cargo publish --dry-run`, install smoke test, archived logs.  
**Do Not Regress checks:** validation must fail before any malformed publish attempt.  
**Completion:** VERIFIED after dry-run logs exist.

### Task 7.3: Publish workflow
**File:** `.github/workflows/publish.yml`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 7.1, Task 7.2

```yaml
name: publish
on:
  release:
    types: [published]
```

**Active Constraints / Do Not Regress:** AC-R3, AC-R6  
**Required verification evidence:** tag checkout, one-package publish, hard stop on first publish failure.  
**Do Not Regress checks:** no ad hoc manifest rewriting; no attempt to publish anything except `todos-lsp`.  
**Completion:** VERIFIED after non-publishing rehearsal succeeds.  
**[CROSS-TEAM NEED]:** crates.io owners, token, and GitHub secret posture must be confirmed before first release PR merge.

### Task 7.4: Operator-facing project README
**File:** `README.md`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 5.3, Task 6.6, Task 7.3

```md
# todos-lsp
- install `todos-lsp`
- run CLI parity surface from the same binary
- run LSP with `todos-lsp serve`
- document clean-room parity scope as MVP-only
```

**Verify:** manual doc review plus packaged help smoke test.

### Task 7.5: Packaged release smoke tests
**File:** `tests/release_smoke.rs`
**Test:** `tests/release_smoke.rs`
**Depends:** Task 1.1, Task 1.3, Task 7.2, Task 7.3

```rust
#[test]
fn packaged_binary_exposes_help_and_serve_help() {
    todo!()
}
```

**Active Constraints / Do Not Regress:** AC-R1, AC-R6  
**Required verification evidence:** packaged install runs `todos-lsp --help` and `todos-lsp serve --help`; release config is single-package.  
**Do Not Regress checks:** no separate executable or multi-package release output.  
**Completion:** VERIFIED after packaged smoke test and dry-run publish pass.

## Recommended Implementation Slices
1. **Slice A — Foundation + Gate Docs:** Batch 1, Tasks 2.1-2.2, 3.1-3.3. Stop at CP-1 and CP-2.
2. **Slice B — Harness Before Parser Growth:** Tasks 2.3-2.6. Upstream comparison must exist before broad CLI/runtime work.
3. **Slice C — Shared Core Only:** Batch 4. Stop at CP-3 and require disk/text equivalence proof.
4. **Slice D — CLI MVP Parity:** Batch 5 plus parity matrix row closure. Stop at CP-4 before any non-required CLI enhancement.
5. **Slice E — LSP MVP:** Batch 6. Stop at CP-5 with transcript evidence for diagnostics, `workspace/symbol`, and additive command.
6. **Slice F — Release Readiness:** Batch 7. Stop at CP-6 with release-please dry-run, cargo publish dry-run, and packaged smoke test evidence.

## Critical Verification Gates
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --test parity_cli
cargo test --test core_scan
cargo test --test config_fail_closed
cargo test --test cli_args
cargo test --test cli_render
cargo test --test open_buffer_overlay
cargo test --test lsp_diagnostics
cargo test --test lsp_workspace_symbols
cargo test --test release_smoke
cargo test --all-targets
cargo package
cargo publish --dry-run
```

## Lead Routing Notes
- **[CROSS-TEAM NEED]:** confirm crates.io ownership, token storage, and GitHub release permissions before first release PR merge.
- **[CROSS-TEAM IMPACT]:** if parity requires reproducing undocumented upstream quirks exactly, stop and route to lead before implementation hardens.
- **[CROSS-TEAM IMPACT]:** final LSP command schema is a downstream editor integration contract; route naming and args to lead before declaring it stable.
- **Breaking/reset path:** if any earlier local experiments assumed workspace crates, a separate LSP binary, or copied upstream artifacts, delete and recreate that work before Batch 4.

## Integration Verification
```bash
cargo test --all-targets && cargo clippy --all-targets --all-features -- -D warnings && cargo fmt --check
```

[DONE] docs/thoughts/plans/bootstrap-implementation-plan.md
