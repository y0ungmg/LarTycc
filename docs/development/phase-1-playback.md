# Phase 1 playback slice

The current slice proves the project lifecycle and realtime engine separately:

```bash
cmake -S . -B build/cpp
cmake --build build/cpp --parallel
./build/cpp/audio-engine/lartycc_render_demo

cargo run -p lartycc-desktop -- demo-project.json
```

The renderer creates `lartycc-demo.wav`: one second of a generated mono sample
played through the stereo master path in 128-frame callback blocks. Loading and
allocation happen before `play`; `process` only reads prepared storage and uses
atomics for transport and gain.

The desktop command creates or opens a validated project, adds the first audio
track through the command system, and saves with a sibling temporary file. The
React Easy timeline is the presentation prototype for the same state.

`AudioOutput` now enumerates stable device IDs and drives the engine callback
through WASAPI on Windows or ALSA on Linux. The null backend keeps headless CI
deterministic. The React `HostBridge` consumes typed device and transport
snapshots and falls back to an explicit browser preview implementation.

Native bridge injection and Rust project-command routing into the C++ transport
remain Phase 1 work. Hardware availability and low-latency behavior must still
be measured on reference PCs before the full Phase 1 gate can close.
