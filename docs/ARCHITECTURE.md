# Architecture

## Principles

1. **Two execution backends, one application.** Windows and Android Bedrock are separate runtime targets with independent lifecycle and compatibility constraints.
2. **Capability-driven UI.** Shared screens ask a backend for supported capabilities; they never assume every feature works on both targets.
3. **No bundled game content.** The launcher manages user-authorised installations and never redistributes proprietary game binaries, assets, or Marketplace content.
4. **Least privilege.** No root daemon is required. Runtime setup must use documented user-level tools and explain any external dependency.
5. **Data ownership.** Profiles, worlds, add-ons, and backups remain in user-controlled locations with explicit import/export operations.

## Layers

- **UI:** Qt/QML desktop shell; communicates with the Rust application service through a narrow IPC/API boundary.
- **Application service:** coordinates profiles, settings, tasks, progress, and capability discovery.
- **Domain core:** backend-neutral identifiers, manifests, validation, and service contracts.
- **Backend adapters:** Windows compatibility runtime and Android compatibility runtime.
- **Platform integration:** Secret Service, controller APIs, filesystem integration, and distribution packaging.

## Backend contract

Each backend must report:
- backend identity and detected runtime availability;
- supported capabilities and their limitations;
- installed game profiles and versions;
- preflight diagnostics before launch;
- structured progress and errors for long-running operations.

A launch request must identify a profile and backend explicitly. The application must never silently switch a user's selected backend.

## Data layout

Use XDG Base Directory conventions:
- configuration: `$XDG_CONFIG_HOME/tomos-bedrock-launcher`
- application state: `$XDG_STATE_HOME/tomos-bedrock-launcher`
- data and managed profiles: `$XDG_DATA_HOME/tomos-bedrock-launcher`
- cache and temporary downloads: `$XDG_CACHE_HOME/tomos-bedrock-launcher`

Resolve defaults when the relevant XDG variable is unset. Do not assume paths are always under `~/.local`.

## Long-running tasks

Downloads, imports, backups, and runtime setup must be cancellable where safe, report progress, verify checksums when available, and clean up partial temporary files after failure. Destructive operations require confirmation and should offer backup first.

## UI technology decision

Qt/QML is the current proposed desktop UI. Before adding UI dependencies, validate packaging availability and accessibility on all target distributions. Keep UI transport independent from the Rust domain model so the frontend can be changed without rewriting backend logic.
