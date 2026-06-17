# Bootstrap MVP Release Readiness — `todos-lsp` 0.2.0

## Status

Release coordination only. Do **not** publish, tag, deploy, or create a release PR until the user gives final explicit confirmation.

## Semantic Version / release-please Dry-Run Result

- Current manifest baseline: `.release-please-manifest.json` records `.` at `0.1.0`.
- Current package version before release PR: `Cargo.toml` records `0.1.0`.
- release-please dry-run result: would open `chore(main): release todos-lsp 0.2.0`.
- Semantic release type: **minor** (`0.2.0`) because the included MVP work contains a `feat:` bootstrap milestone commit and is backward-compatible for the single-package Rust release path.

## Commits Included Since Manifest Baseline

Baseline commit on `main`: `403d80a baseline`.

Included release commits:

1. `5e12624 feat: bootstrap todos-lsp mvp`
   - Bootstrap standalone Rust `todos-lsp` package.
   - Adds parity-oriented CLI surface targeting `ianlewis/todos` v0.14.0 for the MVP only.
   - Adds additive `todos-lsp serve` Language Server Protocol entrypoint.
   - Preserves clean-room implementation posture and single-package MVP architecture.
2. `cdf275a fix: resolve review blockers`
   - Resolves review, QA, and hardening blockers identified before release coordination.
   - Confirms release-please dry-run succeeds for `todos-lsp 0.2.0`.

## Validation Evidence Summary

Evidence recorded in `.opencode/session/context.md` and project parity documentation:

- Build phase completed with implementation and review fixes pushed to `main` at `5e12624` and `cdf275a`.
- Parity validation completed for the MVP gate.
- `docs/parity/matrix.md` records 13 MVP CLI/LSP parity rows passing and **0 blocked rows**.
- Packaging validation completed, including `cargo package` and `cargo publish --dry-run` evidence.
- release-please dry-run completed and would prepare `todos-lsp 0.2.0`.
- Code re-review: **PASS**.
- QA re-review: **RELEASE READY** after README/operator documentation fix.
- Security review: no critical stop; requested hardening implemented.

## Required Secrets / Permissions Before Actual Publish

Before actual release/publish, confirm:

- crates.io ownership/publish permission for crate name `todos-lsp`.
- `CARGO_REGISTRY_TOKEN` configured in the publishing environment, preferably GitHub Actions secrets, not passed on the command line.
- GitHub release-please workflow has sufficient token permissions:
  - `contents: write` for tags/releases/version commits.
  - `pull-requests: write` for release PR creation/update.
- Publish workflow is restricted to release/tag events created by release-please, not arbitrary branch pushes.
- CI has access to the pinned upstream `ianlewis/todos` v0.14.0 binary for parity validation, via `.tmp/todos-upstream` or `UPSTREAM_TODOS_BIN` as appropriate.
- Maintainers who can merge the release PR and trigger publication are explicitly approved for this release.

## Deployment / Publish Checklist

### Already Evidenced

- [x] MVP scope approved in session context.
- [x] Single `todos-lsp` package release architecture approved.
- [x] Clean-room / MVP-only parity framing preserved.
- [x] Main implementation commit included: `5e12624`.
- [x] Review-blocker fix commit included: `cdf275a`.
- [x] Parity evidence completed with 0 blocked rows.
- [x] Code review passed.
- [x] QA review marked release ready.
- [x] Security review found no critical stop.
- [x] release-please dry-run would open `chore(main): release todos-lsp 0.2.0`.
- [x] Package dry-run evidence completed.

### Required Before Publish

- [ ] Obtain final explicit user confirmation to release/publish.
- [ ] Create or allow release-please to create the release PR for `todos-lsp 0.2.0`.
- [ ] Review generated release PR diff, especially `Cargo.toml`, `Cargo.lock`, `.release-please-manifest.json`, and `CHANGELOG.md`.
- [ ] Confirm generated changelog mentions only approved MVP work.
- [ ] Re-run CI on the release PR, including formatting, clippy, tests, parity tests, `cargo package`, and `cargo publish --dry-run`.
- [ ] Confirm `CARGO_REGISTRY_TOKEN` is present only in the secure publishing environment.
- [ ] Merge release PR only after approvals and green CI.
- [ ] Confirm release-please creates tag `v0.2.0` and GitHub release metadata.
- [ ] Confirm publish workflow publishes the single crate `todos-lsp` to crates.io.
- [ ] Smoke-test installation after publication: `cargo install todos-lsp --version 0.2.0`.
- [ ] Smoke-test CLI and LSP entrypoints: `todos-lsp --help` and `todos-lsp serve` startup behavior.

## Rollback / Yank Guidance

### Before Release PR Merge

1. Close or update the release PR.
2. Push corrective commits to `main`.
3. Re-run release-please dry-run and validation.
4. Do not create tags or publish crates.io artifacts.

### After Tag/GitHub Release, Before crates.io Publish

1. Stop the publish workflow if still running.
2. Delete or mark the GitHub release as draft/pre-release if it was created incorrectly.
3. Delete tag `v0.2.0` only if no downstream publication or external consumption occurred.
4. Push corrective commits and let release-please regenerate the release flow.

### After crates.io Publish

1. Treat recovery as forward-fix by default; published crate contents are immutable.
2. If the release must be withdrawn from normal dependency resolution, yank it:
   ```sh
   cargo yank todos-lsp --vers 0.2.0
   ```
3. Announce the yank in GitHub release notes and maintainer channels.
4. Fix source on `main` and cut a new patch release through release-please.
5. Verify service/user recovery by confirming the replacement version installs and the yanked version is no longer selected for new resolutions.

## Final Release Notes Guidance

### Internal Notes

- This is the completed bootstrap MVP release for `todos-lsp`.
- The release ships one Rust package and one binary, `todos-lsp`.
- CLI behavior is scoped to MVP parity with `ianlewis/todos` v0.14.0 only.
- LSP support is additive via `todos-lsp serve`.
- Clean-room posture remains mandatory: do not imply copied implementation, fixture, regex, language table, or upstream endorsement.
- Post-MVP development is independent and should not promise ongoing upstream feature-for-feature parity.

### External Notes Draft

`todos-lsp 0.2.0` is the bootstrap MVP release of a clean-room Rust TODO scanner with both CLI and Language Server Protocol interfaces. The CLI targets the approved MVP parity surface for `ianlewis/todos` v0.14.0, while the additive `todos-lsp serve` entrypoint provides editor/LSP integration paths using the same internal finding model. This release establishes the single-package crates.io publishing flow, release-please versioning, clean-room parity documentation, and validation gates for future independent development.

### Confirmation Gate

Actual release and publish require this explicit user confirmation:

> Confirm release and publish `todos-lsp 0.2.0`.

Until that confirmation is received, no release PR, tag, GitHub release, deployment, or crates.io publication should be created.
