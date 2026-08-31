//! Versioned native-host command router shared by the desktop shell and tests.

use lartycc_audio_bridge::AudioHost;
use lartycc_core::{Command, Project, ProjectSession};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::f32::consts::TAU;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

pub mod webview_contract;

pub const HOST_PROTOCOL_VERSION: u32 = 1;
const DEFAULT_SAMPLE_RATE: u32 = 48_000;
const DEFAULT_PERIOD_FRAMES: u32 = 128;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostRequest {
    version: u32,
    id: String,
    command: String,
    #[serde(default)]
    payload: Value,
    expected_project_revision: Option<u64>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct HostResponse {
    version: u32,
    id: String,
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<HostFailure>,
}

#[derive(Debug, Serialize)]
struct HostFailure {
    code: String,
    message: String,
}

#[derive(Debug)]
pub struct HostError {
    code: &'static str,
    message: String,
}

impl HostError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl Display for HostError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl Error for HostError {}

pub struct HostRouter {
    session: ProjectSession,
    project_path: PathBuf,
    audio: AudioHost,
    sample_loaded: bool,
    sample_rate: u32,
}

impl HostRouter {
    /// Opens an existing project or creates an unsaved project at `path`.
    ///
    /// # Errors
    ///
    /// Returns an error when the project is invalid or the native audio host
    /// cannot be created.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, HostError> {
        let path = path.into();
        let project = if path.exists() {
            Project::load(&path)
                .map_err(|error| HostError::new("project_open_failed", format!("{error:?}")))?
        } else {
            Project::new("local-project")
        };
        let audio = AudioHost::new()
            .map_err(|error| HostError::new("audio_unavailable", error.to_string()))?;
        Ok(Self {
            session: ProjectSession::new(project),
            project_path: path,
            audio,
            sample_loaded: false,
            sample_rate: DEFAULT_SAMPLE_RATE,
        })
    }

    /// Dispatches one protocol-v1 JSON request and always returns JSON.
    #[must_use]
    pub fn invoke_json(&mut self, input: &str) -> String {
        let request = match serde_json::from_str::<HostRequest>(input) {
            Ok(request) => request,
            Err(error) => {
                return serialize_response(&HostResponse::failure(
                    "unknown",
                    HostError::new("invalid_request", error.to_string()),
                ));
            }
        };
        let id = request.id.clone();
        let response = match self.dispatch(&request) {
            Ok(result) => HostResponse::success(id, result),
            Err(error) => HostResponse::failure(id, error),
        };
        serialize_response(&response)
    }

    fn dispatch(&mut self, request: &HostRequest) -> Result<Value, HostError> {
        if request.version != HOST_PROTOCOL_VERSION {
            return Err(HostError::new(
                "unsupported_version",
                format!("expected protocol version {HOST_PROTOCOL_VERSION}"),
            ));
        }
        if request.id.trim().is_empty() {
            return Err(HostError::new(
                "invalid_request",
                "request id cannot be empty",
            ));
        }
        match request.command.as_str() {
            "host.getState" => Ok(self.state()),
            "audio.listDevices" => self.list_devices(),
            "audio.loadTestTone" => self.load_test_tone(),
            "transport.play" => self.play(request),
            "transport.stop" => {
                self.audio.stop();
                Ok(self.transport())
            }
            "transport.seek" => self.seek(&request.payload),
            "project.setTempo" => self.set_tempo(request),
            "project.createTrack" => self.create_track(request),
            "project.undo" => self.undo(request),
            "project.redo" => self.redo(request),
            "project.save" => self.save(request),
            command => Err(HostError::new(
                "unknown_command",
                format!("unsupported host command '{command}'"),
            )),
        }
    }

    fn list_devices(&mut self) -> Result<Value, HostError> {
        let devices = self
            .audio
            .devices()
            .map_err(|error| HostError::new("audio_device_error", error.to_string()))?;
        Ok(Value::Array(
            devices
                .into_iter()
                .map(|device| {
                    json!({
                        "id": device.id,
                        "name": device.name,
                        "isDefault": device.is_default,
                    })
                })
                .collect(),
        ))
    }

    fn load_test_tone(&mut self) -> Result<Value, HostError> {
        let phase_step = TAU * 220.0 / 48_000.0;
        let mut phase = 0.0_f32;
        let sample = (0..48_000)
            .map(|_| {
                let value = phase.sin() * 0.2;
                phase += phase_step;
                value
            })
            .collect::<Vec<_>>();
        self.audio
            .load_mono(&sample)
            .map_err(|error| HostError::new("sample_load_failed", error.to_string()))?;
        self.sample_loaded = true;
        self.sample_rate = DEFAULT_SAMPLE_RATE;
        Ok(json!({"frames": sample.len(), "sampleRate": self.sample_rate}))
    }

    fn play(&mut self, request: &HostRequest) -> Result<Value, HostError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Payload {
            device_id: Option<String>,
            #[serde(default = "default_sample_rate")]
            sample_rate: u32,
            #[serde(default = "default_period_frames")]
            period_frames: u32,
        }
        if !self.sample_loaded {
            return Err(HostError::new(
                "sample_not_loaded",
                "load audio before starting transport",
            ));
        }
        let payload: Payload = decode_payload(&request.payload)?;
        let selected = payload.device_id.as_deref().filter(|id| !id.is_empty());
        self.audio
            .start(selected, payload.sample_rate, payload.period_frames)
            .map_err(|error| HostError::new("audio_start_failed", error.to_string()))?;
        self.audio
            .play()
            .map_err(|error| HostError::new("transport_play_failed", error.to_string()))?;
        self.sample_rate = payload.sample_rate;
        Ok(self.transport())
    }

    fn seek(&mut self, payload: &Value) -> Result<Value, HostError> {
        #[derive(Deserialize)]
        struct Payload {
            frame: usize,
        }
        let payload: Payload = decode_payload(payload)?;
        self.audio
            .seek(payload.frame)
            .map_err(|error| HostError::new("transport_seek_failed", error.to_string()))?;
        Ok(self.transport())
    }

    fn set_tempo(&mut self, request: &HostRequest) -> Result<Value, HostError> {
        #[derive(Deserialize)]
        struct Payload {
            bpm: f64,
        }
        self.require_revision(request)?;
        let payload: Payload = decode_payload(&request.payload)?;
        self.session
            .apply(&Command::SetTempo { bpm: payload.bpm })
            .map_err(command_error)?;
        Ok(self.project())
    }

    fn create_track(&mut self, request: &HostRequest) -> Result<Value, HostError> {
        #[derive(Deserialize)]
        struct Payload {
            id: String,
            name: String,
        }
        self.require_revision(request)?;
        let payload: Payload = decode_payload(&request.payload)?;
        let id = payload
            .id
            .parse()
            .map_err(|_| HostError::new("invalid_payload", "track id must be a u128 string"))?;
        self.session
            .apply(&Command::CreateTrack {
                id,
                name: payload.name,
            })
            .map_err(command_error)?;
        Ok(self.project())
    }

    fn undo(&mut self, request: &HostRequest) -> Result<Value, HostError> {
        self.require_revision(request)?;
        self.session.undo().map_err(command_error)?;
        Ok(self.project())
    }

    fn redo(&mut self, request: &HostRequest) -> Result<Value, HostError> {
        self.require_revision(request)?;
        self.session.redo().map_err(command_error)?;
        Ok(self.project())
    }

    fn save(&self, request: &HostRequest) -> Result<Value, HostError> {
        self.require_revision(request)?;
        self.session
            .project()
            .save_atomic(&self.project_path)
            .map_err(|error| HostError::new("project_save_failed", error.to_string()))?;
        Ok(self.project())
    }

    fn require_revision(&self, request: &HostRequest) -> Result<(), HostError> {
        let expected = request.expected_project_revision.ok_or_else(|| {
            HostError::new(
                "revision_required",
                "mutating project commands require expectedProjectRevision",
            )
        })?;
        let actual = self.session.project().revision;
        if expected != actual {
            return Err(HostError::new(
                "revision_conflict",
                format!("expected revision {expected}, current revision is {actual}"),
            ));
        }
        Ok(())
    }

    fn state(&self) -> Value {
        json!({
            "project": self.project(),
            "transport": self.transport(),
            "sampleLoaded": self.sample_loaded,
            "audioAvailable": self.audio.is_available(),
        })
    }

    fn project(&self) -> Value {
        serde_json::from_str(&self.session.project().to_json())
            .unwrap_or_else(|_| json!({"revision": self.session.project().revision}))
    }

    fn transport(&self) -> Value {
        json!({
            "playing": self.audio.is_playing(),
            "positionFrames": self.audio.position(),
            "sampleRate": self.sample_rate,
            "callbackCount": self.audio.callback_count(),
        })
    }
}

impl HostResponse {
    fn success(id: String, result: Value) -> Self {
        Self {
            version: HOST_PROTOCOL_VERSION,
            id,
            ok: true,
            result: Some(result),
            error: None,
        }
    }

    fn failure(id: impl Into<String>, error: HostError) -> Self {
        Self {
            version: HOST_PROTOCOL_VERSION,
            id: id.into(),
            ok: false,
            result: None,
            error: Some(HostFailure {
                code: error.code.to_owned(),
                message: error.message,
            }),
        }
    }
}

fn decode_payload<T: for<'de> Deserialize<'de>>(payload: &Value) -> Result<T, HostError> {
    serde_json::from_value(payload.clone())
        .map_err(|error| HostError::new("invalid_payload", error.to_string()))
}

fn command_error(error: lartycc_core::CommandError) -> HostError {
    HostError::new("command_rejected", format!("{error:?}"))
}

const fn default_sample_rate() -> u32 {
    DEFAULT_SAMPLE_RATE
}

const fn default_period_frames() -> u32 {
    DEFAULT_PERIOD_FRAMES
}

fn serialize_response(response: &HostResponse) -> String {
    serde_json::to_string(response).unwrap_or_else(|_| {
        "{\"version\":1,\"id\":\"unknown\",\"ok\":false,\"error\":{\"code\":\"serialization_failed\",\"message\":\"response serialization failed\"}}".to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::{HostRouter, HOST_PROTOCOL_VERSION};
    use serde_json::Value;

    fn invoke(router: &mut HostRouter, request: &str) -> Value {
        serde_json::from_str(&router.invoke_json(request)).expect("response JSON")
    }

    #[test]
    fn protocol_routes_project_command_and_detects_stale_revision() {
        let mut router = HostRouter::open("protocol-test-project.json").expect("router");
        let applied = invoke(
            &mut router,
            r#"{"version":1,"id":"tempo-1","command":"project.setTempo","payload":{"bpm":148.0},"expectedProjectRevision":0}"#,
        );
        assert_eq!(applied["ok"], true);
        assert_eq!(applied["result"]["tempo"], 148.0);
        assert_eq!(applied["result"]["revision"], 1);

        let conflict = invoke(
            &mut router,
            r#"{"version":1,"id":"tempo-2","command":"project.setTempo","payload":{"bpm":120.0},"expectedProjectRevision":0}"#,
        );
        assert_eq!(conflict["ok"], false);
        assert_eq!(conflict["error"]["code"], "revision_conflict");
    }

    #[test]
    fn protocol_exposes_audio_boundary_without_starting_hardware() {
        let mut router = HostRouter::open("audio-protocol-test.json").expect("router");
        let devices = invoke(
            &mut router,
            r#"{"version":1,"id":"devices-1","command":"audio.listDevices"}"#,
        );
        assert_eq!(devices["version"], HOST_PROTOCOL_VERSION);
        assert_eq!(devices["id"], "devices-1");
        assert_eq!(devices["ok"], true);
        assert!(devices["result"].is_array());

        let tone = invoke(
            &mut router,
            r#"{"version":1,"id":"tone-1","command":"audio.loadTestTone"}"#,
        );
        assert_eq!(tone["ok"], true);
        assert_eq!(tone["result"]["frames"], 48_000);
    }

    #[test]
    fn protocol_rejects_unknown_versions_and_commands() {
        let mut router = HostRouter::open("version-protocol-test.json").expect("router");
        let version = invoke(
            &mut router,
            r#"{"version":2,"id":"future","command":"host.getState"}"#,
        );
        assert_eq!(version["error"]["code"], "unsupported_version");

        let command = invoke(
            &mut router,
            r#"{"version":1,"id":"unknown","command":"project.explode"}"#,
        );
        assert_eq!(command["error"]["code"], "unknown_command");
    }

    #[test]
    fn shared_protocol_schema_is_valid_json() {
        let schema = include_str!("../../../shared/schemas/host-protocol-v1.schema.json");
        let parsed: Value = serde_json::from_str(schema).expect("host protocol schema");
        assert_eq!(
            parsed["$defs"]["request"]["properties"]["version"]["const"],
            1
        );

        let qualification =
            include_str!("../../../shared/schemas/realtime-qualification-v1.schema.json");
        let parsed: Value = serde_json::from_str(qualification).expect("qualification schema");
        assert_eq!(parsed["properties"]["schema_version"]["const"], 1);

        let reference = include_str!("../../../shared/schemas/reference-pc-result-v1.schema.json");
        let parsed: Value = serde_json::from_str(reference).expect("reference PC schema");
        assert_eq!(
            parsed["properties"]["system"]["properties"]["environment"]["const"],
            "physical"
        );
    }
}
