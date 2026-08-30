# Desktop host

The Phase 1 native host is a Rust project-lifecycle and audio-device executable.
It creates or opens a validated project, routes initial edits through the command
system, and saves atomically. It also reaches the C++ engine through the safe
`lartycc-audio-bridge` crate.

```bash
cargo run -p lartycc-desktop -- project.json
cargo run -p lartycc-desktop -- --list-devices
cargo run -p lartycc-desktop -- --play-test [device-id]
```

The window/webview and typed React IPC binding remain open Phase 1 work. ALSA on
Linux and WASAPI on Windows are integrated, but reference-PC latency and underrun
qualification must pass before the Phase 1 hardware-audio gate closes.
