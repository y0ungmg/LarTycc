//! Application-domain core for `LarTycc`.

use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

pub type EntityId = u128;

#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    pub id: EntityId,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: String,
    pub revision: u64,
    pub tempo: f64,
    pub tracks: Vec<Track>,
}

impl Project {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            revision: 0,
            tempo: 120.0,
            tracks: Vec::new(),
        }
    }

    #[must_use]
    pub fn to_json(&self) -> String {
        let mut tracks = String::new();
        for (index, track) in self.tracks.iter().enumerate() {
            if index > 0 {
                tracks.push(',');
            }
            let _ = write!(
                tracks,
                "{{\"id\":\"{}\",\"name\":\"{}\",\"kind\":\"audio\"}}",
                track.id,
                escape_json(&track.name)
            );
        }
        format!("{{\n  \"schemaVersion\": 1,\n  \"projectId\": \"{}\",\n  \"revision\": {},\n  \"tempo\": {},\n  \"tracks\": [{}]\n}}\n", escape_json(&self.id), self.revision, self.tempo, tracks)
    }

    /// Parses the strict Phase 1 project JSON subset.
    ///
    /// # Errors
    /// Returns [`ProjectError`] for malformed or unsupported data.
    pub fn from_json(input: &str) -> Result<Self, ProjectError> {
        if u64_field(input, "schemaVersion")? != 1 {
            return Err(ProjectError::UnsupportedSchema);
        }
        let id = string_field(input, "projectId")?;
        let revision = u64_field(input, "revision")?;
        let tempo = number_field(input, "tempo")?;
        if !tempo.is_finite() || !(20.0..=400.0).contains(&tempo) {
            return Err(ProjectError::InvalidFormat);
        }
        let tracks = object_items(array_field(input, "tracks")?)?
            .into_iter()
            .map(|object| {
                let id = string_field(object, "id")?
                    .parse::<EntityId>()
                    .map_err(|_| ProjectError::InvalidFormat)?;
                Ok(Track {
                    id,
                    name: string_field(object, "name")?,
                })
            })
            .collect::<Result<Vec<_>, ProjectError>>()?;
        Ok(Self {
            id,
            revision,
            tempo,
            tracks,
        })
    }

    /// Writes a complete project through a sibling temporary file.
    ///
    /// # Errors
    /// Propagates filesystem errors and removes an incomplete temporary file.
    pub fn save_atomic(&self, path: &Path) -> io::Result<()> {
        let temporary = temporary_path(path);
        let result = (|| {
            let mut file = File::create(&temporary)?;
            file.write_all(self.to_json().as_bytes())?;
            file.sync_all()?;
            fs::rename(&temporary, path)
        })();
        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        result
    }

    /// Loads and validates a project from disk.
    ///
    /// # Errors
    /// Returns I/O failures or validation errors.
    pub fn load(path: &Path) -> Result<Self, LoadError> {
        Self::from_json(&fs::read_to_string(path).map_err(LoadError::Io)?)
            .map_err(LoadError::Project)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    SetTempo { bpm: f64 },
    CreateTrack { id: EntityId, name: String },
    DeleteTrack { id: EntityId },
}

impl Command {
    /// Validates command-local invariants.
    ///
    /// # Errors
    /// Returns [`CommandError`] when a value is invalid.
    pub fn validate(&self) -> Result<(), CommandError> {
        match self {
            Self::SetTempo { bpm } if !bpm.is_finite() || !(20.0..=400.0).contains(bpm) => {
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
    TrackAlreadyExists,
    TrackNotFound,
    NothingToUndo,
    NothingToRedo,
}

pub struct ProjectSession {
    project: Project,
    undo: Vec<Command>,
    redo: Vec<Command>,
}

impl ProjectSession {
    #[must_use]
    pub const fn new(project: Project) -> Self {
        Self {
            project,
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    #[must_use]
    pub const fn project(&self) -> &Project {
        &self.project
    }

    /// Applies one validated command and records its inverse.
    ///
    /// # Errors
    /// Returns [`CommandError`] when validation or project invariants fail.
    pub fn apply(&mut self, command: &Command) -> Result<(), CommandError> {
        self.undo.push(apply_command(&mut self.project, command)?);
        self.redo.clear();
        Ok(())
    }

    /// Reverts the latest command.
    ///
    /// # Errors
    /// Returns [`CommandError::NothingToUndo`] when history is empty.
    pub fn undo(&mut self) -> Result<(), CommandError> {
        let command = self.undo.pop().ok_or(CommandError::NothingToUndo)?;
        self.redo.push(apply_command(&mut self.project, &command)?);
        Ok(())
    }

    /// Reapplies the latest reverted command.
    ///
    /// # Errors
    /// Returns [`CommandError::NothingToRedo`] when redo history is empty.
    pub fn redo(&mut self) -> Result<(), CommandError> {
        let command = self.redo.pop().ok_or(CommandError::NothingToRedo)?;
        self.undo.push(apply_command(&mut self.project, &command)?);
        Ok(())
    }

    /// Writes the current project to an autosave path.
    ///
    /// # Errors
    /// Propagates filesystem errors from [`Project::save_atomic`].
    pub fn autosave(&self, path: &Path) -> io::Result<()> {
        self.project.save_atomic(path)
    }
}

fn apply_command(project: &mut Project, command: &Command) -> Result<Command, CommandError> {
    command.validate()?;
    let inverse = match command {
        Command::SetTempo { bpm } => {
            let previous = project.tempo;
            project.tempo = *bpm;
            Command::SetTempo { bpm: previous }
        }
        Command::CreateTrack { id, name } => {
            if project.tracks.iter().any(|track| track.id == *id) {
                return Err(CommandError::TrackAlreadyExists);
            }
            project.tracks.push(Track {
                id: *id,
                name: name.clone(),
            });
            Command::DeleteTrack { id: *id }
        }
        Command::DeleteTrack { id } => {
            let index = project
                .tracks
                .iter()
                .position(|track| track.id == *id)
                .ok_or(CommandError::TrackNotFound)?;
            let track = project.tracks.remove(index);
            Command::CreateTrack {
                id: track.id,
                name: track.name,
            }
        }
    };
    project.revision = project.revision.saturating_add(1);
    Ok(inverse)
}

/// Reduces mono sample data to absolute peak buckets for waveform drawing.
#[must_use]
pub fn waveform_peaks(samples: &[f32], buckets: usize) -> Vec<f32> {
    if samples.is_empty() || buckets == 0 {
        return Vec::new();
    }
    let width = samples.len().div_ceil(buckets);
    samples
        .chunks(width)
        .take(buckets)
        .map(|chunk| {
            chunk
                .iter()
                .fold(0.0_f32, |peak, value| peak.max(value.abs()))
        })
        .collect()
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Project(ProjectError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectError {
    InvalidFormat,
    UnsupportedSchema,
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut value = path.as_os_str().to_owned();
    value.push(".tmp");
    PathBuf::from(value)
}
fn escape_json(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn key_tail<'a>(input: &'a str, key: &str) -> Result<&'a str, ProjectError> {
    input
        .find(&format!("\"{key}\""))
        .and_then(|index| {
            input[index..]
                .find(':')
                .map(|colon| &input[index + colon + 1..])
        })
        .ok_or(ProjectError::InvalidFormat)
}

fn number_field(input: &str, key: &str) -> Result<f64, ProjectError> {
    let tail = key_tail(input, key)?.trim_start();
    let end = tail
        .find(|character: char| !character.is_ascii_digit() && !matches!(character, '.' | '-'))
        .unwrap_or(tail.len());
    tail[..end]
        .parse::<f64>()
        .map_err(|_| ProjectError::InvalidFormat)
}

fn u64_field(input: &str, key: &str) -> Result<u64, ProjectError> {
    let tail = key_tail(input, key)?.trim_start();
    let end = tail
        .find(|character: char| !character.is_ascii_digit())
        .unwrap_or(tail.len());
    tail[..end]
        .parse::<u64>()
        .map_err(|_| ProjectError::InvalidFormat)
}

fn string_field(input: &str, key: &str) -> Result<String, ProjectError> {
    let tail = key_tail(input, key)?.trim_start();
    let body = tail.strip_prefix('"').ok_or(ProjectError::InvalidFormat)?;
    let (mut escaped, mut result) = (false, String::new());
    for character in body.chars() {
        if escaped {
            match character {
                '\\' | '"' => result.push(character),
                _ => return Err(ProjectError::InvalidFormat),
            }
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == '"' {
            return Ok(result);
        } else {
            result.push(character);
        }
    }
    Err(ProjectError::InvalidFormat)
}

fn array_field<'a>(input: &'a str, key: &str) -> Result<&'a str, ProjectError> {
    let body = key_tail(input, key)?
        .trim_start()
        .strip_prefix('[')
        .ok_or(ProjectError::InvalidFormat)?;
    Ok(&body[..body.rfind(']').ok_or(ProjectError::InvalidFormat)?])
}

fn object_items(input: &str) -> Result<Vec<&str>, ProjectError> {
    let (mut items, mut start, mut in_string, mut escaped) = (Vec::new(), None, false, false);
    for (index, character) in input.char_indices() {
        if escaped {
            escaped = false;
        } else if character == '\\' && in_string {
            escaped = true;
        } else if character == '"' {
            in_string = !in_string;
        } else if !in_string && character == '{' {
            if start.is_some() {
                return Err(ProjectError::InvalidFormat);
            }
            start = Some(index);
        } else if !in_string && character == '}' {
            let begin = start.take().ok_or(ProjectError::InvalidFormat)?;
            items.push(&input[begin..=index]);
        }
    }
    if start.is_some() || in_string {
        return Err(ProjectError::InvalidFormat);
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::{waveform_peaks, Command, Project, ProjectSession};
    use std::fs;

    #[test]
    fn commands_undo_and_redo() {
        let mut session = ProjectSession::new(Project::new("test"));
        session.apply(&Command::SetTempo { bpm: 145.0 }).unwrap();
        session
            .apply(&Command::CreateTrack {
                id: 1,
                name: "Drums".into(),
            })
            .unwrap();
        session.undo().unwrap();
        assert!(session.project().tracks.is_empty());
        session.redo().unwrap();
        assert_eq!(session.project().tracks[0].name, "Drums");
    }

    #[test]
    fn project_round_trips_and_saves_atomically() {
        let mut session = ProjectSession::new(Project::new("project-1"));
        session
            .apply(&Command::CreateTrack {
                id: 42,
                name: "Audio \\\"one\\\"".into(),
            })
            .unwrap();
        let parsed = Project::from_json(&session.project().to_json()).unwrap();
        assert_eq!(&parsed, session.project());
        let path = std::env::temp_dir().join(format!("lartycc-{}.json", std::process::id()));
        session.autosave(&path).unwrap();
        assert_eq!(Project::load(&path).unwrap(), parsed);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn computes_waveform_peaks() {
        assert_eq!(waveform_peaks(&[0.0, -0.5, 1.0, -0.25], 2), vec![0.5, 1.0]);
    }
}
