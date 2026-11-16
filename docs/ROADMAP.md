# Roadmap

## Stage 0 — Architecture and repository foundation
- [x] Establish project repository and Rust workspace.
- [x] Separate Windows and Android adapter crates.
- [x] Define initial security and architecture principles.
- [x] Define capability-aware backend contract.
- [x] Add read-only Linux host and dependency detection.
- [x] Review Windows and Android upstream candidates.
- [ ] Add CI checks and validate the complete workspace on Linux.

## Stage 1 — Application foundation
- [x] Add read-only XDG configuration/data/cache path resolution.
- [x] Add dashboard snapshot model for host and backend status.
- [x] Add Qt 6 / Qt Quick desktop shell prototype.
- [x] Add backend selection presentation and disabled Play state.
- [x] Add a Rust dashboard service exposing read-only host and backend probes as JSON.
- [x] Connect the Qt application to the Rust service at startup.
- [x] Display detected distribution, architecture, session type and runtime presence.
- [ ] Add structured task/progress/error model.
- [ ] Add live refresh and detailed diagnostics presentation.

## Stage 2 — Backend prototypes
- [x] Windows runtime executable presence detection.
- [x] Android runtime executable presence detection.
- [ ] Verify Windows upstream command-line and process integration.
- [ ] Verify Android runtime launch and process integration.
- [ ] Prove launch of user-installed game content on each backend.
- [ ] Document unsupported services and known limitations.

## Stage 3 — Shared game management
- [ ] Version/profile manager.
- [ ] Add-on import, validation, enable/disable, and removal.
- [ ] World listing, backup, restore, and export.
- [ ] Controller detection and mapping.
- [ ] Multiplayer server list and direct connection where supported.
- [ ] Realms compatibility investigation and testing.

## Stage 4 — Distribution and release
- [ ] Ubuntu/Debian packages.
- [ ] Fedora RPM.
- [ ] Arch/Omarchy PKGBUILD.
- [ ] Flatpak and AppImage feasibility review.
- [ ] TomOS integration.
- [ ] Reproducible builds, release signing, and upgrade tests.

## Release gates

No backend is marked supported until clean-install, launch, update, account, save-data, controller, networking, backup/restore, and uninstall tests have been run and recorded on a documented hardware/software matrix.
