//! Windows Bedrock adapter using an external, user-installed compatibility
//! project. The adapter does not install or modify that project automatically.

use tomos_bedrock_core::{
    BackendKind,
    backend::{Backend, BackendProbe},
    host::HostInfo,
};

pub struct WindowsBackend;

impl Backend for WindowsBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Windows
    }

    fn display_name(&self) -> &'static str {
        "Windows Bedrock"
    }

    fn probe(&self, host: &HostInfo) -> BackendProbe {
        if host.has_command("bedrock-on-linux") {
            BackendProbe {
                kind: BackendKind::Windows,
                available: true,
                // Feature support is not inferred from binary presence.
                capabilities: Vec::new(),
                diagnostics: Vec::new(),
            }
        } else {
            BackendProbe::unavailable(
                BackendKind::Windows,
                "windows_backend_not_found",
                "BedrockOnLinux was not found on PATH. Install and configure the upstream project separately; this launcher will not download game files or install it silently.",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsBackend;
    use tomos_bedrock_core::{
        BackendKind,
        backend::Backend,
        host::HostInfo,
    };

    fn host(tools: &[&str]) -> HostInfo {
        HostInfo {
            os_id: Some("ubuntu".into()),
            os_like: vec!["debian".into()],
            pretty_name: Some("Ubuntu".into()),
            architecture: "x86_64".into(),
            kernel_release: Some("test-kernel".into()),
            session_type: Some("wayland".into()),
            available_tools: tools.iter().map(|tool| (*tool).into()).collect(),
        }
    }

    #[test]
    fn reports_missing_upstream_binary() {
        let result = WindowsBackend.probe(&host(&[]));
        assert_eq!(result.kind, BackendKind::Windows);
        assert!(!result.available);
        assert_eq!(result.diagnostics[0].code, "windows_backend_not_found");
    }

    #[test]
    fn detects_upstream_binary_without_claiming_feature_support() {
        let result = WindowsBackend.probe(&host(&["bedrock-on-linux"]));
        assert!(result.available);
        assert!(result.capabilities.is_empty());
    }
}
