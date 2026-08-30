# Training Beat Generator v0

Training is offline, reproducible, and optional; it never runs inside the DAW.

## Reproducible run

1. Pin a reviewed dataset manifest and tokenizer version.
2. Validate rights coverage, group-aware split, hashes, and quality thresholds.
3. Generate token shards and a signed summary without modifying raw inputs.
4. Train from a versioned YAML configuration with all random seeds recorded.
5. Evaluate every checkpoint on objective, human, safety, and memorization suites.
6. Export a model package and verify parity against the training runtime.
7. Publish a model card only after license and release-gate review.

Planned command surface:

```bash
python -m lartycc_ai.dataset validate configs/dataset-v0.yaml
python -m lartycc_ai.dataset build configs/dataset-v0.yaml
python -m lartycc_ai.train configs/beat-generator-tiny-v0.yaml
python -m lartycc_ai.evaluate runs/<run-id>
python -m lartycc_ai.export runs/<run-id> --format onnx
```

These modules beyond the tokenizer primitive are design targets, not Phase 0
implementations.

## Metrics and gates

Track loss/perplexity, onset F1, constraint adherence, groove similarity,
polyphony/collision errors, diversity, repetition, silence, nearest-neighbor
memorization, inference latency, peak memory, and artifact size. Human raters
compare candidates blindly and score usefulness, controllability, musicality,
and edit acceptance. Results are sliced by genre, tempo, time signature, track
role, length, and hardware tier.

A candidate fails release if rights coverage is incomplete, leakage is found,
export parity exceeds tolerance, memorization crosses the reviewed threshold,
or measured latency/memory misses the published tier.

## Checkpoint policy

Checkpoints and datasets do not enter Git. Store them in access-controlled
artifact storage with checksums, retention, and lineage. A release package
contains only reviewed weights, tokenizer, model card, license, manifest
references, compatibility metadata, and benchmark results.

