# Dataset governance and format v0

No dataset is distributed in Phase 0. This document defines the acceptance bar
for future symbolic-music training data.

## Non-negotiable rules

- Public availability is not permission to train or redistribute.
- Every item needs provenance, a stable source reference, license/consent basis,
  allowed uses, and a source hash.
- Unknown, conflicting, non-commercial-only, or revocable terms are quarantined
  until a maintainer completes a written compatibility review.
- Personal information is removed. Takedown and exclusion requests propagate to
  future manifests, checkpoints, and published model cards.
- Dataset, code, model, and output licenses are tracked independently.

## Example manifest record

```json
{
  "schema_version": 1,
  "example_id": "sha256:...",
  "dataset": { "name": "example", "version": "1.0.0" },
  "source": {
    "uri": "https://example.invalid/item/123",
    "retrieved_at": "2026-08-30T00:00:00Z",
    "sha256": "...",
    "creator_group_id": "artist-or-source-group"
  },
  "rights": {
    "spdx": "CC0-1.0",
    "evidence_uri": "https://example.invalid/license",
    "training_allowed": true,
    "redistribution_allowed": true
  },
  "music": {
    "bpm": 145.0,
    "time_signature": "4/4",
    "key": "F# minor",
    "bars": 8,
    "roles": ["drums", "bass"]
  },
  "pipeline": {
    "tokenizer": "remi-lartycc-v0",
    "transformations": [],
    "quality_flags": [],
    "split": "train"
  }
}
```

## Pipeline

Ingest into a read-only raw store, snapshot license evidence, validate MIDI,
normalize without destroying the original, identify near-duplicates, group by
creator/source, compute quality features, assign leakage-safe splits, tokenize,
and write immutable sharded artifacts. Every stage records code revision,
configuration hash, input/output checksums, counts, failures, and seed.

## Split and dedup policy

Exact hashes and normalized event hashes remove duplicates. MinHash/fingerprint
search flags transpositions and near-duplicates. All versions, arrangements,
and works from one source/creator group stay in one split. Default split is
90/5/5 by group with a separate frozen challenge set. Evaluation data must not
be selected by listening to model results.

## Takedown

The exclusion ledger stores source hashes and group IDs without retaining the
removed content. A takedown produces a new manifest, identifies affected model
runs, blocks future training, updates model cards, and triggers retraining or
other remediation based on legal and technical review.

