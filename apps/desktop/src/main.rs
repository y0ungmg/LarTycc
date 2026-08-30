use lartycc_core::{Command, Project, ProjectSession};
use std::error::Error;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from("LarTycc-demo.json"), PathBuf::from);
    let project = if path.exists() {
        Project::load(&path)
            .map_err(|error| io::Error::other(format!("could not load project: {error:?}")))?
    } else {
        Project::new("local-project")
    };
    let mut session = ProjectSession::new(project);
    if session.project().tracks.is_empty() {
        session
            .apply(&Command::CreateTrack {
                id: 1,
                name: "Audio 1".into(),
            })
            .map_err(|error| io::Error::other(format!("could not create track: {error:?}")))?;
    }
    session.project().save_atomic(&path)?;
    println!(
        "LarTycc project '{}' · {} BPM · {} track(s) · saved to {}",
        session.project().id,
        session.project().tempo,
        session.project().tracks.len(),
        path.display()
    );
    Ok(())
}
