# ADR-002: Clean-Room Implementation Rules

## Status
Accepted

## Context
`todos-lsp` is a standalone Rust rewrite that targets MVP behavioral parity with `ianlewis/todos` v0.14.0 for the initial release only, while preserving an MPL-2.0 clean-room posture and long-term product independence. The team needs explicit implementation and review rules so parity work does not drift into source-derived copying.

The core tension is simple enough: we must be strict enough to keep the rewrite defensibly independent, but not so strict that parity becomes hand-wavy or unverifiable. That requires hard boundaries on what may be copied, clear rules for observing upstream behavior, original fixture authoring standards, and an escalation path when exact parity pressure conflicts with clean-room requirements.

## Options Considered
| Option | Pros | Cons | Verdict |
|--------|------|------|---------|
| A. Loose inspiration policy with reviewer discretion | Fastest to start; low process overhead | High legal/provenance risk; inconsistent enforcement; easy regression into copied artifacts; weak auditability | ✗ |
| B. Strict clean-room policy with behavior-only observation, original artifacts, and documented escalation | Enforceable; auditable; consistent with MPL-2.0 rewrite posture; preserves evidence-based parity gate | Slower fixture authoring; may require resetting work if provenance is weak; more review discipline required | ✓ |
| C. Full dual-team clean-room isolation (spec team vs implementation team) | Strongest formal separation | Excessive process for this project stage; slows delivery materially; unnecessary unless risk increases | ✗ |

## Decision
Adopt a strict single-team clean-room policy: upstream `ianlewis/todos` v0.14.0 may be used only as a behavioral and specification reference, never as a source artifact donor. All implementation, tests, fixtures, documentation, parsing patterns, and support tables in `todos-lsp` must be independently authored. Behavioral parity must be proven by observation notes and original parity fixtures, not by porting or transliterating upstream materials.

When parity and clean-room requirements appear to conflict, clean-room rules win by default unless lead review explicitly approves a documented exception path or a parity-scope clarification.

### Architecture Overview
```text
ianlewis/todos v0.14.0
  ├─ allowed inputs: CLI help, docs, binary behavior, observable outputs
  └─ forbidden inputs: source artifacts and derivative copies

Observed behavior/spec notes
  └─ parity matrix + authored scenario descriptions
        └─ original fixtures + comparison harness
              ├─ todos-lsp Rust implementation
              └─ parity evidence for MVP gate
```

### Data Flow
1. Researcher/engineer observes upstream behavior through allowed channels.
2. Observations are rewritten as original parity notes and scenario descriptions.
3. Team authors original fixtures and expected outcomes from those notes.
4. Rust implementation is written from scratch against the parity notes and project spec.
5. Comparison harness validates `todos-lsp` behavior against the pinned upstream binary on the clean-room corpus.
6. Any case requiring suspected source-derived reconstruction or undocumented quirk mirroring is escalated before merge.

### Key Interfaces
- **Allowed reference boundary**
  - Upstream CLI help text, documented flags, release notes, public docs, and runtime behavior may inform parity expectations.
  - Upstream binaries may be executed for black-box verification.
- **Prohibited artifact boundary**
  - No verbatim or near-verbatim copying of upstream code, tests, README prose, examples, fixtures, regexes, comment grammars, language-definition tables, ignore-pattern tables, output templates, or internal data structures.
  - No transliteration across languages or “manual re-expression” of upstream implementation artifacts with only superficial syntax changes.
- **Verification boundary**
  - Parity is established by authored fixtures and observed outputs, not by asserting equivalence to upstream internals.

## Consequences

### Benefits
- Preserves a defensible clean-room rewrite posture.
- Keeps MVP parity evidence rigorous and reviewable.
- Prevents accidental long-term upstream lockstep through copied artifacts.
- Forces the team to encode behavior as project-owned requirements rather than imported implementation detail.

### Tradeoffs
- Fixture creation and review will be slower than copying upstream scenarios.
- Some early work may need to be discarded if provenance is unclear.
- Undocumented edge-case parity may remain unresolved until explicit escalation is completed.

### Risks
| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Contributor copies or lightly rewrites upstream artifact content | Medium | High | Require provenance review for parser logic, fixtures, tables, and docs; reject suspiciously similar artifacts |
| Team weakens parity verification to avoid clean-room effort | Medium | High | Keep parity matrix and black-box comparison harness mandatory for MVP gate |
| Ambiguous upstream behavior triggers late disputes | Medium | High | Record observation notes early; escalate ambiguity before implementation hardens |
| Exact undocumented quirk reproduction pressures team into derivative design | Medium | High | Clean-room rules override by default; require lead approval for any quirk-specific decision |
| Existing work must be reset due to provenance concerns | Low | High | Prefer reset over carrying tainted artifacts; review provenance continuously rather than late |

## Implementation Notes
- Treat `ianlewis/todos` as a black-box behavioral oracle for MVP parity only.
- Allowed reference methods:
  - run the upstream binary against original corpora and record outputs/exit behavior;
  - read public documentation, CLI help, changelogs, and release metadata;
  - write independent scenario notes summarizing observed behavior in original words;
  - use parity matrices that describe user-visible behavior categories only.
- Disallowed practices:
  - copying or adapting upstream source files, tests, fixtures, README text, docs prose, examples, regexes, language tables, ignore tables, sample outputs, or comment/token grammars;
  - deriving regexes by minimally editing upstream regexes;
  - copying test case structure line-for-line even with renamed files/messages;
  - preserving upstream typos, prose cadence, or table ordering where doing so suggests artifact copying.
- Fixture and test authoring rules:
  - every fixture must be newly authored and stored with a provenance note stating it was created for `todos-lsp` parity testing;
  - fixtures should model behavior categories, not upstream file contents;
  - expected results may be generated by executing the pinned upstream binary on the authored corpus, but the expected-result artifacts must be stored as `todos-lsp` parity evidence, not as copied upstream tests;
  - when a scenario is inspired by an upstream-documented behavior, rewrite the scenario description independently and vary filenames/messages/content enough to avoid derivative reproduction while still testing the same behavior class;
  - prefer minimal fixtures that isolate one behavior at a time plus a smaller number of realistic integration corpora.
- Review and documentation expectations:
  - contribution docs must state the clean-room rules plainly;
  - parity-related PRs must include provenance notes for fixtures, parser rules, and support tables;
  - ADR-3 and the parity matrix must reference this ADR as the governing policy;
  - project docs may acknowledge that MVP parity was validated against `ianlewis/todos` v0.14.0, but must not imply endorsement, code sharing, or ongoing lockstep.
- Attribution expectations:
  - cite `ianlewis/todos` v0.14.0 as the behavioral reference for MVP parity in ADRs, parity docs, and release notes where relevant;
  - do not copy upstream license notices into files unless legally required for some separately incorporated asset, which this ADR assumes is not permitted for MVP;
  - describe the relationship as “behavioral reference/inspiration” and “clean-room rewrite,” not “port,” “translation,” or “derived implementation.”
- Escalation rules:
  - escalate immediately if an engineer cannot tell whether a planned artifact is independently authored;
  - escalate if black-box observation is insufficient to resolve behavior needed for parity;
  - escalate if reproducing an undocumented quirk appears to require reverse-engineering internal artifacts rather than validating user-visible behavior;
  - escalate if a reviewer suspects a regex, fixture, table, or doc is too similar to upstream;
  - escalation outcome must explicitly choose one of: documented independent interpretation, narrowed MVP parity claim, or approved reset/rework of tainted artifacts.
- Breaking/reset guidance:
  - if tainted artifacts are found, delete and recreate them rather than editing around the contamination;
  - if a shortcut already introduced copied or derivative material, reset that work before implementation proceeds, even at schedule cost. Proper architecture and provenance are worth the inconvenience. How inconvenient for shortcuts.
- [CROSS-TEAM REVIEW]: lead approval is required before parity-fixture implementation proceeds under this policy.
- [CROSS-TEAM IMPACT]: any request to mirror undocumented upstream quirks exactly may increase legal and maintenance risk and requires lead review.

## Questions for User
- None.
