# Backend strategy

## Windows Bedrock

Target: the Windows Bedrock distribution running through a supported Wine/Proton-based compatibility environment.

Before implementation:
- inspect candidate projects and their current Linux support;
- verify each component's licence and transitive dependencies;
- confirm whether the user's legitimate installation can be used without redistributing game files;
- test Microsoft account sign-in and token storage using documented flows;
- test launch, updates, multiplayer, controller input, and Realms independently.

Do not assume Windows Bedrock features are available merely because the game launches under Wine. GDK, Xbox services, anti-cheat/service dependencies, and Microsoft authentication may impose blockers.

## Android Bedrock

Target: the Android Bedrock application running through a compatible Android-on-Linux runtime.

Before implementation:
- verify runtime support for the host kernel, graphics stack, audio, input, and architecture;
- document Google Play / Android licensing and authentication requirements;
- ensure the launcher does not download APKs from unofficial mirrors or bypass licensing;
- test application data migration and backups carefully.

## Shared capabilities

Version management, add-ons, worlds, controllers, multiplayer, and Realms are capability-gated. A capability may be available on one backend, partially supported on another, or unavailable. The UI must display that status accurately.

## Candidate upstream projects

Candidates are research inputs, not approved dependencies:
- BedrockLauncher / BL.Linux: https://github.com/BedrockLauncher
- Minecraft Linux Launcher: https://github.com/minecraft-linux/mcpelauncher-manifest

No source code is copied or linked into this workspace until licence compatibility, maintenance status, security posture, and technical fit are reviewed.
