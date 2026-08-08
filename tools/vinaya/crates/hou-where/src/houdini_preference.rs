use std::env;
use std::path::{Path, PathBuf};

use hou_variable::HoudiniVersionShort;
use path_slash::PathExt;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct HoudiniPreference {
    pub version: HoudiniVersionShort,
    pub directory: PathBuf,
}
impl HoudiniPreference {
    pub fn preference_root() -> mischief::Result<PathBuf> {
        if let Ok(pref_dir) = env::var("HOUDINI_USER_PREF_DIR") {
            let path = PathBuf::from(pref_dir);
            return Ok(path
                .parent()
                .ok_or_else(|| mischief::mischief!("HOUDINI_USER_PREF_DIR has no parent."))?
                .to_path_buf());
        }

        let home = dirs::home_dir()
            .ok_or_else(|| mischief::mischief!("Could not determine home directory."))?;

        cfg_select! {
            target_os = "macos" => Ok(home.join("Library").join("Preferences").join("houdini")),
            _ => Ok(home),
        }
    }
    pub fn from_version(version: &HoudiniVersionShort) -> mischief::Result<Self> {
        if let Ok(val) = env::var("HOUDINI_USER_PREF_DIR") {
            let pref_dir: PathBuf = Path::new(&val)
                .parent()
                .ok_or_else(|| mischief::mischief!("HOUDINI_USER_PREF_DIR has no parent."))?
                .join( cfg_select! {
                    target_os = "macos" => format!("{version}"),
                    _ =>format!("houdini{version}"),
                });
            let perf = Self {
                version: version.clone(),
                directory: pref_dir,
            };
            Ok(perf)
        } else {
            let pref_dir: PathBuf = Self::preference_root()?.join(format!("houdini{version}"));
            let perf: Self = Self {
                version: version.clone(),
                directory: pref_dir,
            };
            Ok(perf)
        }
    }
    pub fn check_is_existed(&self) -> mischief::Result<&Self> {
        if !&self.directory.exists() {
            mischief::bail!(
                "Houdini preference directory is not existed: {}",
                self.directory.to_slash_lossy(),
            )
        }
        Ok(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from_version_env_default() {
        unsafe { env::remove_var("HOUDINI_USER_PREF_DIR") };
        let pref = HoudiniPreference::from_version(&HoudiniVersionShort {
            major: 20,
            minor: 5,
        })
        .unwrap();
        let home = dirs::home_dir().unwrap();
        let expected = cfg_select! {
            target_os = "macos" => home
                .join("Library")
                .join("Preferences")
                .join("houdini")
                .join("houdini20.5"),
            _ => home.join("houdini20.5"),
        };
        assert_eq!(pref.directory.to_slash_lossy(), expected.to_slash_lossy());
    }
    #[test]
    fn test_from_version_env_override() {
        unsafe { env::set_var("HOUDINI_USER_PREF_DIR", "/some/custom/path/houdini__HVER__") };
        let pref = HoudiniPreference::from_version(&HoudiniVersionShort {
            major: 20,
            minor: 5,
        })
        .unwrap();
        assert_eq!(
            pref.directory.to_slash_lossy(),
            "/some/custom/path/houdini20.5"
        );
        unsafe { env::remove_var("HOUDINI_USER_PREF_DIR") };
    }
}
