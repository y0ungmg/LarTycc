# Desktop webview third-party licenses

This optional workspace has its own lockfile so its platform dependency graph
can be audited and released independently from the LarTycc core workspace.
Every package resolved in `Cargo.lock` declares a license expression; the audit
for the locked graph found no package with missing license metadata.

| License family | Notable packages |
| --- | --- |
| Apache-2.0 | Tao and platform support crates |
| Apache-2.0 OR MIT | Wry, serde, HTTP and Rust platform crates |
| MIT | GTK/WebKit bindings and utility crates |
| MPL-2.0 | `cssparser`, `selectors`, `option-ext` |
| Unicode-3.0 | ICU and Unicode data crates |
| Zlib | `foldhash` and Objective-C support crates |
| Unlicense OR MIT | `memchr`, `same-file`, `walkdir`, `winapi-util` |

Exact package names and versions are pinned in `Cargo.lock`. License texts and
notices required for a distributable build must be generated from that exact
lockfile during packaging; this phase does not redistribute Rust source crates.

The Linux shell dynamically uses the system WebKitGTK 4.1 libraries
(LGPL-2.1-or-later). The Windows shell uses the system Microsoft Edge WebView2
Evergreen Runtime under Microsoft's terms. Neither runtime is bundled here.
