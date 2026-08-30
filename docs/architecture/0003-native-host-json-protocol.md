# ADR 0003: Versioned JSON protocol for the embedded native host

- Status: Accepted
- Date: 2026-08-30

## Context

React needs to request project and audio operations from the Rust desktop host.
The boundary is local and trusted, but ad-hoc JavaScript objects would make
versioning, errors, stale edits, and conformance difficult to test. The wider
architecture already reserves Protobuf for separate workers and services.

## Decision

Use protocol-v1 JSON request/response envelopes for the embedded webview. Every
request contains `version`, `id`, `command`, optional `payload`, and an expected
project revision for mutations. Every response echoes the version and ID and is
either a result or a stable error code/message. The shared JSON Schema is the
language-neutral source of truth.

Rust owns parsing, command validation, project state, and audio dispatch. The
TypeScript bridge creates envelopes and turns failure responses into errors. A
newline-delimited stdio adapter exercises the exact router during development;
it is not a remote API or the final packaged webview transport.

## Alternatives

- Protobuf in the webview would require a JavaScript generation/runtime layer
  for a small local boundary and would not remove the need for JS object checks.
- Framework-specific command macros would couple the domain router to a window
  toolkit before the toolkit is selected.
- Unversioned JSON would be initially smaller but could not evolve safely.

## Consequences

- Webview injection can remain a thin adapter over an independently tested host.
- Optimistic revision checks prevent silent stale project mutations.
- Breaking changes require a new schema and parallel conformance fixtures.
- Separate workers/services still use Protobuf; collaboration uses an
  authenticated network protocol rather than exposing this local router.
- Before accepting untrusted input, byte/depth limits, timeouts, authentication,
  and capability checks must be added at the outer transport.
