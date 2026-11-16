//! XDG-compliant locations for launcher-owned configuration and data.

use std::env;
use std::path::{Path, PathBuf};

const APP_DIRECTORY: &str = "tomos-bedrock-launcher";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
}

/// Resolves paths without creating directories or changing the user's filesystem.
impl AppPaths {
    pub fn discover() -> Result<Self, &'static str> {
        let home = env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or("HOME is not set")?;
        Self::from_values(
            &home,
            env::var_os("XDG_CONFIG_HOME").as_deref(),
            env::var_os("XDG_DATA_HOME").as_deref(),
            env::var_os("XDG_CACHE_HOME").as_deref(),
        )
    }

    pub fn from_values(
        home: &Path,
        config_home: Option<&std::ffi::OsStr>,
        data_home: Option<&std::ffi::OsStr>,
        cache_home: Option<&std::ffi::OsStr>,
    ) -> Result<Self, &'static str> {
        if !home.is_absolute() {
            return Err("home directory must be an absolute path");
        }

        let config_base = xdg_base(config_home, home.join(".config"));
        let data_base = xdg_base(data_home, home.join(".local/share"));
        let cache_base = xdg_base(cache_home, home.join(".cache"));

        Ok(Self {
            config_dir: config_base.join(APP_DIRECTORY),
            data_dir: data_base.join(APP_DIRECTORY),
            cache_dir: cache_base.join(APP_DIRECTORY),
        })
    }

    /// Creates only the launcher-owned directories after an explicit caller request.
    pub fn create_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config_dir)?;
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }
}

fn xdg_base(value: Option<&std::ffi::OsStr>, fallback: PathBuf) -> PathBuf {
    value
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .unwrap_or(fallback)
}

#[cfg(test)]
mod tests {
    use super::AppPaths;
    use std::ffi::OsStr;
    use std::path::Path;

    #[test]
    fn uses_xdg_directories_when_absolute() {
        let paths = AppPaths::from_values(
            Path::new("/home/tester"),
            Some(OsStr::new("/config")),
            Some(OsStr::new("/data")),
            Some(OsStr::new("/cache")),
        )
        .unwrap();

        assert_eq!(
            paths.config_dir,
            Path::new("/config/tomos-bedrock-launcher")
        );
        assert_eq!(paths.data_dir, Path::new("/data/tomos-bedrock-launcher"));
        assert_eq!(paths.cache_dir, Path::new("/cache/tomos-bedrock-launcher"));
    }

    #[test]
    fn ignores_relative_xdg_values_and_uses_home_fallbacks() {
        let paths = AppPaths::from_values(
            Path::new("/home/tester"),
            Some(OsStr::new("relative-config")),
            None,
            None,
        )
        .unwrap();

        assert_eq!(
            paths.config_dir,
            Path::new("/home/tester/.config/tomos-bedrock-launcher")
        );
        assert_eq!(
            paths.data_dir,
            Path::new("/home/tester/.local/share/tomos-bedrock-launcher")
        );
        assert_eq!(
            paths.cache_dir,
            Path::new("/home/tester/.cache/tomos-bedrock-launcher")
        );
    }

    #[test]
    fn rejects_relative_home_directory() {
        assert!(AppPaths::from_values(Path::new("home/tester"), None, None, None).is_err());
    }
}
