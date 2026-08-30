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

This is intentionally a **headless playback slice**. A real ALSA/WASAPI device
adapter and the typed IPC binding between the native host and React remain Phase
1 work. The project does not claim audible hardware playback yet.
