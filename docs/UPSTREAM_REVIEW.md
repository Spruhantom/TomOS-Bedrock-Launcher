# Upstream integration review

Review date: 2026-11-16.

## Windows: Wyze3306/BedrockOnLinux

Repository: https://github.com/Wyze3306/BedrockOnLinux

The upstream README describes a Windows Bedrock launcher for Linux with Microsoft Store download using the user's own account, Xbox sign-in, Friends, public servers, Realms, Marketplace, Stable/Preview and older version selection. It also documents controller support, world backup before opening worlds with a different version, and an internet requirement at each start.

The repository declares MIT for its launcher, while bundled components retain their own licences. It documents x86-64 Linux and Vulkan as its standard requirements; ARM requires emulation. This is a strong Windows-backend candidate.

Integration decision:
- First integration should be an optional external executable adapter, not a source-code fork.
- Do not infer multiplayer, Realms, or controller support from executable presence; capability reporting remains empty until verified against a supported upstream version.
- Do not automate Microsoft Store sign-in or credential capture.
- Do not bundle the game or upstream binaries without a separate distribution and licence review.

## Android: minecraft-linux/mcpelauncher-manifest

Repository: https://github.com/minecraft-linux/mcpelauncher-manifest

The upstream project identifies itself as an unstable unofficial launcher for Android Bedrock on Linux/macOS and declares GPL-3.0. Its README documents architecture- and graphics-dependent version support, including OpenGL ES requirements and known Xbox invite / friend-join limitations. The project publishes packages for Debian/Ubuntu and Fedora and documents AUR packages.

Integration decision:
- Treat as an optional external runtime dependency for the Android backend.
- Do not copy GPL code into the launcher workspace unless the project adopts a compatible distribution licence and fulfils corresponding source obligations.
- Do not distribute APKs or bypass Google Play ownership checks.
- Keep version compatibility and graphics requirements visible to users; a detected binary is not proof that a current Minecraft release will run.

## Conclusion

The two upstreams are materially different products and runtime models. Maintain independent adapters and present backend-specific feature availability. Continue with external-process integration first; revisit deeper library integration only after technical, security, and licence review.
