# TomOS Bedrock Launcher

An independent Linux launcher project for Minecraft Bedrock Edition, designed around separate Windows and Android execution backends and shared launcher services.

> This project is unofficial and is not affiliated with, endorsed by, or sponsored by Mojang or Microsoft. It does not distribute Minecraft game files. Users must obtain game content through legitimate channels and comply with applicable platform terms.

## Project status

**Stage:** Rust workspace, capability-aware backend adapters, read-only host detection, XDG path resolution, dashboard model, and Qt 6 desktop shell. The desktop shell now invokes a small Rust service at startup and displays the detected Linux host and whether the two upstream runtime executables are present. This is presence detection only: it does not prove that a game version is compatible or launchable.

## Goals

- One native Linux launcher for Windows Bedrock and Android Bedrock execution paths.
- Shared version, add-on, world, controller, multiplayer, and settings experiences where backend capabilities permit.
- Support TomOS, Ubuntu, Debian, Fedora, Arch Linux, and Omarchy.
- Keep game files, credentials, and user data under the user's control.

## Desktop application

See [app/README.md](app/README.md) for Qt 6 and Rust build requirements. CMake builds the Rust dashboard service and the Qt Quick UI. The Play button remains disabled until profiles, preflight checks, and the launch lifecycle are implemented.

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/BACKENDS.md](docs/BACKENDS.md), [docs/SECURITY.md](docs/SECURITY.md), [docs/UPSTREAM_REVIEW.md](docs/UPSTREAM_REVIEW.md), and [docs/ROADMAP.md](docs/ROADMAP.md).

## Disclaimer

Minecraft is a trademark of Microsoft/Mojang. This is an independent community project. Users are responsible for ensuring they have the rights to any game content they install or use.
