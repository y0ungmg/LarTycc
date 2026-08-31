# ADR 0004: Wry/Tao for the optional desktop shell

- Status: Accepted
- Date: 2026-08-31

## Context

The React interface and Rust protocol-v1 router were independently testable but
not connected in a real window. LarTycc targets Linux and Windows and should use
the operating-system webview without shipping a second browser engine. Webview
system packages must not become prerequisites for the core workspace.

## Decision

Use Wry 0.56.1 with Tao 0.36.0 in the standalone
`apps/desktop-webview` Cargo workspace. Linux uses WebKitGTK 4.1 and Windows uses
the Evergreen WebView2 Runtime. The application loads the Vite production build
through a private custom protocol and injects a minimal `window.lartyccHost`
adapter before React starts.

IPC forwards the existing JSON envelope unchanged to `HostRouter`. Responses
are parsed as JSON before being inserted into one controlled JavaScript callback.
The shell blocks path traversal, external navigation, popups, downloads,
clipboard access, and messages larger than one MiB. Content is served with a
restrictive CSP and `nosniff`.

## Alternatives

- Tauri provides packaging and commands but would duplicate the existing router
  and introduce broader framework ownership before installer work begins.
- Electron would bundle Chromium and materially increase distribution size.
- Direct WebKitGTK/WebView2 bindings would duplicate lifecycle and platform code.
- Keeping Wry optional inside the core workspace still expanded the root
  lockfile by roughly 250 platform packages, so a separate workspace was chosen.

## Consequences

- The core build and lockfile remain independent of webview system libraries.
- The optional shell currently requires Rust 1.88 because of its separately
  locked platform dependency graph; the core workspace remains on Rust 1.81.
- Webview CI is a separate Linux/Windows matrix and part of the quality gate.
- Contributors build React assets before launching the window in this phase.
- Asset embedding, signing, installers, updates, and runtime availability checks
  remain packaging work; they do not change the host protocol.
