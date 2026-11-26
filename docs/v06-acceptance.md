# v0.6 acceptance — release candidate, not stable
- Native CVNSS tables: 758 base rows, 336 patch rows, 56 ambiguous reverse groups; 867 unique forward keys, 810 reverse keys.
- Runtime Rust CQN/CVSS word rules, candidate resolution and reversible raw escape. Full public JS converter API (CVN result objects, audit metadata, inspection) is not yet ported.
- B27N tags 0–9; legacy B27F decode; B27S streaming (256 KiB blocks).
- B27S: magic B27S, repeated big-endian u32 frame size and B27N frame, zero terminator; no trailing bytes. Each block has SHA-256. No whole-stream authentication: deletion/reordering of intact blocks is not detected.
- Browser Studio: local WASM single-frame codec; manual trusted artifact load if not deployed.
- Stable gate: all CI green on exact commit, JS/Rust rule equivalence (including non-Vietnamese Unicode), deterministic rule generation, lockfile, fuzzing, memory and CPU benchmarks, release artifacts, streaming whole-file integrity. No claim of 100% acceptance yet.
