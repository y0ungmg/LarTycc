# Native desktop webview

This optional application embeds the production React build in Wry and routes
protocol-v1 requests directly to the Rust `HostRouter`. It is a separate Cargo
workspace so GTK/WebKit/WebView2 dependencies never enter the core lockfile or
default build.

The shell requires Rust 1.88 or newer. The main workspace keeps its Rust 1.81
minimum because it does not resolve this optional platform dependency graph.

Linux requires the WebKitGTK 4.1 development package. Windows uses the installed
Evergreen WebView2 Runtime.

```bash
npm ci
npm run build
cargo run --manifest-path apps/desktop-webview/Cargo.toml -- \
  demo-project.json ui/dist
```

The first argument is the project path. The second is the built UI directory;
when omitted it defaults to `ui/dist` or `LARTYCC_UI_DIST`.

The shell serves only canonical files below that directory through the private
`lartycc://` scheme. It rejects traversal, external navigation, new windows,
downloads, clipboard access, zoom hotkeys, and IPC messages over one MiB. A CSP
allows packaged scripts/styles and blocks network connections.
