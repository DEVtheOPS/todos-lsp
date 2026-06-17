# ADR-005: Release-Please and Cargo Publishing Architecture

## Status
Proposed

## Context
`todos-lsp` needs a release architecture that matches the approved MVP product shape: one project, one user-facing binary, GitHub Actions automation, release-please-managed versioning/changelog flow, and Cargo/crates.io publication. The release design must also align with the tightened MVP architecture direction recorded in session context: one publishable Rust package named `todos-lsp`, with internal module boundaries for shared core, CLI, and LSP concerns kept inside that package and not published as separate crates during MVP.

The main forces are:
- the project must use release-please for MVP rather than alternate release tooling;
- Cargo/crates.io publishing is required, but only for the top-level `todos-lsp` package;
- the installed command surface must remain a single binary, `todos-lsp`, with `todos-lsp serve` as the LSP entrypoint;
- internal core/CLI/LSP boundaries must remain real in code structure, but unpublished in package/release terms for MVP;
- the first public publish must be guarded by strict dry-run and verification gates;
- publish automation must fail closed because crates.io publishes are irreversible per version once accepted;
- crate extraction and Rust workspace publishing are explicitly deferred until post-MVP if later justified.

This resolves the prior packaging blocker cleanly. A published Cargo package cannot depend on unpublished path-only crates, so the proper MVP design is not “publish all internal crates anyway,” but rather “do not split the package yet.” Occasionally the least theatrical architecture is also the correct one.

## Options Considered
| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| A. Single-package MVP release architecture: release-please + GitHub Actions + crates.io publishing only for top-level `todos-lsp`, with internal module boundaries kept unpublished | Fully compatible with Cargo/crates.io rules; preserves approved release tooling; preserves single-binary product story; removes prior packaging blocker; keeps future extraction optional | Defers crate/workspace extraction work until post-MVP; less package-level isolation than a true multi-crate workspace | ✓ |
| B. Multi-crate workspace with lockstep publishing of all dependency crates | Compatible with Cargo/crates.io rules | Reintroduces the MVP packaging conflict; publishes internal implementation crates prematurely; adds avoidable release complexity; contradicts approved tightened MVP direction | ✗ |
| C. Publish only `todos-lsp` while keeping internal crates unpublished path dependencies | Preserves hard unpublished crate boundaries | Not compatible with Cargo/crates.io for a published crate that depends on unpublished path-only crates | ✗ |
| D. Custom release bundling that flattens a workspace into one publishable package at release time | Could preserve source-level crate split while publishing one package | Adds bespoke packaging logic, drift risk, and avoidable release fragility; unjustified for MVP | ✗ |
| E. Replace release-please with Rust-specific tooling such as `cargo-release` or `release-plz` | Strong Rust ergonomics | Violates approved MVP requirement; changes release governance model; reopens an already-settled tooling decision | ✗ |

## Decision
Adopt a single-package MVP release architecture for `todos-lsp`.

This means:
- release-please owns semantic version calculation, changelog generation, release PR creation, tagging, and GitHub release metadata;
- GitHub Actions owns CI validation, release dry-runs, and tagged publish execution;
- Cargo/crates.io publishing applies only to the top-level `todos-lsp` package;
- shared core, CLI adapter, and LSP adapter boundaries remain internal Rust modules within the `todos-lsp` package for MVP;
- no Rust workspace publishing model is used for MVP;
- any future extraction into separate crates or a workspace is explicitly deferred until post-MVP and requires a later ADR/spec decision.

### Architecture Overview
```text
Conventional commits on main
          |
          v
+-------------------------------+
| release-please                |
| - computes next version       |
| - updates Cargo.toml          |
| - updates changelog/release   |
| - opens/updates release PR    |
+---------------+---------------+
                |
         merge release PR
                |
                v
+-------------------------------+
| GitHub release/tag created    |
| tag: vX.Y.Z                   |
+---------------+---------------+
                |
                v
+-------------------------------+
| publish workflow              |
| - checkout tagged source      |
| - verify package integrity    |
| - cargo publish todos-lsp     |
+-------------------------------+

Inside package `todos-lsp`
+---------------------------------------------------+
| src/                                              |
|   core/   - shared scanning/model logic           |
|   cli/    - parity-oriented command handling      |
|   lsp/    - serve mode + protocol integration     |
|   main.rs - single binary entrypoint              |
+---------------------------------------------------+
```

### Data Flow
1. Developers merge conventional-commit-formatted changes to `main`.
2. A `release-please` GitHub Actions workflow runs on `main` and updates or creates the `todos-lsp` release PR.
3. The release PR contains:
   - the next package version for `todos-lsp`,
   - changelog updates,
   - any release metadata updates required for the single package.
4. Maintainers review and merge the release PR.
5. On merge, release-please creates the project tag (`vX.Y.Z`) and GitHub release.
6. A publish workflow triggered from the release/tag checks out the exact tagged revision, performs final verification, and publishes the `todos-lsp` package to crates.io.
7. After successful registry publication, the GitHub release is the authoritative public release record for that version.

### Key Interfaces
- `release-please` configuration
  - Use a single Rust package release configuration for the repository.
  - Maintain one project tag format: `vX.Y.Z`.
  - Generate one release PR and one GitHub release stream for `todos-lsp`.
  - Do not configure manifest-linked multi-crate publishing for MVP.

- Package/publication boundary
  - `todos-lsp`: the only publishable Cargo package for MVP and the only supported install target.
  - `src::core`: internal shared logic boundary; not a published crate.
  - `src::cli`: internal CLI boundary; not a published crate.
  - `src::lsp`: internal LSP boundary; not a published crate.
  - These boundaries are architectural and testability concerns inside the package, not separate release artifacts during MVP.

- GitHub Actions responsibilities
  - `ci.yml`
    - run formatting, linting, unit/integration tests, parity tests, and package build checks on PRs/pushes.
  - `release-please.yml`
    - run release-please on `main` to maintain the release PR and create tags/releases after merge.
  - `release-validate.yml`
    - run release dry-runs and package validation without publishing.
  - `publish.yml`
    - on release/tag, verify the tagged source and publish `todos-lsp`.

## Consequences

### Benefits
- Preserves the approved GitHub Actions + release-please + Cargo/crates.io toolchain.
- Aligns the release flow with the approved single-package MVP architecture.
- Removes the prior packaging blocker without custom release transforms.
- Preserves the single `todos-lsp` binary product story.
- Keeps internal core/CLI/LSP boundaries intact in code while avoiding premature public crate surfaces.
- Defers irreversible packaging complexity until there is post-MVP evidence that extraction is worthwhile.

### Tradeoffs
- Internal boundaries are enforced by modules and review discipline rather than separate published crates during MVP.
- Future crate/workspace extraction will require deliberate refactoring if the project later needs reusable libraries or independently versioned components.
- The release pipeline is simpler now precisely because package-level separation is deferred, not solved forever.

### Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Internal module boundaries erode because they are not separate crates yet | Medium | Medium | Keep explicit module ownership, APIs, and tests; review architecture drift during implementation checkpoints |
| First public release exposes package metadata or publish configuration issues | Medium | High | Gate first publish on `cargo package`, `cargo publish --dry-run`, install smoke tests, and release-please dry-run evidence |
| Team later needs reusable/public subcrates and must refactor after MVP | Medium | Medium | Treat extraction as an explicit post-MVP architecture step with its own ADR rather than prematurely publishing unstable internals |
| Tag/release is created but publish fails before crates.io upload completes | Low | Medium | Allow safe rerun of publish workflow from same tag when failure is transient or credentials-related |
| Pressure returns to introduce ad hoc workspace or publish hacks before MVP is complete | Medium | High | Keep ADR-001/ADR-005 aligned on single-package MVP; reject interim packaging shortcuts that reintroduce the blocker |

## Implementation Notes
- Use release-please in a single-package Rust configuration appropriate for the top-level `todos-lsp` package.
- Keep one tag namespace for the project: `vX.Y.Z`.
- Changelog and GitHub release notes should describe the unified `todos-lsp` project release.
- Package metadata must support public publication for `todos-lsp` without manual manifest rewriting at release time.
- Organize internal code so shared scanning logic, CLI parity logic, and LSP logic remain cleanly separated inside the package.
- Do not introduce a Cargo workspace publishing model for MVP.
- If post-MVP needs justify extraction, do it as a conscious reset with updated release architecture rather than half-introducing it during bootstrap.

### GitHub Actions Responsibilities
- **CI workflow**
  - `cargo fmt --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test --all-targets`
  - parity/integration validation required by the MVP gate
  - package metadata sanity checks as soon as manifests exist
- **Release validation workflow**
  - run release-please in dry-run mode against the repository state;
  - verify generated version/changelog outputs are structurally correct;
  - run `cargo package` for `todos-lsp`;
  - run `cargo publish --dry-run` for `todos-lsp` from a clean checkout;
  - perform install smoke tests against the packaged `todos-lsp` artifact;
  - archive dry-run logs as release evidence for G6.
- **Release workflow**
  - maintain the release PR on every merge to `main`;
  - create/update GitHub release metadata only through release-please.
- **Publish workflow**
  - trigger only from release/tag events created by release-please;
  - require crates.io token and least-privilege permissions;
  - publish only `todos-lsp`;
  - stop immediately on first failed publish step;
  - never attempt ad hoc manifest rewriting during publish.

### Dry-Run and First-Publish Verification Requirements
Before the first public publish is allowed:
1. Release-please dry-run must show the expected single project release PR, version, and changelog output.
2. `cargo package` must succeed for `todos-lsp` from a clean checkout.
3. `cargo publish --dry-run` must succeed for `todos-lsp`.
4. A smoke install of the packaged `todos-lsp` artifact must succeed and run `todos-lsp --help` plus `todos-lsp serve --help`.
5. The tagged-release publish workflow must be rehearsed in a non-publishing mode and the logs preserved as milestone evidence.
6. Crates.io ownership, token storage, and maintainer access must be confirmed before merge of the first release PR. [CROSS-TEAM NEED]

### Failure Behavior and Rollback Expectations
- Release PR generation is reversible; update or replace commits and let release-please refresh the PR.
- Tag creation and GitHub release creation are automatable and can be deleted/recreated before any successful crates.io publish if absolutely necessary.
- crates.io publishes are not practically reversible. A published version must be treated as immutable.
- If failure occurs before successful package publication, fix the pipeline or credentials issue and rerun the publish workflow for the same tag.
- If failure occurs after `todos-lsp` is published, recovery is forward-only: fix the issue in source and cut a new patch release through release-please.
- Because Cargo publish is immutable, dry-run validation is mandatory and not negotiable for MVP.

### Why Alternate Tooling Is Rejected for MVP
- `cargo-release` is rejected because it changes the release control model from release-please's PR-centric GitHub workflow to a Cargo-driven release command flow.
- `release-plz` is rejected because it would replace an already approved release governance requirement rather than solving a gap in the approved stack.
- custom scripts/semantic-release hybrids are rejected because they add novel failure modes and reduce auditability where the project needs boring, reviewable automation.
- In short: release-please is already a project requirement, sufficiently capable for the MVP, and the architecture should adapt to that choice rather than reopening a tooling debate.

## Questions for User
- None.
