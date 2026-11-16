//! Android Bedrock execution adapter.
//!
//! This crate intentionally contains no installer, authentication bypass,
//! proprietary game payload, or launch implementation yet. Integration work
//! is gated on compatibility and licensing review.

use tomos_bedrock_core::{BackendKind, Capability};

pub const BACKEND_KIND: BackendKind = BackendKind::Android;

/// Capabilities currently verified by this adapter.
pub fn supported_capabilities() -> &'static [Capability] {
    &[]
}
