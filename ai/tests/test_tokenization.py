import pytest
from lartycc_ai import NoteEvent, tokenize_note


def test_tokenizes_note_deterministically() -> None:
    assert tokenize_note(NoteEvent(60, 100, 240)) == (
        "NOTE_ON_60",
        "VELOCITY_24",
        "DURATION_240",
    )


def test_rejects_invalid_pitch() -> None:
    with pytest.raises(ValueError, match="pitch"):
        tokenize_note(NoteEvent(128, 100, 240))
