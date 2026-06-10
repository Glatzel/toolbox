use std::env;
use std::path::{Path, PathBuf};

use path_slash::PathExt;
use validator::Validate;

use crate::{HOUDINI_VERSION_MAJOR_MAX, HOUDINI_VERSION_MAJOR_MIN, HOUDINI_VERSION_MINOR_MAX};

#[derive(Debug, Clone, Validate)]
pub struct HoudiniPreference {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN,max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u16,
    #[validate(range(max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u16,
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
            target_os = "macos" => {
               home.join("Library").join("Preferences").join("houdini")
            }
            _ => {
                Ok(home.clone())
            }
        }
    }
    pub fn from_version(major: u16, minor: u16) -> mischief::Result<Self> {
        match env::var("HOUDINI_USER_PREF_DIR") {
            Ok(val) => {
                let pref_dir: PathBuf = Path::new(&val)
                    .parent()
                    .unwrap()
                    .join(format!("houdini{major}.{minor}"));
                let perf = Self {
                    major,
                    minor,
                    directory: pref_dir,
                };
                Ok(perf)
            }
            Err(_) => {
                let pref_dir: PathBuf =
                    Self::preference_root()?.join(format!("houdini{major}.{minor}"));
                let perf: HoudiniPreference = Self {
                    major,
                    minor,
                    directory: pref_dir,
                };
                Ok(perf)
            }
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
        let pref = HoudiniPreference::from_version(20, 5).unwrap();
        let home = dirs::home_dir().unwrap();
        let expected = cfg_select! {
            target_os = "macos" => { home.join("Library").join("Preferences").join("houdini").join("houdini20.5") }
            _ =>                   { home.join("houdini20.5") }
        };
        assert_eq!(pref.directory.to_slash_lossy(), expected.to_slash_lossy());
    }
    #[test]
    fn test_from_version_env_override() {
        unsafe { env::set_var("HOUDINI_USER_PREF_DIR", "/some/custom/path/houdini__HVER__") };
        let pref = HoudiniPreference::from_version(20, 5).unwrap();
        assert_eq!(
            pref.directory.to_slash_lossy(),
            "/some/custom/path/houdini20.5"
        );
        unsafe { env::remove_var("HOUDINI_USER_PREF_DIR") };
    }
}
