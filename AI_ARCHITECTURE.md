# AI architecture

LarTycc AI is a constrained planning subsystem, not an authority over project
state. Its output is always untrusted until schema validation, semantic checks,
simulation, and (for edits) user approval succeed.

## Runtime components

1. Intent router classifies explanation, analysis, generation, or edit.
2. Context Engine builds a minimal, redacted project slice with a token budget.
3. Model Manager selects a compatible local model by task, hardware, and policy.
4. Planner returns versioned structured tool calls with deterministic seeds.
5. Tool gateway validates capabilities, ranges, complexity, and project revision.
6. Simulator runs the transaction on a copy-on-write project snapshot.
7. Preview service produces a human diff and, when useful, offline audio.
8. Apply submits the exact validated commands to the central command bus.

```mermaid
flowchart LR
  PROMPT["Prompt"] --> INTENT["Intent router"] --> CTX["Context Engine"]
  CTX --> MODEL["Model Manager"] --> PLAN["Structured plan"]
  PLAN --> VALIDATE["Tool gateway"] --> SIM["Simulation"] --> PREVIEW["Preview / Apply"]
```

## Tool policy

Tools are allowlisted by task. Each declares input/output schema, required
capability, worst-case cost, affected project region, preview mode, and inverse
strategy. No tool executes arbitrary code, shell commands, network requests, or
unbounded filesystem access. Destructive project-wide operations always require
explicit approval. Explanations cannot smuggle mutation calls.

## Model Manager contract

Model packages contain weights, tokenizer, generation defaults, model card,
task/capability list, compatible schema versions, checksums, signature, license,
minimum hardware, and benchmark results. Selection considers task support,
available RAM/VRAM, latency target, language, offline policy, and user choice.
Downloads are opt-in and resumable; load failures fall back to a smaller model.

## Model and data card minimum

- creator, version, date, license, intended and prohibited uses;
- training datasets with exact manifest hashes and licenses;
- architecture, parameter count, tokenizer, quantization, and export format;
- evaluation by genre/task and known limitations;
- memorization, privacy, bias, and safety testing;
- tested hardware and measured latency/memory;
- compatibility range and cryptographic checksums.

## Threat model

Treat prompts, project metadata, imported MIDI, model files, and model output as
untrusted. Defend against prompt injection in file metadata, path traversal,
resource exhaustion, malformed tensors, incompatible model schemas, tool-call
amplification, and proposals that target hidden tracks. A later sandbox will
separate model execution; Phase 0 establishes the interface and deny-by-default
policy only.

