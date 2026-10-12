# The brand layer

Atelier Ledger is GridCraft with a different name on the door. We keep the change small on purpose:
upstream lands many changes a day, and every file we touch is a file that can conflict when we pull
them in. So:

**Renamed (what a person sees):** the window title and app menu, the Help menu and About (where
Ledger comes from, the Nation, this repository), the "About Atelier Ledger" ribbon button in every
language, message and data-validation dialog titles, messages in the UI crate and the apps, the PDF
Creator and Producer, Windows file properties, the MCP server title, instructions and screenshot
tool text, installer and package names, vendor, links and "Open with" names, the browser page, the
app icon, and where settings and logs live (`~/Library/Application Support/Atelier Ledger`,
`%APPDATA%\Atelier Ledger`).

**Not renamed (machine names):** crate names (`gridcraft-*`), the binaries (`gridcraft`,
`gridcraft-cli`), command ids, the `ai.storyteller.gridcraft` app id and icon file names in
packaging, the eframe app name, the default author name "GridCraft User", the Application tag in
saved .xlsx files, `INFO("osversion")`, the Windows ProgIds (`GridCraft.Workbook`,
`GridCraft.TextData`), registry keys and upgrade code, environment variables (`GRIDCRAFT_*`), the
Linux settings folder, and the docs under `docs/`.

## Where the name lives

| Place | File |
|---|---|
| English UI strings | literal `"Atelier Ledger"` in `crates/ui-egui/src` and `apps/` |
| Translated "About" button | `crates/ui-egui/src/i18n.rs` and `i18n/{ko,pt,ru,zh}.rs` |
| About and Help links | `crates/ui-egui/src/dialogs.rs` (`about`), `apps/gridcraft/src/native_menu.rs` |
| Settings and log folders, window title | `apps/gridcraft/src/main.rs` |
| PDF Creator and Producer | `crates/pdf/src/lib.rs` |
| MCP and CLI | `crates/mcp/src/{server,backend,tools}.rs`, `apps/gridcraft-cli/src/main.rs` |
| Windows file properties | `apps/gridcraft/build.rs` |
| macOS bundle | `packaging/macos/Info.plist.in` (display name, copyright; identifier unchanged) |
| Windows installer | `packaging/windows/gridcraft.wxs` (names, manufacturer, links, setup dialogs, install folder, shortcuts, Open With names; ProgIds, registry keys and upgrade code unchanged), with its checks in `test-desktop-shortcut.ps1`, `preview-installer-dialogs.ps1`, `.github/workflows/windows-arm64.yml`, and the signing description in `sign.ps1` |
| Linux desktop entry, AppStream, nfpm | `packaging/linux/ai.storyteller.gridcraft.*`, `nfpm.yaml` |
| Browser build | `apps/gridcraft-web/index.html`, `apps/gridcraft-web/src/web.rs` |
| README and NOTICE | README: our header above upstream's feature sections; NOTICE: ours on top of theirs, kept on merge by `.gitattributes` |
| App icon | `assets/app-icon/` (ours: `gridcraft.svg` is the master; rows in `ATTRIBUTION.md`) |

Still upstream's name, for later passes: messages from the engine crates, and the `docs/` manuals.
For the packaging pass (real installers): the macOS `.app` bundle and DMG names
(`packaging/macos/package.sh`), the DMG background, the package scripts' file names, the Linux
package maintainer, the AppStream brand colours, the release titles in `release.yml`, and the
Flatpak repository page titles in `flatpak-pages.yml`.
