# Roadmap

## Stage 0 — Architecture and repository foundation
- [x] Establish project repository and Rust workspace.
- [x] Separate Windows and Android adapter crates.
- [x] Define initial security and architecture principles.
- [x] Define capability-aware backend contract.
- [x] Add read-only Linux host and dependency detection.
- [x] Review Windows and Android upstream candidates.
- [ ] Add CI checks and validate the workspace on Linux.

## Stage 1 — Application foundation
- [ ] Add configuration and XDG path service.
- [ ] Add structured task/progress/error model.
- [ ] Build initial Qt/QML shell and backend selection screen.
- [ ] Add diagnostics presentation and safe repair guidance.

## Stage 2 — Backend prototypes
- [x] Windows runtime presence detection.
- [x] Android runtime presence detection.
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
