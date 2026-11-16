# Desktop UI prototype

The first desktop shell is a Qt 6 / Qt Quick Controls application. It is deliberately a UI-only prototype at this stage: backend detection is not yet wired into QML, and Play remains disabled until a real profile, runtime preflight, and launch service are implemented.

## Build

Dependencies:

- CMake 3.21 or newer
- C++17 compiler
- Qt 6.4 or newer with Quick and Quick Controls 2 development components

From the repository root:

```sh
cmake -S app -B build/ui -DCMAKE_BUILD_TYPE=Release
cmake --build build/ui --parallel
./build/ui/tomos-bedrock-ui
```

On Debian/Ubuntu, install the distribution's Qt 6 Quick development packages and CMake/compiler toolchain. Fedora, Arch/Omarchy, and TomOS should use their native Qt 6 development packages.

## Current limitations

- Runtime status is intentionally shown as pending.
- Backend selection is local UI state only; it does not launch or install anything.
- Game versions, add-ons, worlds, controllers, multiplayer and Realms are navigation placeholders.
- No credentials or game content are read or written.
