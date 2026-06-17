# Milestone Plan: Bootstrap MVP Foundation

## Objective
Deliver the smallest integrated MVP foundation for `todos-lsp`: a clean-room single-package Rust application with shared internal modules, a unified `todos-lsp` binary command surface, initial CLI behavioral parity verification against `ianlewis/todos` v0.14.0 as the bootstrap release gate, additive LSP capabilities exposed via `todos-lsp serve`, and release automation that can publish the approved Cargo package through GitHub Actions + release-please. After MVP, the project may evolve independently and is not committed to perpetual upstream parity tracking unless a later spec explicitly says otherwise.

## Smallest Integrated Slice
A stable single-package Rust application containing internal core, CLI, LSP, and app wiring modules behind one installed `todos-lsp` binary, where:
- the CLI proves MVP bootstrap parity on a curated clean-room fixture corpus against the pinned `ianlewis/todos` v0.14.0 reference,
- the parity evidence is sufficient to declare MVP complete without creating any standing obligation to mirror future upstream changes,
- the unified command surface uses `todos-lsp serve` for the LSP server and parity-oriented subcommands for the remaining MVP CLI scope,
- the LSP uses the same internal core model to publish diagnostics and `workspace/symbol` results,
- CI can validate release-please and Cargo publishing readiness before the first public release.

## Scope Boundaries
### In Scope
- Formal clean-room implementation workstream and approval checkpoints
- Standalone single-package Rust bootstrap deliverables with internal core/CLI/LSP/app module boundaries behind the `todos-lsp` binary
- Behavioral parity planning and MVP-gate verification against `ianlewis/todos` v0.14.0 CLI behavior
- Shared core boundaries for disk scanning and unsaved buffer scanning
- MVP LSP support exposed via `todos-lsp serve`, including diagnostics, `workspace/symbol`, and any required additive server command surface
- Parity evidence gates, fixture strategy, and integration validation order
- GitHub Actions release automation using release-please
- Cargo/crates.io publication readiness for the public `todos-lsp` package only
- Initial packaging, metadata, versioning, and release dry-run readiness

### Out of Scope
- Ongoing post-MVP parity maintenance obligations to `ianlewis/todos`
- Post-MVP parity tracking for upstream versions after v0.14.0 unless a future spec explicitly adds it
- Advanced LSP features beyond diagnostics, `workspace/symbol`, and the MVP `serve` entrypoint plus required command surface
- Editor extension bundling or client-specific UX work
- Replacing release-please with alternate tooling
- Partial CLI parity releases marketed as MVP-complete
- Source-derived upstream fixtures, regexes, tests, or implementation tables
- Rust workspace setup or crate splitting during MVP

## Active Constraints / Do Not Regress
- Version baselines: Rust stable current-generation toolchain; single-package Cargo application on stable ecosystem; GitHub Actions stable runners/actions; release-please-compatible stable workflow components; upstream behavioral reference pinned to `ianlewis/todos` v0.14.0
- Preserve: clean-room rewrite posture; standalone unified binary model; shared internal core independence; CLI/LSP behavior consistency; MVP parity gate against `ianlewis/todos` v0.14.0; post-MVP product autonomy; remote-friendly LSP binary deployment via `todos-lsp serve`; fail-closed dependency handling; MPL-2.0 posture
- Do not do: copy upstream source/tests/README prose/regexes/language tables; design shell-out-first or shell-out-primary architecture; weaken the MVP parity gate; imply perpetual upstream parity maintenance; imply a separate LSP-only binary; replace release-please; defer parity evidence until after implementation closes; reintroduce Rust workspace or multi-crate packaging in MVP
- Regression checks: MVP parity must be proven by matrix + fixture tests before MVP sign-off; milestone language must keep that gate bounded to the bootstrap release boundary; the installed binary must remain `todos-lsp` with `serve` as the LSP entrypoint; remaining MVP CLI scope must stay on parity-oriented subcommands in that same binary; CLI and LSP must consume the same internal core finding model; release flow must cover Cargo/crates.io and GitHub release automation expectations for the single publishable `todos-lsp` package; bootstrap must not reintroduce workspace/crate splitting for convenience
- Evidence required: approved ADRs; clean-room rules doc; parity matrix; original fixture corpus; automated behavioral comparison results; single-package build/test logs; LSP runtime evidence; release dry-run logs; packaging metadata review; milestone gate sign-offs

## ADR Checkpoints
| Checkpoint | ADR Topic | Purpose | Depends On | Can Run In Parallel | Approval Needed |
| ---------- | --------- | ------- | ---------- | ------------------- | --------------- |
| ADR-1 | Single-package MVP architecture (`todos-lsp` with internal core/CLI/LSP/app modules) | Lock module boundaries, shared domain ownership, runtime composition, and single-binary remote deployment model | none | yes | lead |
| ADR-2 | Clean-room implementation and parity-observation rules | Formalize allowed reference materials, authoring constraints, review rules, and escalation path for ambiguous upstream quirks | none | yes | lead |
| ADR-3 | CLI MVP parity contract and verification strategy | Define parity matrix structure, fixture methodology, behavior categories, and pass/fail thresholds for the bootstrap MVP release against v0.14.0, while explicitly scoping parity obligations to MVP unless later re-chartered | ADR-2 | no | lead |
| ADR-4 | LSP architecture and command surface | Define buffer/workspace scan model, diagnostics + `workspace/symbol` flow, `todos-lsp serve` entrypoint contract, and any additive server command surface needed beyond MVP parity CLI behavior | ADR-1 | yes | lead |
| ADR-5 | Release and publish architecture | Define release-please workflow, single-package publication boundary, versioning/changelog inputs, and Cargo publish sequence | ADR-1 | yes | lead |

## Implementation Checkpoints
| Checkpoint | Outcome | Depends On | Integration Gate | Evidence Required |
| ---------- | ------- | ---------- | ---------------- | ----------------- |
| CP-1 | Bootstrap single-package repository skeleton with internal core/CLI/LSP/app module boundaries, shared lint/test baseline, package metadata scaffold, and contribution guardrails for one installed `todos-lsp` binary | ADR-1, ADR-2 | G1 | `cargo` single-package build logs, module boundary summary, command-surface summary, clean-room contribution docs, initial CI pass |
| CP-2 | MVP parity baseline assets: authored parity matrix, clean-room fixture corpus, upstream observation notes, and comparison harness contract | ADR-2, ADR-3 | G2 | parity matrix draft with post-MVP autonomy note, fixture inventory, harness design notes, sample comparison output |
| CP-3 | Shared MVP scanning domain for disk + unsaved text inputs with stable finding model consumed by both adapters | ADR-1, ADR-3, ADR-4 | G3 | core API docs, adapter integration proof, representative scan tests, finding-model consistency evidence |
| CP-4 | Unified `todos-lsp` MVP parity command implementation against the pinned v0.14.0 behavior surface, establishing the bootstrap release gate rather than an indefinite upstream maintenance contract | ADR-3, CP-2, CP-3 | G4 | behavioral comparison test matrix, command mapping summary, exit-code/output evidence, unsupported-path/ignore/symlink/error-mode coverage, explicit note that future divergence requires later product/spec approval |
| CP-5 | `todos-lsp serve` MVP implementation with diagnostics, `workspace/symbol`, and any required additive server command surface routed through shared core | ADR-4, CP-3 | G5 | LSP transcript/logs, diagnostics screenshots or captures, symbol query evidence, `serve` invocation evidence, command-surface protocol evidence |
| CP-6 | Release automation + publishing readiness via GitHub Actions, release-please, single-package validation, and Cargo dry-run/publish flow | ADR-5, CP-1, CP-4, CP-5 | G6 | workflow run logs, release-please dry-run evidence, Cargo publish dry-run output, artifact/package metadata review |

## Integration Gates
| Gate | Happens After | Validates | Required Evidence | Reviewer Timing |
| ---- | ------------- | --------- | ----------------- | --------------- |
| G1 | CP-1 | Single-package shape supports independent core/CLI/LSP evolution without violating clean-room constraints and still ships one `todos-lsp` binary | repo tree, package manifest, module boundary summary, command-surface summary, CI logs, contribution/rules docs | after build only |
| G2 | CP-2 | MVP parity evidence system is rigorous enough before feature implementation scales and clearly bounded to the bootstrap release target | approved parity matrix categories, fixture provenance notes, harness acceptance criteria, milestone/ADR language confirming post-MVP autonomy | after build only |
| G3 | CP-3 | Shared core can serve both CLI and LSP without adapter-specific divergence | dual-adapter integration tests, unsaved-text vs disk evidence, API boundary review | after build only |
| G4 | CP-4 | CLI parity meets the v0.14.0 bootstrap behavioral gate, no feature class is left as "close enough," and MVP completion does not imply perpetual upstream parity duties | full comparison matrix results, failing-case disposition log, exit/output parity evidence, release-boundary parity sign-off note | after build only |
| G5 | CP-5 | Additive LSP behavior is integrated over the same model without regressing parity assumptions and is launched via `todos-lsp serve` | LSP scenario runs, diagnostics/symbol evidence, `serve` behavior notes | after build only |
| G6 | CP-6 | Release pipeline can version, tag, and publish the intended public `todos-lsp` package without manual surgery | release-please workflow logs, Cargo dry-run/publish checks, release artifact checklist | after build only |

## Dependency Map
```text
ADR-1 Single-Package Architecture ──┬────> CP-1 Bootstrap Package ──────────┬────> G1
                                    │                                       │
                                    │                                       ├────> CP-3 Shared Core ─────> G3
                                    │                                       │                            │
                                    │                                       │                            ├────> CP-4 CLI Parity ─────> G4
                                    │                                       │                            └────> CP-5 LSP MVP ───────> G5
                                    │                                       │                                                         │
                                    │                                       └─────────────────────────────────────────────────────────┴────> CP-6 Release Automation ─────> G6
                                    │
ADR-2 Clean-Room Rules ─────────────┼────> CP-1 Bootstrap Package
                                    └────> ADR-3 CLI Parity Contract ───────> CP-2 Parity Assets ─────> G2 ─────> CP-4 CLI Parity

ADR-4 LSP Architecture ──────────────────────────────────────────────────────> CP-3 Shared Core
                                                                               └───────────────> CP-5 LSP MVP

ADR-5 Release Architecture ───────────────────────────────────────────────────────────────────> CP-6 Release Automation
```

## Cross-Team Coordination
- [CROSS-TEAM REVIEW]: lead, ADR-2, clean-room policy must be approved before parity fixtures and implementation proceed.
- [CROSS-TEAM IMPACT]: lead, ADR-3/G4, any requirement to reproduce undocumented upstream quirks exactly raises legal/maintenance risk and needs explicit approval; any proposal to extend parity obligations beyond MVP also requires explicit re-chartering.
- [CROSS-TEAM REVIEW]: lead, ADR-4, the `todos-lsp serve` transport contract and any additive MVP LSP command schema must be approved before client integration work proceeds.
- [CROSS-TEAM NEED]: release/maintainer owner, ADR-5/CP-6, crates.io ownership, token handling, and GitHub Actions secret posture must exist before first publish.
- [CROSS-TEAM IMPACT]: downstream editor/client integrators, CP-5, LSP command name/arguments and remote binary expectations will define future client integration constraints.

## Risks And Mitigations
| Risk | Likelihood | Impact | Mitigation |
| ---- | ---------- | ------ | ---------- |
| Ambiguous upstream behavior creates parity disputes late in implementation | Med | High | Resolve through ADR-3 observation rules, parity notes, and early G2 approval before broad implementation |
| Clean-room violations through copied fixtures/regexes/prose | Med | High | Enforce ADR-2 contribution rules, provenance review, and lead sign-off before parity assets are accepted |
| Core APIs drift toward CLI-first or LSP-first assumptions | Med | High | Lock boundaries in ADR-1 and validate with G3 dual-adapter evidence |
| Unified command surface is implemented with split-binary assumptions and must be reset | Med | High | Lock `todos-lsp` + `serve` command contract in ADR-1/ADR-4 and reject any bootstrap work that introduces separate CLI/LSP binaries |
| Achieving true 100% CLI parity for MVP takes longer than expected because hidden edge cases emerge | High | High | Use parity matrix categories early, comparison harness from CP-2, and do not market MVP completion before G4 passes |
| Release automation fails at first publish due to metadata/versioning misconfiguration | Med | High | Add ADR-5, CP-6 dry-run workflow, package checklist, and pre-publish validation gate |
| Breaking/reset impact if an early bootstrap shortcut reintroduces workspace/crate splitting, weakens the MVP parity harness, or bakes in perpetual upstream-lock assumptions | Med | High | Reject shortcut in G1/G2; if already taken, reset before CP-3 rather than carry architecture debt or false parity obligations into MVP |

## Recommended Next Routing
1. architect for ADR-1 and ADR-4 together: single-package module boundaries, shared-core ownership, unified `todos-lsp` command model, and `serve` LSP transport contract
2. policy/architecture lead for ADR-2: clean-room implementation constraints and contribution review posture
3. planner/test strategy specialist for ADR-3: parity matrix structure, fixture provenance rules, and behavioral comparison harness design
4. release/infra specialist for ADR-5: release-please + Cargo/crates.io publication workflow architecture for the single `todos-lsp` package
5. implementation planner only after ADR-1 through ADR-5 approvals are recorded

## Questions For User
- None.
