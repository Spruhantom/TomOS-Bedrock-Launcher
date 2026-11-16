# Security model

- Store secrets only through the desktop Secret Service (or an explicitly supported secure credential provider); never store refresh tokens in plaintext configuration, logs, crash reports, or command-line arguments.
- Prefer official OAuth/device authorization flows. Never collect Microsoft or Google passwords directly.
- Do not bypass account ownership, DRM, platform entitlements, or Marketplace access controls.
- Treat downloaded archives, add-ons, manifests, world imports, and metadata as untrusted input.
- Validate archive paths against traversal, symlinks, absolute paths, oversized files, decompression bombs, and unexpected file types before extraction.
- Verify TLS for all network operations. Do not disable certificate validation.
- Use checksums/signatures when published by a trusted source; clearly distinguish integrity checks from publisher authenticity.
- Run game processes as the logged-in user, with no elevated privileges by default.
- Make backup/restore operations atomic where possible and verify restored data.
- Redact tokens, account identifiers, and private filesystem paths from diagnostics.
- Provide an uninstall path that removes launcher-managed components without deleting user worlds unless the user explicitly requests it.
