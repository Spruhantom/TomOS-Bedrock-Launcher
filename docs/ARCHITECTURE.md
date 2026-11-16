# Architecture

## Principles

1. **Two execution backends, one application.** Windows and Android Bedrock are separate runtime targets with independent lifecycle and compatibility constraints.
2. **Capability-driven UI.** Shared screens ask a backend for supported capabilities; they never assume every feature works on both targets.
3. **No bundled game content.** The launcher manages user-authorised installations and never redistributes proprietary game binaries, assets, or Marketplace content.
4. **Least privilege.** No root daemon is required. Runtime setup must use documented user-level tools and explain any external dependency.
5. **Data ownership.** Profiles, worlds, add-ons, and backups remain in user-controlled locations with explicit import/export operations.

## Layers

- **Desktop UI:** Qt 6 / Qt Quick Controls shell in `app/`. QML owns presentation and transient view state only.
- **Application service:** Rust service coordinates profiles, settings, tasks, progress, diagnostics, and capability discovery.
- **Domain core:** backend-neutral identifiers, manifests, validation, and service contracts.
- **Backend adapters:** Windows compatibility runtime and Android compatibility runtime.
- **Platform integration:** Secret Service, controller APIs, filesystem integration, and distribution packaging.

The UI prototype is intentionally not connected to runtime launch services yet. A narrow, typed bridge will be added after the application service API and task lifecycle are stable; QML must not shell out to game runtimes directly.

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
- state: `$XDG_STATE_HOME/tomos-bedrock-launcher`
- data and managed profiles: `$XDG_DATA_HOME/tomos-bedrock-launcher`
- cache and temporary downloads: `$XDG_CACHE_HOME/tomos-bedrock-launcher`

Resolve defaults when the relevant XDG variable is unset. Do not assume paths are always under `~/.local`. The current Rust path service resolves config, data, and cache paths without creating directories. State-directory resolution remains to be added.

## Long-running tasks

Downloads, imports, backups, and runtime setup must be cancellable where safe, report progress, verify checksums when available, and clean up partial temporary files after failure. Destructive operations require confirmation and should offer backup first.

## UI technology decision

Qt 6 / Qt Quick Controls is selected for the Linux desktop shell. It provides a native Qt application and a declarative QML presentation layer. Build and runtime compatibility must be verified on the supported distribution matrix before release.
