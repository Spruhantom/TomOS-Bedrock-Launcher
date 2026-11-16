//! Backend contracts and capability-aware diagnostics.

use crate::host::HostInfo;
use crate::{BackendKind, Capability};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendProbe {
    pub kind: BackendKind,
    pub available: bool,
    pub capabilities: Vec<Capability>,
    pub diagnostics: Vec<Diagnostic>,
}

impl BackendProbe {
    pub fn unavailable(kind: BackendKind, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind,
            available: false,
            capabilities: Vec::new(),
            diagnostics: vec![Diagnostic {
                level: DiagnosticLevel::Warning,
                code,
                message: message.into(),
            }],
        }
    }
}

/// Implemented by each execution adapter. Probing must be read-only:
/// it must not install packages, authenticate, or modify user data.
pub trait Backend: Send + Sync {
    fn kind(&self) -> BackendKind;
    fn display_name(&self) -> &'static str;
    fn probe(&self, host: &HostInfo) -> BackendProbe;
}
