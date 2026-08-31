//! Toolkit-independent constants and validation for the native webview seam.

use serde_json::Value;
use std::path::Path;

pub const MAX_IPC_BYTES: usize = 1_048_576;
pub const APP_SCHEME: &str = "lartycc";
pub const APP_URL: &str = "lartycc://localhost/index.html";

pub const INITIALIZATION_SCRIPT: &str = r#"
(() => {
  const pending = new Map();
  const transportListeners = new Set();

  window.__lartyccHostResolve = (response) => {
    const resolve = pending.get(response.id);
    if (resolve) {
      pending.delete(response.id);
      resolve(response);
    }
    const snapshot = response?.ok && response.result?.transport
      ? response.result.transport
      : response?.ok && typeof response.result?.playing === "boolean"
        ? response.result
        : undefined;
    if (snapshot) transportListeners.forEach((listener) => listener(snapshot));
  };

  window.lartyccHost = Object.freeze({
    invoke(request) {
      return new Promise((resolve) => {
        const message = JSON.stringify(request);
        if (new TextEncoder().encode(message).byteLength > 1048576) {
          resolve({
            version: 1,
            id: request.id,
            ok: false,
            error: {
              code: "request_too_large",
              message: "host request exceeds 1048576 bytes",
            },
          });
          return;
        }
        pending.set(request.id, resolve);
        window.ipc.postMessage(message);
      });
    },
    onTransport(listener) {
      transportListeners.add(listener);
      return () => transportListeners.delete(listener);
    },
  });
})();
"#;

#[must_use]
pub fn asset_relative_path(uri_path: &str) -> Option<&str> {
    let relative = uri_path.trim_start_matches('/');
    if relative.is_empty() {
        return Some("index.html");
    }
    if relative.contains('\\')
        || relative.contains('%')
        || relative
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return None;
    }
    Some(relative)
}

#[must_use]
pub fn content_type(path: &str) -> &'static str {
    let extension = Path::new(path).extension().and_then(|value| value.to_str());
    match extension {
        Some(value) if value.eq_ignore_ascii_case("html") => "text/html; charset=utf-8",
        Some(value) if value.eq_ignore_ascii_case("js") => "text/javascript; charset=utf-8",
        Some(value) if value.eq_ignore_ascii_case("css") => "text/css; charset=utf-8",
        Some(value) if value.eq_ignore_ascii_case("svg") => "image/svg+xml",
        Some(value) if value.eq_ignore_ascii_case("png") => "image/png",
        Some(value) if value.eq_ignore_ascii_case("woff2") => "font/woff2",
        _ => "application/octet-stream",
    }
}

/// Builds the single JavaScript call used to return a native response.
///
/// # Errors
///
/// Returns a JSON parse error rather than embedding malformed content as code.
pub fn resolution_script(response_json: &str) -> Result<String, serde_json::Error> {
    let response: Value = serde_json::from_str(response_json)?;
    Ok(format!("window.__lartyccHostResolve?.({response});"))
}

#[cfg(test)]
mod tests {
    use super::{asset_relative_path, content_type, resolution_script, INITIALIZATION_SCRIPT};

    #[test]
    fn asset_paths_are_relative_and_cannot_traverse() {
        assert_eq!(asset_relative_path("/"), Some("index.html"));
        assert_eq!(asset_relative_path("/assets/app.js"), Some("assets/app.js"));
        for invalid in [
            "/../secret",
            "/assets/../../secret",
            "/a\\b",
            "/%2e%2e/secret",
        ] {
            assert_eq!(asset_relative_path(invalid), None);
        }
    }

    #[test]
    fn bridge_and_response_script_use_the_controlled_global() {
        assert!(INITIALIZATION_SCRIPT.contains("window.lartyccHost"));
        assert!(INITIALIZATION_SCRIPT.contains("new TextEncoder()"));
        assert!(INITIALIZATION_SCRIPT.contains("request_too_large"));
        let script = resolution_script(r#"{"version":1,"id":"one","ok":true,"result":{}}"#)
            .expect("valid response");
        assert!(script.starts_with("window.__lartyccHostResolve?.("));
        assert!(resolution_script("not-json").is_err());
        assert_eq!(
            content_type("assets/app.js"),
            "text/javascript; charset=utf-8"
        );
    }
}
