# Provenance Workflow

1. Observe upstream only through allowed black-box channels: CLI help, public docs, changelog notes, and released binaries.
2. Author original fixture text and metadata under `tests/fixtures/`.
3. Store parity expectations in `docs/parity/matrix.md` before claiming a row as passed.
4. Prefer project-owned behavioral summaries over copied wording.
5. Escalate undocumented quirk mirroring if parity would require reverse-engineering internal upstream artifacts.

Current authored corpus:

- `tests/fixtures/basic`: recursive scan, inline comments, ignore handling, generated-file suppression, INI parsing.
- `tests/fixtures/unsupported`: direct unsupported file failure behavior.
- tempdir-authored parity corpora created during tests: charset detection, symlink traversal, blame success path, mixed-language representative support, and unsupported-file suppression.
