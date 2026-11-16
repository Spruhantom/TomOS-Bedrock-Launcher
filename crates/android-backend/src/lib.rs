//! Android Bedrock adapter for a separately installed MCPE Launcher runtime.
//! Presence detection is not a guarantee that a given game version is playable.

use tomos_bedrock_core::{
    BackendKind,
    backend::{Backend, BackendProbe},
    host::HostInfo,
};

pub struct AndroidBackend;

impl Backend for AndroidBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Android
    }

    fn display_name(&self) -> &'static str {
        "Android Bedrock"
    }

    fn probe(&self, host: &HostInfo) -> BackendProbe {
        if host.has_command("mcpelauncher-client") {
            BackendProbe {
                kind: BackendKind::Android,
                available: true,
                capabilities: Vec::new(),
                diagnostics: Vec::new(),
            }
        } else {
            BackendProbe::unavailable(
                BackendKind::Android,
                "android_backend_not_found",
                "The MCPE Launcher runtime was not found on PATH. Install it through its documented distribution channel and confirm Google Play ownership; this launcher will not fetch APKs from unofficial mirrors.",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AndroidBackend;
    use tomos_bedrock_core::{
        BackendKind,
        backend::Backend,
        host::HostInfo,
    };

    fn host(tools: &[&str]) -> HostInfo {
        HostInfo {
            os_id: Some("fedora".into()),
            os_like: Vec::new(),
            pretty_name: Some("Fedora".into()),
            architecture: "x86_64".into(),
            kernel_release: None,
            session_type: None,
            available_tools: tools.iter().map(|tool| (*tool).into()).collect(),
        }
    }

    #[test]
    fn reports_missing_runtime() {
        let result = AndroidBackend.probe(&host(&[]));
        assert_eq!(result.kind, BackendKind::Android);
        assert!(!result.available);
    }

    #[test]
    fn detects_runtime_without_claiming_capabilities() {
        let result = AndroidBackend.probe(&host(&["mcpelauncher-client"]));
        assert!(result.available);
        assert!(result.capabilities.is_empty());
    }
}
