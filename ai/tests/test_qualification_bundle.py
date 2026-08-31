import hashlib
import json
import subprocess
import sys
from pathlib import Path

VALIDATOR = Path(__file__).parents[2] / "scripts" / "validate_qualification_bundle.py"


def write_report(root: Path, name: str, period: int) -> tuple[str, str]:
    path = root / f"{name}.json"
    path.write_text(
        json.dumps(
            {
                "schema_version": 1,
                "device_id": "device-id",
                "device_name": "Example interface",
                "sample_rate": 48_000,
                "period_frames": period,
                "nominal_period_ms": 1_000 * period / 48_000,
                "duration_seconds": 600,
                "expected_callbacks": 600 * 48_000 // period,
                "callback_count": 600 * 48_000 // period,
                "pass": True,
                "deadline_misses": 0,
                "max_callback_gap_ms": 1_000 * period / 48_000,
                "max_process_time_ms": 0.1,
            }
        ),
        encoding="utf-8",
    )
    return path.name, hashlib.sha256(path.read_bytes()).hexdigest()


def valid_manifest(root: Path) -> Path:
    safe_path, safe_hash = write_report(root, "safe", 256)
    interactive_path, interactive_hash = write_report(root, "interactive", 128)
    manifest = {
        "schema_version": 1,
        "recorded_at": "2026-08-31T12:00:00Z",
        "system": {
            "environment": "physical",
            "os": "linux",
            "os_version": "Example Linux 1",
            "cpu": "Example CPU",
            "ram_gib": 16,
            "power_profile": "performance",
        },
        "audio": {
            "interface": "Example USB interface",
            "connection": "USB",
            "backend": "alsa",
            "driver_version": "kernel 1",
        },
        "build": {"commit": "a" * 40, "build_type": "Release"},
        "profiles": [
            {"name": "safe", "report": safe_path, "sha256": safe_hash},
            {"name": "interactive", "report": interactive_path, "sha256": interactive_hash},
        ],
        "loopback": {
            "method": "wired output to input",
            "sample_count": 20,
            "median_ms": 8.2,
            "p95_ms": 9.1,
        },
    }
    path = root / "manifest.json"
    path.write_text(json.dumps(manifest), encoding="utf-8")
    return path


def test_accepts_complete_physical_bundle(tmp_path: Path) -> None:
    result = subprocess.run(
        [sys.executable, str(VALIDATOR), str(valid_manifest(tmp_path))],
        check=False,
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, result.stderr


def test_rejects_ci_and_tampered_reports(tmp_path: Path) -> None:
    manifest_path = valid_manifest(tmp_path)
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    manifest["system"]["environment"] = "ci"
    manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
    ci_result = subprocess.run(
        [sys.executable, str(VALIDATOR), str(manifest_path)],
        check=False,
        capture_output=True,
        text=True,
    )
    assert ci_result.returncode != 0
    assert "physical" in ci_result.stderr

    manifest["system"]["environment"] = "physical"
    manifest_path.write_text(json.dumps(manifest), encoding="utf-8")
    (tmp_path / "safe.json").write_text("{}", encoding="utf-8")
    tampered_result = subprocess.run(
        [sys.executable, str(VALIDATOR), str(manifest_path)],
        check=False,
        capture_output=True,
        text=True,
    )
    assert tampered_result.returncode != 0
    assert "SHA-256" in tampered_result.stderr
