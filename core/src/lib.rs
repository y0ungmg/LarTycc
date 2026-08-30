//! Application-domain core for LarTycc.

/// Stable identifier used at command and persistence boundaries.
pub type EntityId = u128;

/// Commands are the only supported mutation boundary for projects.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    SetTempo { bpm: f64 },
    CreateTrack { id: EntityId, name: String },
    DeleteTrack { id: EntityId },
}
impl Command {
    /// Performs validation that is independent from the current project state.
    pub fn validate(&self) -> Result<(), CommandError> {
        match self {
            Self::SetTempo { bpm } if !(20.0..=400.0).contains(bpm) => {
                Err(CommandError::TempoOutOfRange)
            }
            Self::CreateTrack { name, .. } if name.trim().is_empty() => {
                Err(CommandError::EmptyTrackName)
            }
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CommandError {
    TempoOutOfRange,
    EmptyTrackName,
}

#[cfg(test)]
mod tests {
    use super::{Command, CommandError};

    #[test]
    fn rejects_invalid_tempo() {
        assert_eq!(
            Command::SetTempo { bpm: 500.0 }.validate(),
            Err(CommandError::TempoOutOfRange)
        );
    }

    #[test]
    fn accepts_valid_track() {
        assert!(Command::CreateTrack {
            id: 1,
            name: "Drums".into()
        }
        .validate()
        .is_ok());
    }
}
