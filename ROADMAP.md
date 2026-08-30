# Roadmap

Roadmap items are gates, not date promises. A phase begins only when the prior
phase's exit criteria are demonstrably green.

## Phase 0 — Architecture and repository foundation (current)

- [x] Technical Design v1 and AI/data/training documentation
- [x] C++20, Rust, React/TypeScript, and Python component seams
- [x] minimal builds/tests and independent CI jobs
- [x] versioned command and project schema examples
- [x] Apache-2.0 license, third-party policy, security/contribution guides
- [ ] merge the Phase 0 pull request and confirm CI on GitHub runners

Exit: clean checkout builds; all tests/lints pass; CI is green on Linux and the
C++ smoke build is green on Windows; README, architecture, and roadmap match the
repository; no Phase 1 behavior is implied.

## Phase 1 — Playback vertical slice

Device selection, transport, clock, sample playback, one mixer path, project
open/save, minimal desktop host, waveform job, and an Easy-mode timeline.

Exit: open a project, play a sample without callback allocations, edit/save,
recover autosave, and meet a measured underrun/latency budget on reference PCs.

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

