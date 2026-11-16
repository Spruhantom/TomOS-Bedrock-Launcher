//! Read-only Linux host information used for diagnostics and preflight checks.

use std::collections::BTreeMap;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostInfo {
    pub os_id: Option<String>,
    pub os_like: Vec<String>,
    pub pretty_name: Option<String>,
    pub architecture: String,
    pub kernel_release: Option<String>,
    pub session_type: Option<String>,
    pub available_tools: Vec<String>,
}

impl HostInfo {
    pub fn has_command(&self, command: &str) -> bool {
        self.available_tools.iter().any(|item| item == command)
    }
}

/// Collects basic information without invoking external commands or changing the host.
pub fn detect_host() -> HostInfo {
    let release = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let values = parse_os_release(&release);
    let path = env::var_os("PATH").unwrap_or_default();
    let available_tools = [
        "bedrock-on-linux",
        "mcpelauncher-client",
        "mcpelauncher-ui-qt",
        "wine",
        "proton",
        "vulkaninfo",
        "glxinfo",
        "eglinfo",
        "flatpak",
        "steam",
    ]
    .into_iter()
    .filter(|name| command_exists(name, &path))
    .map(str::to_owned)
    .collect();

    let os_like = values
        .get("ID_LIKE")
        .map(|value| value.split_whitespace().map(str::to_owned).collect())
        .unwrap_or_default();

    HostInfo {
        os_id: values.get("ID").cloned(),
        os_like,
        pretty_name: values.get("PRETTY_NAME").cloned(),
        architecture: env::consts::ARCH.to_owned(),
        kernel_release: fs::read_to_string("/proc/sys/kernel/osrelease")
            .ok()
            .map(|value| value.trim().to_owned()),
        session_type: env::var("XDG_SESSION_TYPE").ok(),
        available_tools,
    }
}

fn command_exists(command: &str, path: &std::ffi::OsStr) -> bool {
    env::split_paths(path).any(|directory| directory.join(command).is_file())
}

fn parse_os_release(contents: &str) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    for line in contents.lines() {
        let Some((key, raw_value)) = line.split_once('=') else {
            continue;
        };
        if key.is_empty() || !key.bytes().all(|b| b.is_ascii_uppercase() || b == b'_') {
            continue;
        }
        let value = raw_value.trim();
        let unquoted = if value.len() >= 2
            && ((value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\'')))
        {
            &value[1..value.len() - 1]
        } else {
            value
        };
        values.insert(key.to_owned(), unquoted.to_owned());
    }
    values
}

#[cfg(test)]
mod tests {
    use super::parse_os_release;

    #[test]
    fn parses_common_os_release_fields() {
        let parsed = parse_os_release(
            "NAME=Ubuntu\nID=ubuntu\nID_LIKE=\"debian\"\nPRETTY_NAME=\"Ubuntu 26.10\"\n",
        );
        assert_eq!(parsed.get("ID").map(String::as_str), Some("ubuntu"));
        assert_eq!(parsed.get("ID_LIKE").map(String::as_str), Some("debian"));
        assert_eq!(
            parsed.get("PRETTY_NAME").map(String::as_str),
            Some("Ubuntu 26.10")
        );
    }

    #[test]
    fn ignores_malformed_and_non_uppercase_keys() {
        let parsed = parse_os_release("not-a-field\nname=bad\nID=debian\n");
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed.get("ID").map(String::as_str), Some("debian"));
    }
}
