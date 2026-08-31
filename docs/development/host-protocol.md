# Native host protocol v1

The desktop shell exposes one command router to React and development tools. A
request and its response are JSON objects validated against
`shared/schemas/host-protocol-v1.schema.json`. Newline-delimited JSON over stdio
is a conformance/debug transport; the embedded webview will invoke the same Rust
router without a network listener.

```json
{"version":1,"id":"tempo-1","command":"project.setTempo","payload":{"bpm":148},"expectedProjectRevision":0}
```

```json
{"version":1,"id":"tempo-1","ok":true,"result":{"schemaVersion":1,"projectId":"local-project","revision":1,"tempo":148,"tracks":[]}}
```

Every project mutation requires `expectedProjectRevision`. A stale request fails
with `revision_conflict`; the UI must refresh state and let the user retry rather
than silently applying an edit to a different revision. Errors have stable
machine-readable `code` values and human-readable `message` text. Unknown
versions and commands fail closed.

## Commands

| Command | Mutation | Result |
| --- | --- | --- |
| `host.getState` | no | project, transport, audio availability |
| `audio.listDevices` | refreshes device snapshot | device list |
| `audio.loadTestTone` | loads developer fixture | frames and sample rate |
| `transport.play` | native transport | transport snapshot |
| `transport.stop` | native transport | transport snapshot |
| `transport.seek` | native transport | transport snapshot |
| `project.setTempo` | project | project snapshot |
| `project.createTrack` | project | project snapshot |
| `project.undo` / `project.redo` | project | project snapshot |
| `project.save` | filesystem | project snapshot |

`audio.loadTestTone` is a Phase 1 integration fixture, not a project-file model.
Loading real project assets remains a separate playback-slice requirement.

The optional `apps/desktop-webview` shell provides the in-process adapter. Its
initialization script exposes `window.lartyccHost`, passes each envelope to the
same router, and publishes returned transport snapshots to React listeners.

## Limits and evolution

The protocol is process-local and does not authenticate remote clients. A future
network or collaboration boundary uses its own authenticated Protobuf/WebSocket
contract. Version 1 accepts bounded JSON supplied by a trusted packaged webview;
before accepting untrusted sources the host must enforce byte/depth limits and
per-command timeouts. Breaking changes require a new schema version and parallel
compatibility tests.
