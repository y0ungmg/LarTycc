use lartycc_audio_bridge::AudioHost;
use lartycc_core::{Command, Project, ProjectSession};
use lartycc_desktop::HostRouter;
use std::error::Error;
use std::f32::consts::TAU;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        Some("--list-devices") => list_devices(),
        Some("--play-test") => play_test(arguments.next().as_deref()),
        Some("--host-stdio") => run_host_stdio(
            arguments
                .next()
                .as_deref()
                .map_or_else(|| Path::new("LarTycc-demo.json"), Path::new),
        ),
        Some(path) if !path.starts_with('-') => open_project(Path::new(path)),
        None => open_project(Path::new("LarTycc-demo.json")),
        Some(option) => Err(io::Error::other(format!("unknown option: {option}")).into()),
    }
}

fn run_host_stdio(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut router = HostRouter::open(path)?;
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    for line in stdin.lock().lines() {
        writeln!(stdout, "{}", router.invoke_json(&line?))?;
        stdout.flush()?;
    }
    Ok(())
}

fn list_devices() -> Result<(), Box<dyn Error>> {
    let mut host = AudioHost::new()?;
    let devices = host.devices()?;
    if devices.is_empty() {
        println!("No playback devices reported by the active audio backend.");
        return Ok(());
    }
    for device in devices {
        let default_marker = if device.is_default { " (default)" } else { "" };
        println!("{}\t{}{}", device.id, device.name, default_marker);
    }
    Ok(())
}

fn play_test(device_id: Option<&str>) -> Result<(), Box<dyn Error>> {
    const SAMPLE_RATE: u32 = 48_000;
    let mut host = AudioHost::new()?;
    let phase_step = TAU * 220.0 / 48_000.0;
    let mut phase = 0.0_f32;
    let sample = (0..48_000)
        .map(|_| {
            let value = phase.sin() * 0.2;
            phase += phase_step;
            value
        })
        .collect::<Vec<_>>();
    host.load_mono(&sample)?;
    host.start(device_id, SAMPLE_RATE, 128)?;
    host.play()?;
    std::thread::sleep(Duration::from_millis(1_100));
    host.stop();
    println!(
        "Played a 220 Hz test tone through {} ({} callbacks).",
        device_id.unwrap_or("the default device"),
        host.callback_count()
    );
    Ok(())
}

fn open_project(path: &Path) -> Result<(), Box<dyn Error>> {
    let project = if path.exists() {
        Project::load(path)
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
    session.project().save_atomic(path)?;
    println!(
        "LarTycc project '{}' · {} BPM · {} track(s) · saved to {}",
        session.project().id,
        session.project().tempo,
        session.project().tracks.len(),
        path.display()
    );
    Ok(())
}
