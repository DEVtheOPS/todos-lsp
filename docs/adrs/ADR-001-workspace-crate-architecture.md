# ADR-001: Single-Package Module Architecture for MVP

## Status
Accepted

## Context
`todos-lsp` needs the smallest architecture that still protects the approved MVP constraints:
- standalone Rust implementation
- one shared internal core model used by both CLI and LSP
- one user-facing binary named `todos-lsp`
- LSP launched as `todos-lsp serve`
- clean-room implementation with no dependency on upstream Go internals
- no shell-out-first architecture
- MVP-only parity framing against `ianlewis/todos` v0.14.0
- remote-friendly separate binary installation model
- post-MVP freedom to evolve independently of upstream parity obligations

Earlier design work proposed a Rust workspace with multiple internal crates. That shape is viable long term, but it is larger than the approved MVP reset and introduces setup, packaging, and release complexity before the product has proven that crate extraction is necessary. The execution verdict for this phase is to tighten the design and defer workspace/crate splitting until post-MVP unless real implementation pain justifies it.

The architecture decision therefore needs to optimize for the smallest approved design reset while preserving hard boundaries between shared scanning logic, CLI concerns, and LSP concerns.

## Options Considered
| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| A. Single publishable Rust package `todos-lsp` with internal modules for core, CLI adapter, and LSP adapter | Smallest approved MVP shape; simplest Cargo/release/publish story; preserves one binary and one shared core model; keeps boundaries explicit without premature crate overhead; easy to install remotely as one artifact | Requires discipline to keep module boundaries clean inside one package; extraction to crates later is a deliberate refactor | ✓ |
| B. Rust workspace with `todo-core`, adapter crates, and one `todos-lsp` binary crate | Strong physical separation; future extraction already done; fine-grained crate tests | More setup and release complexity now; premature packaging split; conflicts with approved MVP tightening direction; adds cost before proof of need | ✗ |
| C. Monolithic single package with no meaningful internal boundaries | Fastest to start | High risk of CLI/LSP divergence; weak testing boundaries; harder post-MVP extraction; encourages convenience coupling | ✗ |
| D. Separate user-facing binaries for CLI and LSP | Clear executable separation | Violates approved command surface and remote install model; increases user-visible surface and release complexity | ✗ |

## Decision
Adopt a single publishable Rust package named `todos-lsp` for MVP. Inside that package, maintain explicit internal module boundaries for:
- shared core concerns
- CLI adapter concerns
- LSP adapter concerns
- thin application composition and command routing

Ship exactly one user-facing binary, `todos-lsp`. The LSP server is started via the `serve` subcommand. Workspace/crate extraction is explicitly deferred until post-MVP and only if implementation pain demonstrates a real need.

### Architecture Overview
```text
todos-lsp (single Cargo package)
|
+-- src/main.rs                # binary entrypoint and command routing
+-- src/app/                   # composition/wiring only
+-- src/core/                  # canonical scan/domain/config/finding logic
+-- src/cli/                   # CLI adapter over core
+-- src/lsp/                   # LSP adapter over core
+-- tests/
    +-- fixtures/              # shared clean-room corpus
    +-- parity/                # CLI parity/integration tests
    +-- lsp/                   # protocol/integration tests
```

Logical dependency direction:

```text
main/app -> cli -> core
main/app -> lsp -> core
cli  -/-> lsp
lsp  -/-> cli
core -/-> cli/lsp
```

### Data Flow
1. `todos-lsp` parses process arguments and chooses either a parity-oriented CLI path or `serve`.
2. The app layer wires runtime dependencies and dispatches to the relevant adapter.
3. The CLI adapter translates arguments, flags, and output expectations into core scan requests.
4. The LSP adapter translates protocol messages, workspace state, and unsaved document text into core scan requests.
5. The core performs scanning, parsing, filtering, ignore handling, finding classification, and shared configuration resolution.
6. The core returns a transport-neutral result model.
7. The adapter formats that result for its transport:
   - CLI output and exit behavior for the CLI path
   - diagnostics, symbols, and command responses for the LSP path

### Key Interfaces
- `core`
  - Owns the canonical domain model for findings, locations, scan requests, scan results, marker classification, and shared configuration.
  - Exposes entrypoints for disk/workspace scanning and unsaved-text/document scanning.
  - Must not depend on Clap, terminal rendering, LSP protocol types, or editor-specific state.

- `cli`
  - Maps CLI commands and flags to core requests.
  - Maps core results to parity-compatible stdout/stderr and exit semantics.
  - Must not implement independent scanning or parsing logic.

- `lsp`
  - Owns protocol session state, workspace/document lifecycle, diagnostics publication, `workspace/symbol`, and the MVP additive server command surface.
  - Converts workspace and buffer events into core requests.
  - Must not implement a second TODO parser or scanner.

- `app`
  - Owns startup wiring, dependency construction, and top-level command dispatch.
  - Must remain thin and avoid accumulating domain logic.

## Consequences

### Benefits
- Matches the smallest approved MVP architecture reset.
- Keeps Cargo publishing and release automation centered on one package and one binary.
- Preserves one shared source of truth for TODO semantics across CLI and LSP.
- Retains clean internal seams for later crate extraction if justified.
- Reduces setup and migration cost now without giving up post-MVP evolution paths.

### Tradeoffs
- Module boundaries are logical rather than enforced by Cargo package boundaries.
- Future crate extraction will be a planned refactor if the codebase outgrows this shape.
- Engineers must actively prevent convenience imports and adapter leakage into the core.

### Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Core logic drifts toward CLI- or LSP-specific assumptions | Medium | High | Keep transport types out of core APIs; review dependency direction; add shared-fixture consistency tests |
| CLI and LSP diverge in behavior over time | Medium | High | Require both adapters to consume the same core model and cover with dual-adapter tests over the same fixtures |
| Single package turns into an unstructured monolith | Medium | Medium | Enforce explicit module ownership and keep `app` thin; reject duplicate parser/scanner logic in adapters |
| Later crate extraction is needed and requires refactoring | Medium | Medium | Design module seams now to mirror likely future crate boundaries; defer extraction until pain is proven |
| Release/publish assumptions drift toward multi-package behavior prematurely | Low | Medium | Keep MVP release architecture centered on one publishable `todos-lsp` package only |

## Implementation Notes
- Package shape should remain a single Cargo package for MVP.
- The only public installation target is the `todos-lsp` binary.
- `serve` is the only LSP entrypoint.
- Internal module boundaries should be treated as architectural contracts even though they live in one package.
- `core` is the only place allowed to own:
  - scan orchestration
  - file eligibility and ignore evaluation
  - parser/comment-awareness rules
  - finding identity and location model
  - shared config schema/defaults
- `cli` may own:
  - command/flag mapping
  - output rendering
  - exit code mapping
- `lsp` may own:
  - protocol state and session lifecycle
  - document synchronization state
  - diagnostics/symbol translation
  - additive server command handling
- Testing boundaries:
  - `core`: unit tests plus fixture-driven behavior tests for scanning, parsing, ignore handling, config resolution, and finding identity.
  - `cli`: adapter tests for argument mapping, output formatting, and exit behavior; parity integration tests against the shared fixture corpus.
  - `lsp`: protocol/session tests, document lifecycle tests, diagnostics/symbol mapping tests, and unsaved-buffer behavior tests.
  - binary/app: minimal smoke tests for command routing and startup wiring.
  - cross-cutting: consistency tests must prove CLI and LSP produce materially equivalent findings for the same content/configuration through the shared core.
- Suggested source layout:

```text
src/
  main.rs
  app/
    mod.rs
  core/
    mod.rs
    scan.rs
    config.rs
    model.rs
    parser.rs
  cli/
    mod.rs
    commands.rs
    render.rs
  lsp/
    mod.rs
    server.rs
    session.rs
    translate.rs
```

- Deferred until post-MVP unless proven necessary:
  - Rust workspace setup
  - extraction of `core`, `cli`, or `lsp` into separate crates
  - any public library crate/API commitment beyond the binary package
- Extraction trigger should be real pain, such as:
  - repeated ownership or compile-boundary issues
  - testing isolation problems not solvable within the package
  - materially different release/support requirements for subcomponents
  - sustained architectural friction confirmed after MVP
- If extraction is pursued later, the first target shape should preserve the same logical seams already defined here rather than redesigning the product surface.

## Questions for User
- None.
