#!/usr/bin/env python3
"""Run the required realtime profiles and create a validated result bundle."""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import tempfile
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

from validate_qualification_bundle import ValidationError, validate_bundle

PROFILE_PERIODS = {"safe": 256, "interactive": 128, "aggressive": 64}


def probe_command(
    executable: Path, device: str, seconds: int, period_frames: int
) -> list[str]:
    command = [
        str(executable),
        "--seconds",
        str(seconds),
        "--sample-rate",
        "48000",
        "--period-frames",
        str(period_frames),
        "--max-deadline-misses",
        "0",
    ]
    if device:
        command.extend(["--device", device])
    return command


def run_profile(
    executable: Path,
    output_dir: Path,
    profile: str,
    device: str,
    seconds: int,
) -> dict[str, str]:
    result = subprocess.run(
        probe_command(executable, device, seconds, PROFILE_PERIODS[profile]),
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        detail = result.stderr.strip() or result.stdout.strip() or "no diagnostic output"
        raise ValidationError(f"{profile} probe failed ({result.returncode}): {detail}")
    try:
        report = json.loads(result.stdout)
    except json.JSONDecodeError as error:
        raise ValidationError(f"{profile} probe returned invalid JSON: {error}") from error
    path = output_dir / f"{profile}.json"
    path.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    return {
        "name": profile,
        "report": path.name,
        "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
    }


def manifest(arguments: argparse.Namespace, profiles: list[dict[str, str]]) -> dict[str, Any]:
    return {
        "schema_version": 1,
        "recorded_at": datetime.now(UTC).isoformat().replace("+00:00", "Z"),
        "system": {
            "environment": "physical",
            "os": arguments.os,
            "os_version": arguments.os_version,
            "cpu": arguments.cpu,
            "ram_gib": arguments.ram_gib,
            "power_profile": arguments.power_profile,
        },
        "audio": {
            "interface": arguments.interface,
            "connection": arguments.connection,
            "backend": "alsa" if arguments.os == "linux" else "wasapi",
            "driver_version": arguments.driver_version,
        },
        "build": {"commit": arguments.commit, "build_type": "Release"},
        "profiles": profiles,
        "loopback": {
            "method": arguments.loopback_method,
            "sample_count": arguments.loopback_sample_count,
            "median_ms": arguments.loopback_median_ms,
            "p95_ms": arguments.loopback_p95_ms,
        },
    }


def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--probe", required=True, type=Path)
    parser.add_argument("--output-dir", required=True, type=Path)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--os", required=True, choices=("linux", "windows"))
    parser.add_argument("--os-version", required=True)
    parser.add_argument("--cpu", required=True)
    parser.add_argument("--ram-gib", required=True, type=float)
    parser.add_argument("--power-profile", required=True)
    parser.add_argument("--interface", required=True)
    parser.add_argument("--connection", required=True)
    parser.add_argument("--driver-version", required=True)
    parser.add_argument("--device", default="")
    parser.add_argument("--loopback-method", required=True)
    parser.add_argument("--loopback-sample-count", required=True, type=int)
    parser.add_argument("--loopback-median-ms", required=True, type=float)
    parser.add_argument("--loopback-p95-ms", required=True, type=float)
    parser.add_argument("--include-aggressive", action="store_true")
    return parser.parse_args()


def main() -> int:
    arguments = parse_arguments()
    probe = arguments.probe.resolve()
    output_dir = arguments.output_dir.resolve()
    if not probe.is_file():
        raise SystemExit(f"probe does not exist: {probe}")
    if output_dir.exists():
        raise SystemExit(f"output directory already exists: {output_dir}")
    output_dir.parent.mkdir(parents=True, exist_ok=True)

    profiles = ["safe", "interactive"]
    if arguments.include_aggressive:
        profiles.append("aggressive")
    temporary = Path(tempfile.mkdtemp(prefix="lartycc-qualification-", dir=output_dir.parent))
    try:
        entries = [run_profile(probe, temporary, name, arguments.device, 600) for name in profiles]
        manifest_path = temporary / "manifest.json"
        manifest_path.write_text(
            json.dumps(manifest(arguments, entries), indent=2) + "\n", encoding="utf-8"
        )
        validate_bundle(manifest_path)
        temporary.rename(output_dir)
    except (OSError, ValidationError) as error:
        shutil.rmtree(temporary, ignore_errors=True)
        raise SystemExit(str(error)) from error

    print(f"validated qualification bundle: {output_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
