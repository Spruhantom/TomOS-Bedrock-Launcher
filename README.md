# TomOS Bedrock Launcher

An independent Linux launcher project for Minecraft Bedrock Edition, designed around separate Windows and Android execution backends and shared launcher services.

> This project is unofficial and is not affiliated with, endorsed by, or sponsored by Mojang or Microsoft. It does not distribute Minecraft game files. Users must obtain game content through legitimate channels and comply with applicable platform terms.

## Project status

**Stage:** Architecture and repository foundation. No game installation, authentication, or launch functionality is implemented yet.

## Goals

- One native Linux launcher for Windows Bedrock and Android Bedrock execution paths.
- Shared version, add-on, world, controller, multiplayer, and settings experiences where backend capabilities permit.
- Support TomOS, Ubuntu, Debian, Fedora, Arch Linux, and Omarchy.
- Keep game files, credentials, and user data under the user's control.

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/BACKENDS.md](docs/BACKENDS.md), [docs/SECURITY.md](docs/SECURITY.md), and [docs/ROADMAP.md](docs/ROADMAP.md).

## Development

The Rust workspace is being established first. Backend integrations will remain behind capability-based interfaces and will not be enabled until their licensing, security, and compatibility have been reviewed.

## Disclaimer

Minecraft is a trademark of Microsoft/Mojang. This is an independent community project. Users are responsible for ensuring they have the rights to any game content they install or use.