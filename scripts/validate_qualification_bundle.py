#!/usr/bin/env python3
"""Validate a physical reference-PC qualification bundle without dependencies."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
from datetime import datetime
from pathlib import Path
from typing import Any

PROFILE_PERIODS = {"safe": 256, "interactive": 128, "aggressive": 64}
REQUIRED_PROFILES = {"safe", "interactive"}


class ValidationError(ValueError):
    """A qualification bundle is incomplete or internally inconsistent."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValidationError(message)


def require_keys(
    value: dict[str, Any], required: set[str], optional: set[str] | None = None
) -> None:
    optional = optional or set()
    missing = required - value.keys()
    unknown = value.keys() - required - optional
    require(not missing, f"missing fields: {', '.join(sorted(missing))}")
    require(not unknown, f"unknown fields: {', '.join(sorted(unknown))}")


def load_object(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ValidationError(f"cannot read JSON {path}: {error}") from error
    require(isinstance(value, dict), f"{path} must contain a JSON object")
    return value


def resolve_report(root: Path, relative: str) -> Path:
    require(bool(relative) and not Path(relative).is_absolute(), "report path must be relative")
    resolved = (root / relative).resolve()
    require(resolved.is_relative_to(root), "report path escapes the bundle directory")
    require(resolved.is_file(), f"report does not exist: {relative}")
    return resolved


def validate_probe_report(report: dict[str, Any], profile: str) -> None:
    require_keys(
        report,
        {
            "schema_version",
            "device_id",
            "device_name",
            "sample_rate",
            "period_frames",
            "nominal_period_ms",
            "duration_seconds",
            "expected_callbacks",
            "callback_count",
            "deadline_misses",
            "max_callback_gap_ms",
            "max_process_time_ms",
            "pass",
        },
    )
    require(report.get("schema_version") == 1, f"{profile}: unsupported report schema")
    require(bool(report.get("device_name")), f"{profile}: device name is required")
    require(report.get("sample_rate") == 48_000, f"{profile}: sample rate must be 48000")
    require(
        report.get("period_frames") == PROFILE_PERIODS[profile],
        f"{profile}: unexpected period size",
    )
    require(report.get("duration_seconds", 0) >= 600, f"{profile}: run must last 600 seconds")
    require(report.get("deadline_misses") == 0, f"{profile}: deadline misses are not zero")
    expected = report.get("expected_callbacks", 0)
    require(expected > 0, f"{profile}: expected callback count must be positive")
    require(
        report.get("callback_count", 0) >= expected * 9 // 10,
        f"{profile}: callback coverage is below 90%",
    )
    nominal_period = 1_000.0 * PROFILE_PERIODS[profile] / 48_000.0
    require(
        abs(report.get("nominal_period_ms", 0) - nominal_period) < 0.000_001,
        f"{profile}: nominal period is inconsistent",
    )
    require(
        0 <= report.get("max_process_time_ms", -1) <= nominal_period * 0.7,
        f"{profile}: callback processing exceeds 70% of the period",
    )
    require(report.get("max_callback_gap_ms", -1) >= 0, f"{profile}: callback gap is invalid")
    require(report.get("pass") is True, f"{profile}: probe did not pass")


def validate_bundle(manifest_path: Path) -> None:
    manifest_path = manifest_path.resolve()
    root = manifest_path.parent
    manifest = load_object(manifest_path)
    require_keys(
        manifest,
        {"schema_version", "recorded_at", "system", "audio", "build", "profiles", "loopback"},
        {"notes"},
    )
    require(manifest.get("schema_version") == 1, "unsupported manifest schema")
    try:
        recorded_at = datetime.fromisoformat(
            str(manifest.get("recorded_at", "")).replace("Z", "+00:00")
        )
    except ValueError as error:
        raise ValidationError("recorded_at must be an ISO 8601 date-time") from error
    require(recorded_at.tzinfo is not None, "recorded_at must include a timezone")

    system = manifest.get("system")
    require(isinstance(system, dict), "system metadata is required")
    require_keys(system, {"environment", "os", "os_version", "cpu", "ram_gib", "power_profile"})
    require(system.get("environment") == "physical", "only physical machines qualify")
    require(system.get("os") in {"linux", "windows"}, "unsupported operating system")
    for field in ("os_version", "cpu", "power_profile"):
        require(bool(system.get(field)), f"system.{field} is required")
    require(system.get("ram_gib", 0) > 0, "system.ram_gib must be positive")

    audio = manifest.get("audio")
    require(isinstance(audio, dict), "audio metadata is required")
    require_keys(audio, {"interface", "connection", "backend", "driver_version"})
    expected_backend = "alsa" if system["os"] == "linux" else "wasapi"
    require(audio.get("backend") == expected_backend, "audio backend does not match the OS")
    for field in ("interface", "connection", "driver_version"):
        require(bool(audio.get(field)), f"audio.{field} is required")

    build = manifest.get("build")
    require(isinstance(build, dict), "build metadata is required")
    require_keys(build, {"commit", "build_type"})
    require(build.get("build_type") == "Release", "qualification requires a Release build")
    require(
        re.fullmatch(r"[0-9a-f]{40}", str(build.get("commit", ""))) is not None,
        "build.commit must be a full lowercase Git SHA",
    )

    profiles = manifest.get("profiles")
    require(isinstance(profiles, list), "profiles must be an array")
    seen: set[str] = set()
    for entry in profiles:
        require(isinstance(entry, dict), "each profile must be an object")
        require_keys(entry, {"name", "report", "sha256"})
        profile = entry.get("name")
        require(profile in PROFILE_PERIODS, f"unknown profile: {profile}")
        require(profile not in seen, f"duplicate profile: {profile}")
        seen.add(profile)
        report_path = resolve_report(root, str(entry.get("report", "")))
        digest = hashlib.sha256(report_path.read_bytes()).hexdigest()
        require(digest == entry.get("sha256"), f"{profile}: SHA-256 mismatch")
        validate_probe_report(load_object(report_path), profile)
    require(REQUIRED_PROFILES <= seen, "safe and interactive profiles are required")

    loopback = manifest.get("loopback")
    require(isinstance(loopback, dict), "loopback measurements are required")
    require_keys(loopback, {"method", "sample_count", "median_ms", "p95_ms"})
    require(bool(loopback.get("method")), "loopback.method is required")
    require(loopback.get("sample_count", 0) >= 10, "loopback needs at least 10 samples")
    median = loopback.get("median_ms", 0)
    p95 = loopback.get("p95_ms", 0)
    require(median > 0 and p95 >= median, "loopback latency values are inconsistent")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("manifest", type=Path)
    arguments = parser.parse_args()
    try:
        validate_bundle(arguments.manifest)
    except ValidationError as error:
        parser.error(str(error))
    print(f"valid qualification bundle: {arguments.manifest}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
