use std::path::{Path, PathBuf};

use glob::glob;
use hou_variable::HoudiniVersion;
use mischief::{IntoMischief, mischief};
use validator::Validate;

#[derive(Debug, Clone, Copy, Validate)]
pub struct HoudiniInstance {
    pub version: HoudiniVersion,
}

impl HoudiniInstance {
    pub const INSTALL_DIR: &str =
        cfg_select! {
            target_os = "windows" => "C:/Program Files/Side Effects Software",
            target_os = "macos" => "/Applications/Houdini",
            target_os = "linux" => "/opt",
        };

    fn dir_name(version: &HoudiniVersion) -> mischief::Result<String> {
        match *version {
            HoudiniVersion {
                major,
                minor,
                patch: Some(patch),
            } => {
                cfg_select! {
                    target_os = "windows" => Ok(format!("Houdini {major}.{minor}.{patch}")),
                    target_os = "macos" => Ok(format!("Houdini{major}.{minor}.{patch}")),
                    _ => Ok(format!("hfs{major}.{minor}.{patch}")),
                }
            }
            _ => {
                mischief::bail!("invalid version")
            }
        }
    }

    const DIR_GLOB_PATTERN: &'static str = cfg_select! {
        target_os = "windows" => "Houdini *.*.*",
        target_os = "macos" => "Houdini*.*.*",
        target_os = "linux" => "hfs*.*.*",
    };

    fn version_from_dir_name(name: &str) -> mischief::Result<HoudiniVersion> {
        let version_str =
            cfg_select! {
                target_os = "windows" => name
                    .split(' ')
                    .nth(1)
                    .ok_or_else(|| mischief!("Invalid Houdini directory name: {}", name))?,
                target_os = "macos" => name
                    .strip_prefix("Houdini")
                    .ok_or_else(|| mischief!("Invalid Houdini directory name: {}", name))?,
                _ => name
                    .strip_prefix("hfs")
                    .ok_or_else(|| mischief!("Invalid Houdini directory name: {}", name))?,
            };
        let version = HoudiniVersion::try_from(version_str)?;
        Ok(version)
    }

    pub fn from_version_string(version_string: &str) -> mischief::Result<Self> {
        let version = HoudiniVersion::try_from(version_string)?;
        if version.patch.is_none() {
            mischief::bail!("version string must include patch number")
        }
        Ok(Self { version })
    }

    pub fn list_installed() -> mischief::Result<Vec<Self>> {
        let glob_result = glob(
            &Path::new(Self::INSTALL_DIR)
                .join(Self::DIR_GLOB_PATTERN)
                .to_string_lossy(),
        )
        .into_mischief()?;

        let mut hinstances = glob_result
            .map(|f| {
                let path = f.into_mischief()?;
                let name = path
                    .file_name()
                    .ok_or_else(|| mischief::mischief!("fail to get file name"))?
                    .to_string_lossy();
                Ok(Self {
                    version: Self::version_from_dir_name(&name)?,
                })
            })
            .collect::<mischief::Result<Vec<Self>>>()?;

        hinstances.sort_by(|a, b| {
            b.version.major().cmp(&a.version.major()).then_with(|| {
                b.version
                    .minor()
                    .cmp(&a.version.minor())
                    .then_with(|| b.version.patch().cmp(&a.version.patch()))
            })
        });

        if hinstances.is_empty() {
            mischief::bail!(
                "No Houdini installed.",
                help = format!("Check your Houdini Install path: \"{}\"", Self::INSTALL_DIR)
            )
        }

        Ok(hinstances)
    }

    pub fn latest_installed_version() -> mischief::Result<Self> {
        Self::list_installed()?
            .first()
            .copied()
            .ok_or_else(|| mischief::mischief!("No Houdini installed."))
    }

    pub fn installed(&self) -> mischief::Result<bool> {
        let houdini_executable =
            cfg_select! {
                target_os = "windows" => self.hfs()?.join("bin").join("houdini.exe"),
                _ => self.hfs()?.join("bin").join("houdini.exe"),
            };

        Ok(houdini_executable.exists())
    }

    pub fn hfs(&self) -> mischief::Result<PathBuf> {
        Ok(Path::new(Self::INSTALL_DIR).join(Self::dir_name(&self.version)?))
    }

    pub fn cmake_prefix_path(&self) -> mischief::Result<PathBuf> {
        Ok(Path::new(Self::INSTALL_DIR)
            .join(Self::dir_name(&self.version)?)
            .join("toolkit")
            .join("cmake"))
    }
}
#[cfg(test)]
mod tests {
    use path_slash::PathBufExt;

    use super::*;

    fn instance() -> HoudiniInstance {
        HoudiniInstance {
            version: HoudiniVersion::new(20, 5, Some(123)),
        }
    }

    #[test]
    fn test_dir_name() -> mischief::Result<()> {
        let expected = cfg_select! {
            target_os = "windows" => "Houdini 20.5.123",
            target_os = "macos" => "Houdini20.5.123",
            _ => "hfs20.5.123",
        };
        assert_eq!(
            HoudiniInstance::dir_name(&HoudiniVersion::new(20, 5, Some(123)))?,
            PathBuf::from(expected)
        );
        Ok(())
    }

    #[test]
    fn test_version_from_dir_name_valid() -> mischief::Result<()> {
        let name = HoudiniInstance::dir_name(&HoudiniVersion::new(20, 5, Some(123)))?;
        let version = HoudiniInstance::version_from_dir_name(&name).unwrap();
        assert_eq!(version.major(), 20);
        assert_eq!(version.minor(), 5);
        assert_eq!(version.patch(), Some(123));
        Ok(())
    }

    #[test]
    fn test_version_from_dir_name_invalid() {
        assert!(HoudiniInstance::version_from_dir_name("garbage").is_err());
    }

    #[test]
    fn test_version_string_with_patch() {
        assert_eq!(instance().version.to_string(), "20.5.123");
    }

    #[test]
    fn test_hfs() -> mischief::Result<()> {
        let expected =
            cfg_select! {
                target_os = "windows" => "C:/Program Files/Side Effects Software/Houdini 20.5.123",
                target_os = "macos" => "/Applications/Houdini/Houdini20.5.123",
                _ => "/opt/hfs20.5.123",
            };
        assert_eq!(instance().hfs()?.to_slash_lossy(), expected);
        Ok(())
    }

    #[test]
    fn test_cmake_prefix_path() -> mischief::Result<()> {
        let expected =
            cfg_select! {
                target_os = "windows" => {
                    "C:/Program Files/Side Effects Software/Houdini 20.5.123/toolkit/cmake"
                }
                target_os = "macos" => "/Applications/Houdini/Houdini20.5.123/toolkit/cmake",
                _ => "/opt/hfs20.5.123/toolkit/cmake",
            };
        assert_eq!(instance().cmake_prefix_path()?.to_slash_lossy(), expected);
        Ok(())
    }

    #[test]
    fn test_from_version_string_valid() {
        let inst = HoudiniInstance::from_version_string("20.5.123").unwrap();
        assert_eq!(inst.version.major(), 20);
        assert_eq!(inst.version.minor(), 5);
        assert_eq!(inst.version.patch(), Some(123));
    }

    #[test]
    fn test_from_version_string_invalid() {
        assert!(HoudiniInstance::from_version_string("20.5").is_err());
        assert!(HoudiniInstance::from_version_string("abc").is_err());
    }
}
