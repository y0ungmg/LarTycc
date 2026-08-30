# LarTycc language matrix

Status: accepted language governance. A listing here is not an implementation
commitment. New toolchains enter only through the gates below.

## Lanes and build policy

| Lane | Build policy | Admission rule |
| --- | --- | --- |
| CORE | Required for normal contributor/product builds | Existing product owner |
| OPTIONAL | Feature-gated; product works without it | Measured user value and fallback |
| EXPERIMENTAL | Independent build/test; never packaged by default | Stable interchange format and prototype exit criteria |
| RESEARCH | Reproducible notebooks/benchmarks only | Result must be portable to production code |
| SERVICES | Separate deployable; desktop stays local-first | Versioned network protocol and offline behavior |
| INFRASTRUCTURE | Developer/metadata tooling | Cannot become an application runtime dependency by accident |

The default LarTycc build requires C++20, Rust, TypeScript/Node, and Python for
the repository-wide checks. The shipped workstation runtime must not require
Python or any incubation toolchain. CI for an incubation component is separate
and may not weaken or lengthen the core quality gate without an ADR.

## Summary

| Language | Component | Lane | Required | Runtime | Technical reason |
| --- | --- | --- | --- | --- | --- |
| C++20 | audio engine | CORE | Yes | native | deterministic callback, DSP graph, device boundary |
| C | reviewed DSP/device primitives | OPTIONAL | No | native | narrow ABI and vendor/library interoperability |
| Rust | application core, host, AI runtime | CORE | Yes | native | ownership-safe project truth and orchestration |
| TypeScript | React UI | CORE | Yes | webview | accessible product presentation and interaction |
| Python | AI/data toolchain | CORE repository tooling | No at runtime | offline | mature training and dataset ecosystem |
| FAUST | `dsp-faust/` effects/instruments | OPTIONAL | No | generated native code | declarative, analyzable signal processing |
| Haskell | LarTycc Patterns | EXPERIMENTAL | No | offline worker/tool | compositional generative pattern algebra |
| Prolog | `theory-engine/` | EXPERIMENTAL | No | isolated tool | explainable theory constraints and search |
| Zig | `native-tools/` | EXPERIMENTAL | No | native utility | small cross-platform process/binary utilities |
| Julia | `research-julia/` | RESEARCH | No | research only | numerical DSP/ML prototyping and benchmarking |
| Gleam | LarTycc Live | SERVICES | No | BEAM service | supervised collaborative session processes |
| OCaml | MusicScript compiler | EXPERIMENTAL | No | offline compiler | typed AST and deterministic DSL compilation |
| Racket | LiveCode lab | EXPERIMENTAL | No | separate tool | macro-driven power-user live-coding research |
| Lua | user scripting | OPTIONAL | No | sandboxed worker | small embeddable automation language |
| GLSL/HLSL/WGSL | visualization shaders | OPTIONAL | No | selected GPU backend | spectrum, waveform, and spectrogram rendering |
| WebAssembly | extension sandbox | OPTIONAL | No | sandbox | portable capability-limited extensions |
| Assembly | profiled kernels | OPTIONAL | No | native | last-mile optimization after measurement |
| Go | operational services | SERVICES | No | separate service | simple deployment tooling where BEAM is unnecessary |
| CUDA | NVIDIA acceleration | RESEARCH/OPTIONAL | No | optional GPU | batch analysis and inference acceleration |
| Nix | reproducible development shells | INFRASTRUCTURE | No | developer machine/CI | pinned multi-toolchain environments |
| SQL | registries and indexes | INFRASTRUCTURE | No | embedded/service DB | structured querying of metadata, not project truth |

## Ownership cards

### C++20

- **Why this language?** Predictable native performance and established audio APIs.
- **Why not Rust instead?** The existing tested engine is C++; rewriting it adds risk without user value.
- **Owns:** callback, graph, DSP buffers, device/MIDI scheduling.
- **Forbidden:** project files, UI, networking, ML, blocking callback work.

### C

- **Why this language?** Stable ABI and compatibility with focused DSP/system libraries.
- **Why not Rust/C++ instead?** Use C only when the boundary or dependency is genuinely C-shaped.
- **Owns:** reviewed primitives and ABI surfaces with explicit lifetime rules.
- **Forbidden:** application orchestration, project state, broad new subsystems.

### Rust

- **Why this language?** Strong ownership and error handling for canonical state and native orchestration.
- **Why not C++ instead?** Rust reduces memory/lifetime risk outside the hard realtime callback.
- **Owns:** project truth, commands, undo, persistence, host routing, jobs, capability policy.
- **Forbidden:** React presentation and unreviewed allocation/blocking inside the audio callback.

### TypeScript

- **Why this language?** Typed React tooling and direct accessibility/browser APIs.
- **Why not Rust/C++ instead?** Product UI iteration and webview integration are faster and clearer here.
- **Owns:** views, input, local ephemeral UI state, host request creation.
- **Forbidden:** canonical project mutation, filesystem/device access, direct DSP state.

### Python

- **Why this language?** Best-fit ML, evaluation, and dataset tooling.
- **Why not Rust/C++ instead?** Research libraries and iteration speed dominate offline tasks.
- **Owns:** ingestion, training, evaluation, export, offline experiments.
- **Forbidden:** shipped realtime path, authoritative project mutations, mandatory app runtime.

### FAUST

- **Why this language?** Signal-flow definitions can generate optimized native DSP and multiple targets.
- **Why not Rust/C++ instead?** It reduces boilerplate for suitable processors and enables structural analysis.
- **Owns:** selected effect/instrument equations and generated-code fixtures.
- **Forbidden:** device I/O, graph ownership, project state, UI; generated output without parity benchmarks.

### Haskell

- **Why this language?** Algebraic composition suits transformations, probability, Euclidean rhythm, and polyrhythm.
- **Why not Rust/C++ instead?** The spike tests whether a functional pattern model is materially clearer.
- **Owns:** LarTycc Patterns offline generation into the neutral Event Format.
- **Forbidden:** realtime audio thread, direct project mutation, copied third-party live-coding APIs.

### Prolog

- **Why this language?** Declarative facts, constraints, and explainable search fit music theory queries.
- **Why not Rust/C++ instead?** Backtracking rules are easier to inspect than hand-built search trees.
- **Owns:** scales, chords, harmony, tension facts, progressions, voice-leading constraints.
- **Forbidden:** subjective quality judgments, autonomous edits, realtime execution, unbounded queries.

### Zig

- **Why this language?** Small native binaries, explicit memory control, and cross-compilation tooling.
- **Why not Rust/C++ instead?** Admit only if plugin/process tooling is simpler and measurably smaller.
- **Owns:** future plugin scanner, launcher, binary inspector, crash-isolation helpers.
- **Forbidden:** replacing the C++ engine, project state, UI, speculative utilities.

### Julia

- **Why this language?** High-level numerical notation with strong FFT/scientific performance.
- **Why not Rust/C++ instead?** It accelerates research before a validated algorithm is ported.
- **Owns:** reproducible DSP, feature-extraction, pitch, and dataset experiments.
- **Forbidden:** production runtime, release-critical implementation, irreproducible notebook-only claims.

### Gleam

- **Why this language?** Typed BEAM code and supervision fit long-lived collaborative sessions.
- **Why not Rust/C++ instead?** Actor supervision may simplify rooms, presence, and reconnect behavior.
- **Owns:** future LarTycc Live sessions and ordered project-event transport.
- **Forbidden:** audio streaming in the first collaboration version, local project truth, mandatory connectivity.

### OCaml

- **Why this language?** Algebraic data types and parser tooling suit a small deterministic compiler.
- **Why not Rust/C++ instead?** The experiment tests compiler clarity, not novelty; it must beat a Rust baseline.
- **Owns:** MusicScript lexer, parser, AST, semantic checks, Event Format output.
- **Forbidden:** audio control, direct project writes, live-coding semantics owned by Racket.

### Racket

- **Why this language?** Macros and interactive evaluation fit an expert live-coding laboratory.
- **Why not Rust/C++ instead?** Language experimentation is the feature, isolated from the product DSL.
- **Owns:** LiveCode procedural/event generation for power users.
- **Forbidden:** MusicScript syntax/ownership, core UI, realtime callback, implicit project mutation.

### Lua

- **Why this language?** Small embeddable runtime and familiar scripting model.
- **Why not Rust/C++ instead?** User-authored automation needs a constrained dynamic surface.
- **Owns:** capability-scoped user scripts if the extension threat model passes.
- **Forbidden:** raw filesystem/network/device access, callback execution, bypassing commands/undo.

### GLSL, HLSL, and WGSL

- **Why these languages?** They map visualization work to the selected graphics API.
- **Why not Rust/C++ instead?** GPU shading is expressed at the GPU pipeline boundary.
- **Owns:** spectrum, spectrogram, waveform, and measured visual effects.
- **Forbidden:** three duplicate implementations without active backends; DSP/project logic in shaders.

### WebAssembly

- **Why this language?** Portable sandbox with memory and capability boundaries.
- **Why not native Rust/C++ instead?** Untrusted extensions need isolation more than raw access.
- **Owns:** future bounded extension modules with versioned imports/exports.
- **Forbidden:** unrestricted host calls, realtime deadlines before proof, undocumented ABI.

### Assembly

- **Why this language?** Only for a profiler-proven kernel unsupported by compiler intrinsics.
- **Why not Rust/C++ instead?** Those remain the maintainable baseline and reference implementation.
- **Owns:** tiny architecture-specific leaf routines with equivalence tests.
- **Forbidden:** control flow, business logic, portability-critical features, premature optimization.

### Go

- **Why this language?** Operational simplicity for a future stateless service or CLI.
- **Why not Rust/Gleam instead?** It must demonstrate a clearer deployment/operations fit first.
- **Owns:** optional infrastructure that does not need BEAM session semantics.
- **Forbidden:** duplicate collaboration ownership, desktop core, audio processing.

### CUDA

- **Why this language?** NVIDIA kernels may accelerate inference and batch audio analysis.
- **Why not CPU Rust/C++ instead?** CUDA is admitted only after a measured bottleneck and useful speedup.
- **Owns:** optional kernels and parity benchmarks.
- **Forbidden:** required startup path, NVIDIA-only features, missing CPU fallback, UI rendering policy.

### Nix

- **Why this language?** Reproducible toolchain composition for development and CI.
- **Why not shell scripts alone?** Pinning many optional compilers exceeds reliable ad-hoc setup.
- **Owns:** core and opt-in extended development shells, never application behavior.
- **Forbidden:** making `nix develop` the only supported setup or forcing all incubator toolchains.

### SQL

- **Why this language?** Indexing and querying structured sample/plugin/model metadata.
- **Why not Rust collections/files alone?** Durable search and migrations become valuable at scale.
- **Owns:** metadata registries, caches, indexes, future service storage.
- **Forbidden:** canonical `.lartycc` project representation, audio-thread queries, hidden user-content upload.

## Integration requirements

Every cross-language edge declares its protocol, schema/version, limits, timeout,
error model, and conformance fixtures. Allowed defaults are C ABI for native FFI,
JSON/MessagePack for isolated tools, Protobuf for workers/services, WebSocket for
collaboration, and Wasm imports/exports for extensions. Raw language-specific
memory layouts never cross a boundary.

Initial planned conformance edges are Rust→C++ audio, C++→generated FAUST DSP,
Rust→Prolog, Rust→Haskell, Rust→OCaml, TypeScript→Rust host, and Gleam→Rust client
protocol. Only Rust→C++ and TypeScript→Rust are active in Phase 1.

## Entry and removal gates

Before implementation, an incubation component needs:

1. an ADR proving why an existing core language is insufficient;
2. an owner and supported toolchain/version policy;
3. dependency, source, dataset, and generated-code license review;
4. a stable neutral boundary and malformed-input/timeout behavior;
5. independent tests and CI that do not make the core build depend on it;
6. a baseline comparison for performance, complexity, binary size, and maintenance;
7. explicit graduation and deletion criteria.

An experiment that misses its exit criteria is removed with its boundary intact
for future reconsideration; it does not linger as an unmaintained required tool.
