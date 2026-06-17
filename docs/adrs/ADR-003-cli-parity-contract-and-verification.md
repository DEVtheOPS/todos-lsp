# ADR-003: CLI Parity Contract and Verification

## Status
Proposed

## Context
`todos-lsp` MVP must be a clean-room Rust rewrite that existing `ianlewis/todos` users can adopt for CLI use without behavioral surprises, while also living inside the approved unified `todos-lsp` binary alongside `todos-lsp serve`. The project is explicitly pinned to `ianlewis/todos` v0.14.0 for MVP parity only, must keep CLI and LSP on the same core finding model, and must prove parity through reviewable evidence rather than approximation.

The team therefore needs a precise contract for what “100% behavioral feature parity” means, which dimensions are binding for MVP, what evidence closes the gate, and where parity stops so post-MVP product evolution remains free.

## Options Considered
| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| A. Exact byte-for-byte parity for every CLI artifact, including help prose and all human-facing text | Maximum apparent sameness; simple slogan | Conflicts with clean-room constraints; overfits implementation to upstream wording; treats generated/help prose as product contract; unnecessary lock-in | ✗ |
| B. Behavioral parity contract pinned to v0.14.0 with explicit dimensions, matrix coverage, clean-room fixtures, and automated comparison evidence | Rigorous and measurable; consistent with clean-room rewrite; preserves MVP adoption goal; keeps post-MVP divergence possible | Requires careful matrix authoring, observation notes, and a substantial fixture corpus; ambiguous quirks still need escalation | ✓ |
| C. Best-effort parity judged by maintainers without a formal matrix or comparison harness | Fastest path to implementation | Unreviewable; weak release gate; encourages “close enough”; likely to miss edge cases and undermine MVP claim | ✗ |

## Decision
Adopt a strict MVP-only behavioral parity contract against `ianlewis/todos` v0.14.0.

For MVP, “100% behavioral feature parity” means: every user-visible CLI capability in the pinned upstream release that is within the approved MVP CLI scope must be represented in the parity matrix, implemented on the unified `todos-lsp` command surface, and shown by approved evidence to match the upstream reference behavior for the same authored inputs across all in-scope dimensions.

This is a behavioral contract, not a source or prose-copying contract. `todos-lsp` may reorganize the entrypoint into parity-oriented subcommands under the single `todos-lsp` binary, but the resulting user-observable semantics must match the pinned upstream behavior for the covered feature. Clean-room constraints from ADR-002 govern all fixture, parser, config, and documentation authoring.

### Architecture Overview
```text
ianlewis/todos v0.14.0 (black-box behavioral oracle)
              |
              v
      parity observation notes
              |
              v
        parity matrix
   (feature -> dimensions -> evidence)
              |
              v
 clean-room fixture corpus + comparison harness
              |
      +-------+--------+
      |                |
      v                v
 todos-lsp CLI    shared todo-core
      |
      v
 MVP parity sign-off gate
```

### Data Flow
1. Observe `ianlewis/todos` v0.14.0 only through allowed clean-room channels.
2. Record original parity notes describing command behavior, flags, outputs, exit semantics, discovery behavior, parsing behavior, config interactions, and error modes.
3. Author original fixtures that isolate those behaviors.
4. Run the pinned upstream binary and `todos-lsp` against the same authored corpus and invocation set.
5. Compare outputs, exit codes, and behavior records through an automated harness plus approved manual evidence where automation is insufficient.
6. Mark a parity row complete only when all required evidence exists and any ambiguity is resolved or explicitly escalated.

### Key Interfaces
- **Parity baseline**
  - Binding reference: `ianlewis/todos` v0.14.0 only.
  - Binding duration: MVP / initial release only.
- **Parity claim boundary**
  - Applies to the CLI behavior surface only.
  - LSP behavior remains additive and is governed by ADR-004.
- **Shared-core boundary**
  - CLI parity findings must come from the same `todo-core` finding model consumed by the LSP.
- **Verification boundary**
  - Parity is declared by matrix + fixture + harness evidence, not by intent or informal review.

## Consequences

### Benefits
- Makes the MVP parity promise measurable, reviewable, and release-gated.
- Preserves the clean-room posture while still allowing rigorous black-box comparison.
- Prevents “close enough” interpretations of parity.
- Keeps post-MVP evolution explicitly decoupled from perpetual upstream lockstep.

### Tradeoffs
- The team must build and maintain a more disciplined parity matrix and fixture corpus before claiming MVP completion.
- Some observable upstream quirks may require escalation instead of convenient imitation.
- The unified `todos-lsp` command surface must sometimes map upstream behavior rather than mirror upstream entrypoint shape literally.

### Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Teams treat parity as approximate instead of exhaustive | Medium | High | Require complete matrix coverage and gate MVP sign-off on evidence for every row |
| Clean-room fixture authoring drifts too close to upstream artifacts | Medium | High | Enforce ADR-002 provenance rules and reject derivative fixtures/tests |
| Hidden upstream edge cases emerge late | High | High | Build the matrix early, record ambiguity notes, and escalate undocumented quirks before broad implementation |
| Developers infer ongoing parity obligations after MVP | Medium | High | State MVP-only binding in ADR, matrix, milestone gates, and release notes |
| Early implementation shortcuts bypass shared core for CLI-only behavior | Medium | High | Require parity and dual-adapter consistency evidence over the shared finding model |

## Implementation Notes
- **Definition of MVP parity completeness**
  - “100% behavioral feature parity” for MVP means all rows in the approved parity matrix are in one of only two states:
    - `Pass`: implemented and evidenced as matching v0.14.0 behavior for the row’s dimensions.
    - `Approved N/A`: not part of the upstream v0.14.0 surface or not applicable because of the approved unified-binary remapping, with rationale documented.
  - No row may remain `Partial`, `Close`, `Deferred`, or `Known Difference` at MVP release sign-off.

- **In-scope CLI parity dimensions**
  - commands and subcommands:
    - supported operations from v0.14.0 must be mapped onto the unified `todos-lsp` CLI surface;
    - invocation shape, required positionals, accepted aliases, and invalid-command behavior must be documented.
  - flags and arguments:
    - supported flags, arity, defaults, value parsing, repeatability, incompatibilities, and error behavior are in scope.
  - outputs:
    - machine-consumable outputs and stable human-facing command outputs are in scope for content, ordering, formatting semantics, stdout/stderr channel, and trailing newline behavior where user-visible;
    - generated help prose and wording are not required to be byte-identical, but command availability, flag presence, usage structure, and help exit behavior are in scope.
  - exit codes:
    - success, findings-present behavior, usage/config/runtime failures, unsupported-input failures, and dependency-related failures are in scope.
  - file discovery and ignore rules:
    - recursive traversal scope, include/exclude behavior, ignore-file/config interaction, default discovery roots, and direct-path handling are in scope.
  - symlink behavior:
    - explicit symlink path handling and traversal behavior must match the pinned upstream release.
  - unsupported-file handling:
    - direct unsupported-path invocation behavior, suppression behavior if applicable, message routing, and exit semantics are in scope.
  - blame behavior:
    - enablement, dependency requirements, emitted metadata, failure behavior when dependencies are unavailable, and config/flag interaction are in scope where v0.14.0 supports blame.
  - parsing behavior:
    - marker recognition, comment-awareness, false-positive avoidance, default todo-type set, label/message extraction, source locations, and supported-language behavior exposed by upstream are in scope as observed behavior.
  - config interactions:
    - config discovery, precedence, defaults, invalid-config failures, flag-overrides-config behavior, and parity-relevant settings interactions are in scope.

- **Parity matrix expectations**
  - The matrix must be a project-owned artifact, not a copied upstream table.
  - Each row must represent one upstream CLI capability or edge-case contract.
  - Minimum columns:
    - matrix ID
    - upstream capability/category
    - `todos-lsp` command mapping
    - in-scope dimensions covered
    - authored fixture IDs
    - invocation(s)
    - expected upstream-observed behavior summary
    - automated evidence reference
    - manual evidence reference if needed
    - status (`Pass`, `Approved N/A`, `Blocked` before completion only)
    - ambiguity/provenance notes
  - The matrix must cover at least these sections:
    - command surface and invalid invocation handling
    - flags and argument parsing
    - output modes and formatting contracts
    - exit semantics
    - discovery / ignore / symlink rules
    - unsupported-file behavior
    - blame modes and dependency failures
    - parsing and finding normalization behavior
    - config discovery and precedence behavior

- **Evidence required to declare parity complete**
  - Approved parity matrix with every MVP row closed as `Pass` or `Approved N/A`.
  - Original fixture inventory with provenance notes per ADR-002.
  - Automated comparison harness results showing `todos-lsp` and `ianlewis/todos` v0.14.0 were run against the same fixture corpus and invocation matrix.
  - Captured evidence for:
    - stdout/stderr comparisons for in-scope outputs;
    - exit-code comparisons;
    - discovery/ignore/symlink/unsupported-file cases;
    - blame-on and blame-failure cases if supported;
    - parsing/config interaction cases.
  - Disposition log for every failed or ambiguous comparison encountered during development, showing resolution before sign-off.
  - Dual-adapter consistency evidence that CLI parity findings are produced from the same core finding model used by the LSP.
  - Lead review approval of the final parity evidence gate.

- **Allowed verification methods**
  - Execute the pinned upstream binary and `todos-lsp` against the same clean-room corpus.
  - Capture and compare exit codes, stdout, stderr, discovered finding sets, and relevant filesystem-side effects.
  - Use manual review only for cases that are awkward to automate fully, such as help-structure checks or ambiguity adjudication, and record that evidence in the matrix.
  - When exact text equality is not the correct contract because of clean-room wording restrictions, compare the documented behavioral contract instead of forcing prose duplication.

- **Fixture strategy consistent with clean-room constraints**
  - All fixtures must be newly authored for `todos-lsp`.
  - Do not copy or lightly rewrite upstream fixtures, tests, README examples, regexes, language tables, ignore tables, or sample outputs.
  - Prefer a layered corpus:
    - minimal unit-style fixtures isolating one behavior each;
    - focused edge-case fixtures for discovery, ignore, symlink, unsupported-file, blame, and config precedence scenarios;
    - a small number of realistic integration corpora combining multiple behaviors.
  - Fixture metadata should record:
    - author/provenance note;
    - behavior category;
    - relevant matrix rows;
    - required environment assumptions, such as a git repository for blame cases.
  - Generated expected results may be captured from upstream execution on the authored corpus, but the stored artifact must be project-owned parity evidence, not an imported upstream test.

- **Explicitly out of scope for MVP parity**
  - Any upstream release after `ianlewis/todos` v0.14.0.
  - Perpetual or rolling parity maintenance after MVP sign-off.
  - LSP protocol behavior, beyond the requirement that CLI and LSP share the same core findings model.
  - Byte-identical copying of help prose, documentation wording, or other source-derived text.
  - New `todos-lsp`-specific CLI enhancements that expand or alter the parity surface before MVP parity is complete.
  - Non-user-visible internal implementation details of upstream.

- **Post-MVP divergence rule**
  - Once MVP parity is accepted, upstream parity becomes historical baseline evidence, not ongoing governance.
  - Post-MVP CLI changes may diverge from `ianlewis/todos` by default if they are justified by `todos-lsp` product goals and approved through normal design/spec review.
  - Future divergence may include command reshaping, new outputs, changed defaults, expanded config, or reduced compatibility requirements, provided such changes are documented as `todos-lsp` decisions rather than parity regressions.
  - If a future phase wants renewed parity obligations against upstream or against a newer upstream version, that must be explicitly re-chartered in a later spec/ADR.

- **Breaking/reset guidance**
  - If prototype work weakened the parity gate, used derivative fixtures, or encoded unreviewed “known differences,” reset that work before implementation continues. Slightly inconvenient, yes. Still correct.

- [CROSS-TEAM REVIEW]: lead approval is required for the final MVP parity matrix and sign-off evidence gate.
- [CROSS-TEAM IMPACT]: any request to extend this parity contract beyond MVP, or to reproduce undocumented upstream quirks despite clean-room risk, requires explicit lead re-chartering.

## Questions for User
- None.
