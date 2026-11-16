//! Dashboard-ready application state assembled from host and backend probes.

use crate::{BackendKind, backend::BackendProbe, host::HostInfo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendCard {
    pub kind: BackendKind,
    pub name: &'static str,
    pub available: bool,
    pub status: &'static str,
    pub diagnostic_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DashboardSnapshot {
    pub distribution: String,
    pub architecture: String,
    pub session_type: String,
    pub backends: Vec<BackendCard>,
}

impl DashboardSnapshot {
    pub fn build(host: &HostInfo, probes: &[BackendProbe]) -> Self {
        let backends = probes
            .iter()
            .map(|probe| {
                let (name, status) = match probe.kind {
                    BackendKind::Windows => ("Windows Bedrock", "Detected"),
                    BackendKind::Android => ("Android Bedrock", "Detected"),
                };
                BackendCard {
                    kind: probe.kind,
                    name,
                    available: probe.available,
                    status: if probe.available {
                        status
                    } else {
                        "Not detected"
                    },
                    diagnostic_count: probe.diagnostics.len(),
                }
            })
            .collect();

        Self {
            distribution: host
                .pretty_name
                .clone()
                .or_else(|| host.os_id.clone())
                .unwrap_or_else(|| "Unknown Linux distribution".to_owned()),
            architecture: host.architecture.clone(),
            session_type: host
                .session_type
                .clone()
                .unwrap_or_else(|| "Unknown".to_owned()),
            backends,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DashboardSnapshot;
    use crate::{BackendKind, backend::BackendProbe, host::HostInfo};

    #[test]
    fn builds_a_dashboard_without_overstating_backend_support() {
        let host = HostInfo {
            os_id: Some("ubuntu".into()),
            os_like: vec!["debian".into()],
            pretty_name: Some("Ubuntu 26.10".into()),
            architecture: "x86_64".into(),
            kernel_release: None,
            session_type: Some("wayland".into()),
            available_tools: Vec::new(),
        };
        let probes = vec![BackendProbe::unavailable(
            BackendKind::Windows,
            "missing",
            "Not installed",
        )];

        let dashboard = DashboardSnapshot::build(&host, &probes);
        assert_eq!(dashboard.distribution, "Ubuntu 26.10");
        assert_eq!(dashboard.backends.len(), 1);
        assert!(!dashboard.backends[0].available);
        assert_eq!(dashboard.backends[0].status, "Not detected");
    }
}
