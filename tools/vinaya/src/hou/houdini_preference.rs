use std::env;
use std::path::{Path, PathBuf};

use path_slash::PathExt;
use validator::Validate;

use crate::hou::{
    HOUDINI_VERSION_MAJOR_MAX, HOUDINI_VERSION_MAJOR_MIN, HOUDINI_VERSION_MINOR_MAX,
    HOUDINI_VERSION_MINOR_MIN,
};

#[derive(Debug, Clone, Validate)]
pub struct HoudiniPreference {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN,max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u16,
    #[validate(range(min=HOUDINI_VERSION_MINOR_MIN,max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u16,
    pub directory: PathBuf,
}
impl HoudiniPreference {
    fn default_preference_root() -> mischief::Result<PathBuf> {
        let default_perf_root: PathBuf = PathBuf::from(
            env::var("USERPROFILE")
                .map_err(|_| mischief::mischief!("windows userprofile dir is not found."))?,
        )
        .join("Documents");
        Ok(default_perf_root)
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
                    Self::default_preference_root()?.join(format!("houdini{major}.{minor}"));
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
