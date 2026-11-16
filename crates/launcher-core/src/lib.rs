//! Shared, backend-neutral types for TomOS Bedrock Launcher.

/// A supported game execution family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendKind {
    Windows,
    Android,
}

/// Capabilities a backend may expose. UI features must check these at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    VersionManagement,
    AddonManagement,
    WorldManagement,
    ControllerInput,
    Multiplayer,
    Realms,
}

/// Stable identifier for a user-managed game installation/profile.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfileId(String);

impl ProfileId {
    /// Creates an identifier after validating its basic shape.
    pub fn new(value: impl Into<String>) -> Result<Self, &'static str> {
        let value = value.into();
        if value.is_empty() || value.len() > 64 {
            return Err("profile identifier must contain 1 to 64 characters");
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_".contains(&byte))
        {
            return Err("profile identifier may only contain ASCII letters, digits, '-' and '_'");
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::ProfileId;

    #[test]
    fn accepts_safe_profile_id() {
        assert_eq!(ProfileId::new("survival_1").unwrap().as_str(), "survival_1");
    }

    #[test]
    fn rejects_path_like_profile_id() {
        assert!(ProfileId::new("../world").is_err());
    }

    #[test]
    fn rejects_empty_profile_id() {
        assert!(ProfileId::new("").is_err());
    }
}
