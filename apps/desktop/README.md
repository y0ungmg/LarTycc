# Desktop host

The Phase 1 native host is a Rust project-lifecycle and audio-device executable.
It creates or opens a validated project, routes initial edits through the command
system, and saves atomically. It also reaches the C++ engine through the safe
`lartycc-audio-bridge` crate.

```bash
cargo run -p lartycc-desktop -- project.json
cargo run -p lartycc-desktop -- --list-devices
cargo run -p lartycc-desktop -- --play-test [device-id]
printf '%s\n' '{"version":1,"id":"state","command":"host.getState"}' \
  | cargo run -p lartycc-desktop -- --host-stdio project.json
```

`--host-stdio` accepts one protocol-v1 JSON request per line and emits one JSON
response per line. The router validates versions, request IDs, command payloads,
and expected project revisions before touching project or audio state. This is a
development and conformance transport; the future webview adapter calls the same
router in process.

The window/webview and typed React IPC binding remain open Phase 1 work. ALSA on
Linux and WASAPI on Windows are integrated, but reference-PC latency and underrun
qualification must pass before the Phase 1 hardware-audio gate closes.
