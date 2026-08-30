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

No dataset, pretrained model, audio sample, or third-party asset is bundled in
Phase 0. A source being publicly downloadable does **not** make it suitable for
training or redistribution.
