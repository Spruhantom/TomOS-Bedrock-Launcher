# TomOS Bedrock Launcher

An independent Linux launcher project for Minecraft Bedrock Edition, designed around separate Windows and Android execution backends and shared launcher services.

> This project is unofficial and is not affiliated with, endorsed by, or sponsored by Mojang or Microsoft. It does not distribute Minecraft game files. Users must obtain game content through legitimate channels and comply with applicable platform terms.

## Project status

**Stage:** Rust architecture, read-only backend detection, XDG path resolution, dashboard data model, and an early Qt 6 desktop UI prototype. The UI is not yet connected to the Rust service. Game installation, authentication, and launch are not implemented.

## Goals

- One native Linux launcher for Windows Bedrock and Android Bedrock execution paths.
- Shared version, add-on, world, controller, multiplayer, and settings experiences where backend capabilities permit.
- Support TomOS, Ubuntu, Debian, Fedora, Arch Linux, and Omarchy.
- Keep game files, credentials, and user data under the user's control.

## Desktop UI prototype

See [app/README.md](app/README.md) for Qt 6 build instructions. The current interface demonstrates backend selection and planned navigation; Play remains disabled until real runtime preflight and launch services exist.

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/BACKENDS.md](docs/BACKENDS.md), [docs/SECURITY.md](docs/SECURITY.md), [docs/UPSTREAM_REVIEW.md](docs/UPSTREAM_REVIEW.md), and [docs/ROADMAP.md](docs/ROADMAP.md).

## Disclaimer

Minecraft is a trademark of Microsoft/Mojang. This is an independent community project. Users are responsible for ensuring they have the rights to any game content they install or use.
