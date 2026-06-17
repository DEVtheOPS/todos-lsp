# Spec: todos-lsp Bootstrap Scope

## Problem
The project needs a standalone Rust implementation that can replace `ianlewis/todos` for CLI use while also adding a first-party LSP server behind the same installed binary. The MVP is needed now so the project can deliver a clean-room, remote-friendly Rust binary named `todos-lsp` with one shared scanning core, match established `todos` behavior for repository scanning through MVP-only parity subcommands, and add editor-native diagnostics and symbol support via `todos-lsp serve` without depending on upstream Go internals. After MVP parity is achieved, the product must be free to evolve independently rather than remain constrained by long-term upstream lockstep.

## Success Criteria
- `todos-lsp` v1/MVP achieves 100% documented feature parity with `ianlewis/todos` **behavior** for the pinned upstream reference version `v0.14.0`, except where this spec explicitly adds LSP-only behavior.
- The parity requirement is explicitly scoped to the initial release/MVP as a bootstrap adoption target, not as an ongoing commitment to future upstream lockstep.
- A written parity matrix exists that maps every upstream user-visible feature, flag, output mode, exit behavior, file-discovery rule, ignore rule, and error mode in the pinned reference version to an implemented Rust equivalent or to the one approved additive LSP extension.
- CLI parity is proven by fixture-based behavioral tests that compare Rust output and exit behavior against the pinned upstream reference on the same clean-room test corpus.
- A single publishable Rust package named `todos-lsp` exists for MVP, with internal module boundaries for shared core, CLI, and LSP concerns that ship a unified user-facing binary named `todos-lsp`.
- CLI and LSP use the same core finding model and produce materially equivalent findings for the same content and configuration.
- The unified `todos-lsp` binary exposes `serve` as the LSP server entrypoint and MVP-only parity-oriented subcommands for the remaining initial CLI scope.
- The LSP server adds diagnostics, `workspace/symbol`, and an explicit server command surface beyond upstream CLI parity.
- Release automation uses GitHub Actions plus release-please to version, tag, generate changelog/release metadata, and publish the `todos-lsp` Rust package to Cargo/crates.io.
- Clean-room rewrite constraints and MPL-2.0 posture are documented and enforced throughout implementation and test authoring.
- Post-MVP roadmap decisions may use the MVP parity baseline as historical context, but must not be blocked by a requirement to preserve ongoing upstream feature parity unless a future spec says otherwise.

## User Stories
- As an existing `ianlewis/todos` user, I want the `todos-lsp` CLI subcommands to behave the same way for supported MVP/v1 features so that I can switch tools without relearning semantics during initial adoption.
- As a maintainer, I want upstream parity treated as a bootstrap target rather than a permanent product constraint so that the project can evolve independently after the initial release.
- As a developer, I want to scan a repository from the CLI so that I can find actionable TODO-style items in code and config files.
- As an editor user, I want TODO diagnostics and workspace symbol results from a native LSP server so that unsaved changes are reflected immediately.
- As a remote development user, I want the language server to run from a separately installed `todos-lsp` binary on the remote host so that editor integrations work without bundling the server.
- As a maintainer, I want release-please-driven Cargo publishing so that releases are repeatable, auditable, and compatible with the stable GitHub Actions ecosystem.

## Requirements

### Must Have (P0)
- Standalone single publishable Rust package for MVP with internal module boundaries for shared core, CLI, and LSP responsibilities — Acceptance: repository builds on stable Rust, the package structure keeps `core`, `cli`, and `lsp` concerns separate inside `todos-lsp`, and user-facing installation exposes a single binary named `todos-lsp`.
- Pinned upstream parity target — Acceptance: v1/MVP parity scope is explicitly pinned to `ianlewis/todos` `v0.14.0`, applies to the initial release only, and does not float with future upstream releases.
- Unified MVP command surface — Acceptance: the installed binary name is `todos-lsp`, the LSP server entrypoint is `todos-lsp serve`, and all remaining MVP CLI commands are exposed as parity-oriented subcommands on that same binary rather than as a separate CLI executable or separate LSP executable.
- Complete CLI behavior parity with pinned upstream — Acceptance: every user-visible CLI capability in the pinned upstream release is implemented with matching behavior through the unified `todos-lsp` command surface, including scanning scope, TODO parsing, default todo-type set, output modes, exit codes, unsupported-file handling, symlink behavior, ignore handling, and blame behavior where present upstream.
- Explicit parity verification suite — Acceptance: the repo contains a parity matrix and automated fixture coverage demonstrating that Rust results match the pinned upstream reference for representative cases across supported features.
- Post-MVP independence preserved — Acceptance: spec language and acceptance criteria do not imply a standing requirement for perpetual parity with `ianlewis/todos` after MVP sign-off; future divergence is allowed by default unless a later approved spec says otherwise.
- Shared scanning domain in internal core modules independent from transport/adapters — Acceptance: CLI and LSP consume shared core APIs rather than implementing separate scanning logic.
- Workspace scanning from disk — Acceptance: CLI and LSP can scan files in a workspace path and return TODO findings with file location metadata.
- Open-buffer / unsaved-text scanning — Acceptance: LSP can analyze text not yet written to disk and publish findings without requiring a save.
- LSP diagnostics — Acceptance: LSP publishes diagnostics for discovered items and updates them when files or buffers change.
- LSP `workspace/symbol` support — Acceptance: LSP returns TODO findings as workspace symbols with enough metadata to navigate to source locations.
- LSP server command addition — Acceptance: the LSP exposes at least one documented server command surface for TODO-related actions beyond upstream CLI parity, and the command operates over the shared core model rather than ad hoc editor logic.
- Consistent core model across CLI and LSP — Acceptance: equivalent inputs produce materially equivalent findings, type classification, parsed label/message fields, and source locations.
- Remote-friendly separately installed binary model — Acceptance: the LSP server can be installed and launched as `todos-lsp serve` from an external binary without requiring editor-bundled assets.
- Fail-closed runtime dependency handling — Acceptance: when a required runtime dependency for a requested feature is unavailable, the tool reports the failure clearly and does not silently degrade into incomplete results.
- Clean-room rewrite posture — Acceptance: project docs and contribution rules state that upstream is a behavioral reference only, and repository artifacts do not copy upstream source, tests, README prose, regexes, fixtures, or language-definition tables verbatim.
- Release-please based release automation — Acceptance: GitHub Actions can run release-please, open/update release PRs, cut version tags, and publish the `todos-lsp` package to Cargo/crates.io from tagged releases.
- Release-ready package metadata — Acceptance: Cargo package metadata, licensing, changelog inputs, versioning strategy, and publish configuration are set so a release-please-generated release can publish `todos-lsp` successfully without manual manifest surgery.

### Should Have (P1)
- Shared configuration loading for CLI and LSP — Acceptance: both interfaces resolve the same config schema and defaults unless the LSP command surface explicitly requires transport-specific options.
- Incremental rescanning behavior for LSP — Acceptance: file/buffer changes trigger targeted updates instead of mandatory full-workspace rescans where practical.
- Stable finding identity — Acceptance: repeated scans of unchanged content yield stable identifiers or locations suitable for editor refreshes.
- Release dry-run validation — Acceptance: CI can validate release-please and Cargo publish steps in a non-publishing mode before the first public release.

### Nice to Have (P2)
- Optional human-readable CLI summaries in addition to parity output modes.
- Additional editor-facing commands beyond the first required LSP server command.
- Optional post-MVP parity tracking or comparison reporting for upstream releases newer than the pinned reference, provided it is framed as informative rather than binding.

## Primary Use Cases
1. CLI repository scan: user runs `todos-lsp <parity-subcommand>` on a workspace path with parity-compatible flags → core scans disk files using upstream-equivalent behavior → CLI emits parity-compatible results and exit status.
2. Open file diagnostics: editor launches `todos-lsp serve` and sends current buffer contents → core scans unsaved text with document context → LSP publishes diagnostics.
3. Workspace symbol search: editor requests `workspace/symbol` → LSP queries indexed findings from core → editor receives navigable TODO symbols.
4. LSP command invocation: editor invokes the documented server command → LSP resolves the request via shared core/index state → editor receives deterministic TODO-related results or action outcomes.
5. Release flow: maintainer merges release-please PR → GitHub Actions tags the release and publishes the `todos-lsp` package to Cargo/crates.io → users can install the released Rust artifact.
6. Post-MVP direction flow: maintainers evaluate new product work after parity sign-off → roadmap decisions follow `todos-lsp` product goals and approved specs → upstream behavior may inform decisions but does not constrain them by default.

## User Flows
1. CLI parity flow: user runs `todos-lsp` with a supported MVP parity subcommand and upstream-compatible flag set → core scans eligible inputs using pinned parity rules → CLI returns matching findings/output/exit behavior.
2. Unsaved buffer flow: open/edit file in editor configured to launch `todos-lsp serve` → LSP receives document text → core rescans buffer → diagnostics refresh without disk write.
3. Symbol navigation flow: user searches workspace symbols → LSP returns TODO items from current workspace state → editor jumps to selected item.
4. Release flow: conventional changes merge to main → release-please updates version/changelog state → maintainer merges release PR → CI publishes Cargo artifacts.

## Edge Cases
- Unsupported file passed directly on the command line: behavior must match the pinned upstream release, including exit semantics and any approved suppressing flag behavior.
- Symlinked paths: traversal and explicit-path handling must match the pinned upstream release rather than earlier upstream behavior.
- Marker text inside strings or code for supported languages: do not report as a TODO if the pinned upstream behavior would not report it.
- Unsaved buffer differs from disk: LSP must prefer buffer content for that document over on-disk content.
- Large workspace: system should remain responsive; LSP operations must avoid blocking indefinitely on full rescans.
- Missing required runtime dependency for a requested feature such as blame: return explicit error and incomplete feature refusal rather than silent partial success.
- Upstream behavior discovered to be undocumented but observable during MVP parity work: document the parity decision in the parity matrix and validate it with clean-room fixtures rather than copying upstream tests.
- Post-MVP feature proposal intentionally diverges from `ianlewis/todos`: treat it as allowed by default once MVP parity has been accepted, and evaluate it against `todos-lsp` goals rather than upstream lockstep.
- Cargo publish misconfiguration: release pipeline must fail before publishing partial or malformed artifacts.

## Technical Constraints
- Rust stable current-generation toolchain only; GitHub Actions and release-please must use current stable ecosystem components.
- Shared internal core modules must remain independent of CLI/LSP adapters.
- No dependency on private Go internals from upstream projects.
- No shell-out-first or shell-out-primary architecture.
- Parity target is behavioral, not source-level; implementation must be a clean-room rewrite.
- Upstream parity is an MVP bootstrap constraint only, not a perpetual product-governance model.
- No Rust workspace or split published crates in MVP; workspace/crate extraction is deferred until post-MVP and only if justified by real implementation pain.
- Preserve workspace disk scanning and open-buffer scanning as first-class requirements.
- Preserve consistent CLI/LSP behavior over the same core model.
- Preserve remote-friendly separately installed binary deployment through the single `todos-lsp` executable.
- Preserve MPL-2.0 clean-room rewrite posture.
- Release automation must be compatible with GitHub-hosted workflows and Cargo/crates.io publishing expectations.

## Out of Scope
- Copying upstream code, tests, README text, regexes, or language tables verbatim.
- Depending on upstream Go internals or embedding upstream implementation artifacts.
- Shipping a partial-parity MVP that omits upstream CLI features from the pinned reference version.
- Treating future upstream releases as automatically in-scope for v1/MVP or as binding on the post-MVP roadmap without an explicit spec update.
- Shipping or implying a separate LSP-only binary alongside `todos-lsp` for MVP.
- Introducing a Rust workspace or split published crates during MVP.
- Bundling the LSP server binary inside editor extensions.
- Advanced LSP features beyond diagnostics, `workspace/symbol`, and the required server command addition.
- Alternative release tooling replacing release-please for MVP.
- Silent best-effort degradation when required runtime dependencies are missing.

## Assumptions
- The behavioral parity baseline is `ianlewis/todos` `v0.14.0`, used as the initial-release reference target rather than a permanent upstream roadmap anchor.
- The LSP server command addition is additive scope and does not reduce or relax CLI parity requirements.
- The unified command surface may reorganize upstream command entrypoints into `todos-lsp` subcommands for MVP, but the underlying user-visible behavior must still satisfy the pinned parity target.
- Cargo publishing means publishing the `todos-lsp` package intended for user installation in the Cargo ecosystem, using release-please-managed GitHub releases.
- Internal module seams should be designed so post-MVP extraction into a workspace or separate crates remains possible if implementation pain later justifies it, but that extraction is not part of MVP.
- Clean-room parity verification will use original fixtures and expected-result descriptions created from observed behavior, not copied upstream tests.

## Clean-Room Implementation Implications
- Engineers may inspect upstream binaries, docs, CLI help text, and observed runtime behavior to define parity expectations, but must not port code or transliterate implementation details.
- Upstream materials are reference inputs for MVP parity definition only; after MVP acceptance, future feature planning may consult them but is not required to preserve long-term parity.
- Test fixtures must be authored from scratch, even when inspired by upstream scenarios.
- Language support, parsing rules, and ignore behavior may match upstream behavior, but configuration data and regexes must be independently derived or newly authored.
- When an upstream behavior is ambiguous, the project should prefer independently reasoned behavior plus documented parity notes rather than copying hidden upstream internals.
- Any discovered need to reproduce undocumented quirks exactly should be escalated for lead review because it increases clean-room and maintenance risk. [CROSS-TEAM IMPACT]

## Open Questions
- None at this time for the command surface; the MVP command decision is `todos-lsp` with `serve` as the LSP entrypoint and remaining CLI scope exposed as parity-oriented subcommands.
