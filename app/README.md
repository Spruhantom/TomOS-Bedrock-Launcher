# Desktop application

The desktop shell is a Qt 6 / Qt Quick Controls application backed by a small Rust dashboard service. On startup, the Qt application runs the service, parses its JSON response, and supplies host and backend runtime-presence information to QML.

## Build

Dependencies:

- CMake 3.21 or newer
- C++17 compiler
- Qt 6.4 or newer with Quick and Quick Controls 2 development components
- Current stable Rust toolchain (Cargo and rustc)

From the repository root:

```sh
cmake -S app -B build/ui -DCMAKE_BUILD_TYPE=Release
cmake --build build/ui --parallel
./build/ui/tomos-bedrock-ui
```

The CMake build compiles the Rust service into the build directory and links its executable path into the Qt application. The service performs read-only checks only; it does not install packages, authenticate, download game files, or launch Minecraft.

## Current limitations

- Runtime executable presence does not prove that the installed game is compatible or launchable.
- Backend selection is local UI state only; it does not launch or install anything.
- Game versions, add-ons, worlds, controllers, multiplayer and Realms are navigation placeholders.
- No credentials or game content are read or written.
- The Play button remains disabled until profile discovery, runtime preflight, and safe launch handling are implemented.
