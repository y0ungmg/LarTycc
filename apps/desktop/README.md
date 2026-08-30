# Desktop host

The Phase 1 native host is currently a small Rust project-lifecycle executable.
It creates or opens a validated project, routes initial edits through the command
system, and saves atomically. Run `cargo run -p lartycc-desktop -- project.json`.

The window/webview and typed React IPC binding remain open Phase 1 work. The host
does not claim hardware audio support until an ALSA/WASAPI backend is integrated.
