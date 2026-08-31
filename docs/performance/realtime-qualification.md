# Realtime playback qualification

Phase 1 closes only after the playback slice is measured on physical Linux and
Windows reference PCs. CI and virtual machines do not count as hardware
qualification.

## What the probe measures

`lartycc_latency_probe` enables opt-in callback timing counters, plays a quiet
220 Hz signal, and emits one versioned JSON report. A deadline miss is a gap
between callback starts greater than 1.5 times the requested period. The report
also records maximum callback processing time and callback coverage.

The nominal period is a buffer-duration calculation, not end-to-end round-trip
latency. Closing the Phase 1 gate additionally requires an external wired
loopback measurement from output to input; record the median and p95 round-trip
latency alongside the probe report. Do not claim a round-trip figure from the
nominal period alone.

Timing calls are disabled in normal playback. Qualification mode adds two
monotonic-clock reads and lock-free atomic updates per callback.

## Reference matrix

Run a release build on AC power with background updates paused. Record OS build,
CPU, RAM, audio interface, driver/backend and driver version. Test every row for
10 minutes on one supported Windows PC and one supported Linux PC.

| Profile | Sample rate | Period | Required result |
| --- | ---: | ---: | --- |
| Safe | 48 kHz | 256 frames (5.33 ms) | zero deadline misses |
| Interactive | 48 kHz | 128 frames (2.67 ms) | zero deadline misses |
| Aggressive | 48 kHz | 64 frames (1.33 ms) | informational until Phase 2 |

For required profiles, `callback_count` must reach at least 90% of the nominal
count and `max_process_time_ms` must remain below 70% of `nominal_period_ms`.
The probe returns exit code 2 when these requirements or the allowed miss count
fail.

## Build and run

```bash
cmake -S . -B build/release -DCMAKE_BUILD_TYPE=Release
cmake --build build/release --config Release --parallel

./build/release/audio-engine/lartycc_latency_probe \
  --device DEVICE_ID --seconds 600 --sample-rate 48000 \
  --period-frames 128 --max-deadline-misses 0 \
  > qualification.json
```

On a multi-config Windows generator, the executable is normally below
`build/release/audio-engine/Release/`. Device IDs come from
`cargo run -p lartycc-desktop -- --list-devices`.

Validate the report against
`shared/schemas/realtime-qualification-v1.schema.json`. Commit accepted reports
under `docs/performance/results/` with the machine metadata and loopback method.
The roadmap checkbox remains open until all required rows and both operating
systems have accepted reports.

Bundle the two required reports with a manifest conforming to
`shared/schemas/reference-pc-result-v1.schema.json`, then verify paths, hashes,
profile settings and physical-machine metadata:

```bash
python scripts/validate_qualification_bundle.py path/to/manifest.json
```

Submit the validated files through the **Realtime reference-PC result** GitHub
issue template. A reviewer must reproduce the hash validation before accepting
the result into `docs/performance/results/`.

## Automated bundle collection

After measuring wired loopback, the cross-platform runner executes both required
10-minute profiles, writes reports through a temporary directory, computes their
hashes, creates the manifest, validates it and only then publishes the requested
output directory:

```bash
python scripts/run_qualification_bundle.py \
  --probe build/release/audio-engine/lartycc_latency_probe \
  --output-dir qualification-linux \
  --commit FULL_40_CHARACTER_GIT_SHA --os linux \
  --os-version "OS version" --cpu "CPU model" --ram-gib 16 \
  --power-profile performance --interface "Interface model" \
  --connection USB --driver-version "Driver version" \
  --device DEVICE_ID --loopback-method "wired output to input" \
  --loopback-sample-count 20 --loopback-median-ms 8.2 \
  --loopback-p95-ms 9.1
```

Use the Windows probe executable path and `--os windows` on Windows. Add
`--include-aggressive` to collect the optional 64-frame profile. The runner
refuses to overwrite an existing output directory.
