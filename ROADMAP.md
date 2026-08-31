# Roadmap

Roadmap items are gates, not date promises. A phase begins only when the prior
phase's exit criteria are demonstrably green.

## Phase 0 — Architecture and repository foundation (complete)

- [x] Technical Design v1 and AI/data/training documentation
- [x] C++20, Rust, React/TypeScript, and Python component seams
- [x] minimal builds/tests and independent CI jobs
- [x] versioned command and project schema examples
- [x] Apache-2.0 license, third-party policy, security/contribution guides
- [x] merge the Phase 0 pull request and confirm CI on GitHub runners

Exit: clean checkout builds; all tests/lints pass; CI is green on Linux and the
C++ smoke build is green on Windows; README, architecture, and roadmap match the
repository; no Phase 1 behavior is implied.

## Phase 1 — Playback vertical slice (current)

Device selection, transport, clock, sample playback, one mixer path, project
open/save, minimal desktop host, waveform job, and an Easy-mode timeline.

- [x] callback-safe transport, sample playback, seek, and master gain
- [x] offline WAV render demo and realtime stress test
- [x] versioned project open/save with temporary-file replacement
- [x] command undo/redo, autosave entry point, and waveform peak job
- [x] minimal native project host and Easy timeline prototype
- [x] ALSA and WASAPI device adapters with stable device selection IDs
- [x] typed React host bridge, transport snapshots, and browser preview sync
- [x] safe Rust-to-C++ audio bridge and native device test mode
- [x] versioned native-host router for project and audio commands
- [x] inject the native router into a WebKitGTK/WebView2 desktop shell
- [ ] measured underrun/latency suite on reference PCs

Exit: open a project, play a sample without callback allocations, edit/save,
recover autosave, and meet a measured underrun/latency budget on reference PCs.

## Language incubation backlog (inactive until Phase 1 exits)

This backlog records possible specialist components; it does not authorize empty
scaffolds or add any toolchain to the default build. Every activation needs an
ADR, named owner, dependency/license review, versioned boundary, error model,
independent CI job, CPU/binary-size baseline, and a removal plan. See
[`docs/LANGUAGE_MATRIX.md`](docs/LANGUAGE_MATRIX.md).

- [ ] FAUST DSP pilot: one effect, generated C++ boundary, parity and benchmark
  against a hand-written C++ reference
- [ ] Haskell LarTycc Patterns spike producing the neutral Event Format offline
- [ ] Prolog theory-engine spike with deterministic structured queries from Rust
- [ ] Zig native-tool pilot only when plugin scanning/isolation begins
- [ ] Julia research notebook promoted only after a reproducible benchmark
- [ ] Gleam LarTycc Live protocol spike after single-user project commands mature
- [ ] OCaml MusicScript grammar/compiler spike after the Event Format stabilizes
- [ ] Racket LiveCode lab after MusicScript ownership and syntax are established
- [ ] optional Lua/Wasm scripting and extension sandbox threat model
- [ ] choose only the shader language required by the selected graphics backend;
  WGSL is preferred if WebGPU is selected
- [ ] optional CUDA acceleration with measured CPU feature parity and fallback
- [ ] Nix core development shell; extended toolchains remain opt-in shells
- [ ] SQL metadata store only for registries/indexes, never canonical project truth

## Phase 2 — Beat-making core

Channel rack, MIDI patterns, piano roll, sampler, tempo/signature maps, mixer
buses, automation basics, export, undo/redo coverage, and Pro-mode foundations.

Exit: produce and export a complete short beat with deterministic project reload.

## Phase 3 — AI-assisted symbolic editing

Context Engine, tool schemas, local Model Manager, preview/apply/cancel, three
Beat Generator candidates, mix explanations, and education mode.

Exit: every AI edit validates, previews, applies atomically, and undoes; tiny
model meets the published hardware budget and passes memorization/safety gates.

## Phase 4 — Plugin and production workflow

Choose and implement CLAP/VST3 strategy after current licensing review, add
plugin scanning/hosting, crash isolation, sends, richer automation, freeze,
bounce, recording, and performance-regression dashboards.

Exit: plugin failure cannot corrupt the project or crash the main process;
compatibility, latency compensation, and recovery suites pass.

## Phase 5 — Extensibility and release hardening

Model/tool extension contracts, signed updates, accessibility audit, localization,
installers, migration/fuzz campaigns, telemetry opt-in, and beta documentation.

Later research: custom synthesizer, macOS, collaboration, vocal/pitch tools,
plugin SDK, marketplace, remote inference, and text-to-audio. They have no
commitment until core production reliability is proven.
