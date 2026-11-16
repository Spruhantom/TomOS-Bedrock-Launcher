# Roadmap

## Stage 0 — Architecture and repository foundation
- [x] Establish project repository and Rust workspace.
- [x] Separate Windows and Android adapter crates.
- [x] Define initial security and architecture principles.
- [ ] Add CI checks and validate the workspace on Linux.
- [ ] Complete upstream licence and compatibility review.

## Stage 1 — Application foundation
- [ ] Define backend trait and capability negotiation.
- [ ] Add configuration and XDG path service.
- [ ] Add structured task/progress/error model.
- [ ] Build initial Qt/QML shell and backend selection screen.
- [ ] Add diagnostics and dependency detection.

## Stage 2 — Backend prototypes
- [ ] Windows runtime detection and safe preflight.
- [ ] Android runtime detection and safe preflight.
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
