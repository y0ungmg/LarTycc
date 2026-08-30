"""Deterministic symbolic-event tokenization primitives."""

from dataclasses import dataclass


@dataclass(frozen=True)
class NoteEvent:
    pitch: int
    velocity: int
    duration_ticks: int


def tokenize_note(note: NoteEvent) -> tuple[str, str, str]:
    """Convert a validated MIDI note to the proposed v0 token vocabulary."""
    if not 0 <= note.pitch <= 127:
        raise ValueError("pitch must be in [0, 127]")
    if not 1 <= note.velocity <= 127:
        raise ValueError("velocity must be in [1, 127]")
    if note.duration_ticks <= 0:
        raise ValueError("duration_ticks must be positive")
    velocity_bin = min(31, (note.velocity - 1) // 4)
    return (
        f"NOTE_ON_{note.pitch}",
        f"VELOCITY_{velocity_bin}",
        f"DURATION_{note.duration_ticks}",
    )

