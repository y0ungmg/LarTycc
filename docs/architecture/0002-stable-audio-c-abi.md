# ADR 0002: Stable C ABI for the native audio bridge

- Status: Accepted
- Date: 2026-08-30

## Context

The desktop host is written in Rust while realtime audio and device I/O are
implemented in C++20. Binding C++ classes directly would expose compiler ABI,
standard-library, exception, and ownership details across the language
boundary. Those details are not stable across supported toolchains.

## Decision

Expose an opaque `lartycc_audio_host` through a C-compatible API. The handle
owns `AudioEngine`, `AudioOutput`, and a refreshed device snapshot. The API uses
fixed-width-compatible scalar values, caller-owned string buffers, explicit
capacities, and boolean success results. C++ exceptions are caught before they
can cross the ABI.

The `lartycc-audio-bridge` crate is the only Rust component that calls this API.
It owns the opaque pointer, destroys it through `Drop`, converts device strings,
and exposes ordinary `Result`-based methods. The handle is deliberately neither
`Send` nor `Sync`. Unsafe code remains denied workspace-wide and is allowed only
on the reviewed FFI declarations and wrapper implementations.

Audio samples must be loaded before device startup. The C boundary rejects a
sample replacement while the callback is running, so the realtime thread never
races a vector allocation or replacement.

## Consequences

- Rust and C++ can use different compilers without sharing a C++ ABI.
- Ownership, nullability, string capacity, and failure behavior are explicit.
- New native operations require coordinated additions to the C header, C++
  implementation, and safe Rust wrapper.
- The current API is process-local. IPC and React webview injection remain
  separate application-host responsibilities.
