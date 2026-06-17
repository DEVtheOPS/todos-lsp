# ADR-004: LSP Architecture and Command Surface

## Status
Proposed

## Context
`todos-lsp` must ship a first-party LSP server without introducing a second user-facing binary or a second scanning implementation. The MVP is explicitly constrained to a unified `todos-lsp` binary with `todos-lsp serve` as the LSP entrypoint, diagnostics plus `workspace/symbol` as the supported LSP features, open-buffer scanning for unsaved editor text, remote-friendly deployment on developer hosts, and shared core reuse with the CLI.

Several forces shape this decision:
- CLI and LSP must stay behaviorally consistent over the same finding model.
- Editor integrations, especially remote-development setups, need a stable executable they can invoke directly on the target host.
- Unsaved-buffer analysis must not depend on disk writes.
- The architecture must be transport-aware but not transport-coupled to any one editor.
- Missing required runtime dependencies or configuration must fail closed rather than silently producing incomplete results.
- The MVP must not bind itself to a separate LSP-only binary, shell-out-first execution, or editor-bundled server assets.

## Options Considered
| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| A. Unified `todos-lsp` binary with `serve` subcommand, in-process LSP over stdio, shared workspace/buffer state and shared core services | Single install target; remote-friendly; consistent CLI/LSP behavior; lowest adapter drift; simple editor launch contract; compatible with future alternate transports behind same service layer | Requires clear internal service boundaries so CLI concerns do not leak into LSP runtime; single binary means runtime dependency checks must be feature-scoped | ✓ |
| B. Separate LSP-only binary (for example `todos-lsp-server`) plus CLI binary | Smaller conceptual surface per binary; editor entrypoint is explicit | Violates approved command surface; adds packaging/install complexity; increases drift risk between adapters; worse remote UX; unnecessary MVP fragmentation | ✗ |
| C. CLI-driven or shell-out-primary LSP adapter that invokes scanning commands per request | Reuses CLI superficially; may appear faster to bootstrap | Rejected by constraints; poor latency; weak unsaved-buffer support; brittle process orchestration; inconsistent request/state model; difficult incremental behavior | ✗ |
| D. Unified binary but dedicated network server mode as MVP default transport | Potentially useful for shared daemon scenarios and remote multiplexing | Unnecessary MVP complexity; editor interoperability is best with stdio; expands security and lifecycle concerns too early | ✗ |

## Decision
Adopt a unified binary command surface where `todos-lsp serve` is the only MVP LSP entrypoint. The LSP server runs in-process within the shared Rust workspace, uses stdio JSON-RPC/LSP transport for editor integration, and delegates all scanning, indexing, and finding normalization to shared core services also used by the CLI.

The LSP runtime will maintain a workspace state model composed of:
- workspace configuration snapshot
- on-disk workspace index
- open-document overlay store for unsaved text
- projection layer that resolves each request against the shared finding/index model

For MVP, the LSP supports:
- `textDocument/publishDiagnostics`
- `workspace/symbol`

Open-buffer content is treated as authoritative for any currently open document and overlays the disk-backed workspace view. The LSP request handlers do not implement independent scanning rules; instead they invoke shared scan/index services with either disk-backed inputs, overlay-backed inputs, or a merged view depending on request type.

Failure posture is fail-closed. If a requested operation depends on required runtime dependencies or validated configuration that are unavailable, invalid, or unsupported for that request, the server returns an explicit LSP error or initialization failure and does not silently omit partial findings.

A separate LSP-only binary is explicitly rejected for MVP because it creates needless packaging and integration split, increases long-term drift risk between CLI and LSP adapters, and directly conflicts with the approved product shape and remote-friendly install model.

### Architecture Overview
```text
+--------------------------- todos-lsp ---------------------------+
|                                                                |
|  CLI subcommands                           serve               |
|  (parity surface)                          (LSP entrypoint)    |
|        |                                         |             |
|        v                                         v             |
|   CLI adapter ------------------------+   LSP transport        |
|                                       |   (stdio JSON-RPC)     |
|                                       v             |          |
|                           Shared application services          |
|                    +--------------------------------------+    |
|                    | Config resolution                     |    |
|                    | Scan orchestration                    |    |
|                    | Workspace index                       |    |
|                    | Open-buffer overlay store             |    |
|                    | Finding normalization / identities    |    |
|                    +-------------------+------------------+    |
|                                        |                       |
|                                        v                       |
|                                  todo-core                     |
|                           parser / scanner / domain model      |
+----------------------------------------------------------------+
```

### Data Flow
1. Editor launches `todos-lsp serve` on the local or remote host using stdio transport.
2. During `initialize`, the server validates required configuration/runtime dependencies for enabled capabilities and establishes workspace roots.
3. The LSP adapter creates or refreshes shared services for config, workspace index, and document overlays.
4. Disk-backed indexing scans workspace files through `todo-core` and stores normalized findings keyed by stable document identity.
5. When a document is opened or changed, the LSP stores the unsaved text in the overlay store and triggers a targeted rescan for that document through the same core scanner.
6. Diagnostics publication reads findings from the merged view: overlay findings for open documents, disk index findings for closed documents.
7. `workspace/symbol` queries the normalized finding index, returning symbols generated from the same findings model used by diagnostics.
8. On save/close, overlay lifecycle updates determine whether to keep overlay-backed findings, replace them with disk-backed findings, or clear them.

### Key Interfaces
- Command contract:
  - `todos-lsp serve`
  - no separate LSP user-facing binary in MVP
- Core scan inputs:
  - `DiskScanRequest { workspace_root, paths, config }`
  - `TextScanRequest { uri, language_hint, text, config, workspace_context }`
- Shared finding model:
  - `Finding { id, uri, range, title, body, labels, category, source_kind }`
- Index projection contract:
  - disk index stores findings for persisted workspace content
  - overlay store stores latest in-memory document text and derived findings
  - request resolution chooses overlay findings when a URI is open and dirty
- LSP mappings:
  - diagnostics = `Finding -> Diagnostic`
  - workspace symbols = `Finding -> SymbolInformation` or `WorkspaceSymbol`
- Failure contract:
  - initialization failure for globally required missing dependencies/configuration
  - request error plus no partial success for request-scoped missing dependencies/configuration

## Consequences

### Benefits
- One executable and one install story for CLI and editor integrations.
- Strong consistency because CLI and LSP project the same core findings.
- Unsaved-buffer support is first-class rather than an afterthought.
- Stdio transport fits mainstream editor and remote-host LSP execution models.
- The service-layer split preserves room for future transports without changing the user-facing MVP contract.

### Tradeoffs
- The LSP runtime must manage cache/index/overlay state rather than treating each request as stateless.
- Initialization and request validation are stricter because the server must fail closed instead of degrading.
- `workspace/symbol` quality depends on the normalized shared index being maintained correctly across disk and overlay events.

### Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Overlay and disk index drift produce inconsistent diagnostics or symbols | Medium | High | Treat overlay as authoritative per open URI; centralize merge logic in shared services; add dual-view integration tests |
| Large workspaces make initial indexing or symbol queries too slow | Medium | High | Separate targeted overlay rescans from workspace indexing; cache normalized findings; bound synchronous work per request |
| Missing runtime dependencies are discovered too late in request handling | Medium | Medium | Validate capability prerequisites during initialize and again at request boundary where feature-specific |
| Editor clients assume network transport or bundled binaries | Low | Medium | Document stdio launch contract and remote-host install expectations clearly |
| Early implementation shortcuts couple LSP handlers directly to scanner internals | Medium | High | Enforce adapter-to-service-to-core boundaries in crate APIs and code review |
| Future addition of more LSP features pressures MVP abstractions prematurely | Medium | Medium | Keep MVP request mapping narrow and service-oriented; defer non-MVP protocol features until separately approved |

## Implementation Notes
- The engineer-agent should implement the LSP adapter as a thin transport layer over shared services, not as a second scanner.
- `serve` should be the only documented MVP LSP launch path; other runtime flags must remain subordinate to this contract.
- Prefer stdio transport for MVP. If future network or daemon modes are added, they must reuse the same service layer and preserve `todos-lsp serve`.
- The shared indexing model should expose targeted document rescans for `didOpen`/`didChange` and broader workspace refresh hooks for disk-backed updates.
- Diagnostics and symbol generation must derive from the same `Finding` instances to preserve CLI/LSP equivalence.
- Unsaved text must never require temporary-file shelling or disk writes.
- Required runtime dependency checks must be explicit and surfaced as structured initialization/request failures; no silent omission of findings.
- The planner should add integration coverage for:
  - open buffer diverges from disk
  - dirty buffer closes and reverts to disk index
  - `workspace/symbol` reflects overlay-backed findings
  - missing dependency/configuration yields explicit failure
- This ADR introduces a deliberate breaking constraint if any prototype work assumed a separate LSP binary or shell-out request path; that work should be reset before implementation proceeds rather than carried forward as debt. [CROSS-TEAM REVIEW]
- The stdio-only MVP transport contract and any future additive server command schema constrain downstream editor/client integrations. [CROSS-TEAM IMPACT]

## Questions for User
- None at this time.
