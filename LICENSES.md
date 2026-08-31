# Third-party licenses

LarTycc source code is licensed under Apache-2.0. Dependencies retain their
own licenses. Before a dependency, dataset, model, sample, preset, font, icon,
or other asset is merged, its provenance and redistribution terms must be
recorded here and pass the compatibility review described in CONTRIBUTING.md.

| Component | Version/source | License | Bundled? | Notes |
| --- | --- | --- | --- | --- |
| JUCE | Not yet selected | AGPLv3/commercial | No | Evaluation only; no JUCE code is bundled in Phase 0. |
| React | 19.x (planned) | MIT | No | Development dependency installed by contributors. |
| Rust crates | See Cargo.lock when generated | Per crate | No | Audit before each release. |
| Python packages | See lockfile when generated | Per package | No | Training-only packages are not shipped in the app. |
| miniaudio | 0.11.25 (`9634bed`) | MIT No Attribution | Built from pinned source | Device I/O only; selected from the upstream dual-license terms. |
| `cmake`, `cc`, `find-msvc-tools`, `shlex` Rust crates | Versions pinned in `Cargo.lock` | MIT OR Apache-2.0 | Build-time only | Compile and link the native audio bridge; not shipped as runtime libraries. |
| `serde`, `serde_json`, `itoa`, `proc-macro2`, `quote`, `serde_core`, `serde_derive`, `syn` | Versions pinned in `Cargo.lock` | MIT OR Apache-2.0 | Linked/build-time | Native-host JSON envelopes and derive support. |
| `memchr` | Version pinned in `Cargo.lock` | Unlicense OR MIT | Linked | Transitive `serde_json` parser dependency. |
| `unicode-ident` | Version pinned in `Cargo.lock` | (MIT OR Apache-2.0) AND Unicode-3.0 | Build-time | Transitive Rust macro identifier tables. |
| `zmij` | Version pinned in `Cargo.lock` | MIT | Linked | Transitive `serde_json` numeric conversion dependency. |
| Wry | 0.56.1 | Apache-2.0 OR MIT | Linked in optional desktop shell | OS webview wrapper; isolated in `apps/desktop-webview/Cargo.lock`. |
| Tao | 0.36.0 | Apache-2.0 | Linked in optional desktop shell | Cross-platform native window/event loop. |
| WebKitGTK | System 4.1 API | LGPL-2.1-or-later | No | Dynamically provided by Linux; required only for the optional desktop shell. |
| Microsoft Edge WebView2 Runtime | Evergreen system runtime | Microsoft license | No | Provided by Windows; LarTycc does not redistribute the runtime in this phase. |
| Desktop webview transitive Rust crates | Exact versions in `apps/desktop-webview/Cargo.lock` | Per crate | Optional | Separate lockfile and CI keep this graph out of the core workspace. |

The realtime qualification harness uses only the C++ standard library and the
already registered miniaudio device boundary; it introduces no new third-party
dependency.

No dataset, pretrained model, audio sample, or third-party asset is bundled in
Phase 0. A source being publicly downloadable does **not** make it suitable for
training or redistribution.
