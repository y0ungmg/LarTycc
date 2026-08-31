# Phase 1 playback slice

The current slice proves the project lifecycle and realtime engine separately:

```bash
cmake -S . -B build/cpp
cmake --build build/cpp --parallel
./build/cpp/audio-engine/lartycc_render_demo

cargo run -p lartycc-desktop -- demo-project.json
cargo run -p lartycc-desktop -- --list-devices
cargo run -p lartycc-desktop -- --play-test [device-id]
printf '%s\n' '{"version":1,"id":"state","command":"host.getState"}' \
  | cargo run -p lartycc-desktop -- --host-stdio demo-project.json
npm run build
cargo run --manifest-path apps/desktop-webview/Cargo.toml -- demo-project.json ui/dist
```

The renderer creates `lartycc-demo.wav`: one second of a generated mono sample
played through the stereo master path in 128-frame callback blocks. Loading and
allocation happen before `play`; `process` only reads prepared storage and uses
atomics for transport and gain.

The desktop command creates or opens a validated project, adds the first audio
track through the command system, and saves with a sibling temporary file. The
React Easy timeline is the presentation prototype for the same state.

`AudioOutput` enumerates stable device IDs and drives the engine callback
through WASAPI on Windows or ALSA on Linux. The null backend keeps headless CI
deterministic. `lartycc-audio-bridge` owns a native host through the stable C
ABI, contains all necessary `unsafe` calls in reviewed boundary code, and
exposes safe Rust device, sample, and transport operations. Sample replacement
is rejected while the device callback is running.

The `--list-devices` command prints the stable ID, display name, and default
marker. `--play-test` generates a 220 Hz signal before starting the device,
plays it for one second in 128-frame blocks, then reports the callback count.
Supplying an ID selects that exact device; omitting it uses the backend default.

The React `HostBridge` consumes typed device and transport snapshots and falls
back to an explicit browser preview implementation.

The Rust `HostRouter` now implements the same versioned command vocabulary for
project state, optimistic-revision mutations, persistence, device enumeration,
and transport. Its JSON Lines mode is a development/conformance transport. The
shared JSON Schema and TypeScript adapter test request envelopes and stable error
codes on both sides of the future embedded webview boundary.

The Wry/Tao shell injects that adapter into a real WebKitGTK or WebView2 window
while remaining outside the core Cargo workspace.

The opt-in `lartycc_latency_probe` now produces versioned callback timing
reports without adding clock reads to normal playback. The qualification matrix
and the distinction between nominal buffer duration and wired round-trip
latency are defined in
[`../performance/realtime-qualification.md`](../performance/realtime-qualification.md).
Physical Linux and Windows results are still required before the Phase 1 gate
can close.
