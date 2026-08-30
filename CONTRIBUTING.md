# Contributing

LarTycc is pre-alpha. Discuss broad product or architecture changes in an issue
before implementation. Keep pull requests focused and do not start work from a
later roadmap phase before the current gate is complete.

## Development flow

1. Fork or branch from `main` and use a descriptive branch name.
2. Run the relevant formatter, linter, unit tests, and build locally.
3. Update docs, schemas, tests, and `LICENSES.md` with the code.
4. Open a PR describing intent, boundaries, tests, real-time implications, and
   screenshots/audio evidence when applicable.
5. Use an ADR in `docs/architecture/` for a durable cross-component decision.

Commit messages should use an imperative subject, for example
`feat(core): add validated tempo command`.

## Definition of done

- behavior and failure paths are tested;
- audio-callback code performs no allocation, blocking, I/O, logging, or ML;
- persisted and IPC changes have versions and migration/compatibility notes;
- UI changes are keyboard accessible and work in Easy and Pro views as relevant;
- AI tools are schema-bound, capability-limited, previewable, and undoable;
- CI passes on supported toolchains;
- user-facing behavior and license records are documented.

## Dependencies, data, models, and assets

Do not add a dependency solely for convenience. Record exact purpose, source,
license, shipped/not-shipped status, maintenance health, and alternatives in
`LICENSES.md`. Datasets/models additionally require provenance, consent/license
evidence, allowed uses, takedown path, and a completed data/model card. Never
commit copyrighted samples or weights without explicit redistribution rights.

## Architecture decisions

Copy `docs/architecture/0000-adr-template.md`, assign the next number, and record
context, decision, alternatives, consequences, rollout, and reversal plan. ADRs
are required for FFI/IPC, file format, realtime primitives, UI host, plugin API,
model runtime, or license strategy changes.

