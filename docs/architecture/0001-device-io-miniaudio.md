# ADR 0001: miniaudio for the Phase 1 device boundary

- Status: accepted
- Date: 2026-08-30
- Owners: LarTycc maintainers

## Context

LarTycc needs low-latency WASAPI and ALSA output without moving project state,
DSP, or UI into a cross-platform framework. Hand-maintaining both platform APIs
would delay the first playback slice and multiply lifecycle bugs.

## Decision

Use miniaudio 0.11.25 at commit `9634bedb5b5a2ca38c1ee7108a9358a4e233f14d`
under MIT No Attribution. Build only device I/O with WASAPI on Windows, ALSA on
Linux, and the null backend for CI. Decoding, resource management, node graph,
generation, and miniaudio's high-level engine are disabled. LarTycc's
`AudioOutput` pimpl is the only public boundary; DSP remains `AudioEngine`.

## Alternatives

JUCE adds mature device and plugin APIs but introduces broader framework and
license coupling. RtAudio/PortAudio are viable but add system-library packaging
and do not materially improve this narrow slice. Direct WASAPI/ALSA maximizes
control but has the highest implementation and test burden.

## Consequences

Device enumeration and callbacks work through one dependency while the wrapper
keeps replacement possible. Configure requires fetching a pinned repository.
Releases must reproduce and archive that source, retain its selected license,
test unplug/default-device changes, and audit every version update.

## Rollout and reversal

CI builds the wrapper on Windows and Linux. Hardware tests remain separate from
headless CI. The dependency can be replaced behind `AudioOutput` without changing
the engine or UI contracts.
