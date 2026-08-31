import json
import subprocess
import sys
from pathlib import Path

RUNNER = Path(__file__).parents[2] / "scripts" / "run_qualification_bundle.py"
VALIDATOR = Path(__file__).parents[2] / "scripts" / "validate_qualification_bundle.py"


def fake_probe(path: Path) -> Path:
    script = path / "fake_probe.py"
    script.write_text(
        """#!/usr/bin/env python3
import json, sys
arguments = dict(zip(sys.argv[1::2], sys.argv[2::2], strict=True))
period = int(arguments["--period-frames"])
seconds = int(arguments["--seconds"])
expected = seconds * 48000 // period
print(json.dumps({
    "schema_version": 1,
    "device_id": "device-id",
    "device_name": "Fake interface",
    "sample_rate": 48000,
    "period_frames": period,
    "nominal_period_ms": 1000 * period / 48000,
    "duration_seconds": seconds,
    "expected_callbacks": expected,
    "callback_count": expected,
    "deadline_misses": 0,
    "max_callback_gap_ms": 1000 * period / 48000,
    "max_process_time_ms": 0.1,
    "pass": True,
}))
""",
        encoding="utf-8",
    )
    script.chmod(0o755)
    return script


def test_runner_creates_a_valid_hashed_bundle(tmp_path: Path) -> None:
    output = tmp_path / "bundle"
    result = subprocess.run(
        [
            sys.executable,
            str(RUNNER),
            "--probe",
            str(fake_probe(tmp_path)),
            "--output-dir",
            str(output),
            "--commit",
            "b" * 40,
            "--os",
            "linux",
            "--os-version",
            "Example Linux",
            "--cpu",
            "Example CPU",
            "--ram-gib",
            "16",
            "--power-profile",
            "performance",
            "--interface",
            "Fake interface",
            "--connection",
            "USB",
            "--driver-version",
            "kernel 1",
            "--loopback-method",
            "wired",
            "--loopback-sample-count",
            "20",
            "--loopback-median-ms",
            "8.2",
            "--loopback-p95-ms",
            "9.1",
        ],
        check=False,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, result.stderr
    assert {path.name for path in output.iterdir()} == {
        "manifest.json",
        "safe.json",
        "interactive.json",
    }
    manifest = json.loads((output / "manifest.json").read_text(encoding="utf-8"))
    assert {profile["name"] for profile in manifest["profiles"]} == {"safe", "interactive"}

    validation = subprocess.run(
        [sys.executable, str(VALIDATOR), str(output / "manifest.json")],
        check=False,
        capture_output=True,
        text=True,
    )
    assert validation.returncode == 0, validation.stderr
