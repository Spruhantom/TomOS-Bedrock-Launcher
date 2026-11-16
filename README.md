# TomOS Bedrock Launcher

An independent Linux launcher project for Minecraft Bedrock Edition, designed around separate Windows and Android execution backends and shared launcher services.

> This project is unofficial and is not affiliated with, endorsed by, or sponsored by Mojang or Microsoft. It does not distribute Minecraft game files. Users must obtain game content through legitimate channels and comply with applicable platform terms.

## Project status

**Stage:** Core architecture, backend detection, XDG path resolution, and dashboard data model. There is not yet a graphical interface or working game installation/authentication/launch flow.

## Goals

- One native Linux launcher for Windows Bedrock and Android Bedrock execution paths.
- Shared version, add-on, world, controller, multiplayer, and settings experiences where backend capabilities permit.
- Support TomOS, Ubuntu, Debian, Fedora, Arch Linux, and Omarchy.
- Keep game files, credentials, and user data under the user's control.

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/BACKENDS.md](docs/BACKENDS.md), [docs/SECURITY.md](docs/SECURITY.md), [docs/UPSTREAM_REVIEW.md](docs/UPSTREAM_REVIEW.md), and [docs/ROADMAP.md](docs/ROADMAP.md).

## Development

The Rust workspace is being established first. Backend integrations remain behind capability-based interfaces. Host detection is read-only; XDG paths are resolved without creating directories until the application explicitly requests it.

## Disclaimer

Minecraft is a trademark of Microsoft/Mojang. This is an independent community project. Users are responsible for ensuring they have the rights to any game content they install or use.
