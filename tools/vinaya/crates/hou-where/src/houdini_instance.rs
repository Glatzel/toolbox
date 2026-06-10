use std::path::{Path, PathBuf};

use glob::glob;
use mischief::{IntoMischief, mischief};
use regex::Regex;
use validator::Validate;

use crate::{
    HOUDINI_VERSION_MAJOR_MAX, HOUDINI_VERSION_MAJOR_MIN, HOUDINI_VERSION_MINOR_MAX,
    HOUDINI_VERSION_PATCH_MAX,
};

#[derive(Debug, Clone, Copy, Validate)]
pub struct HoudiniInstance {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN,max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u16,
    #[validate(range(max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u16,
    #[validate(range(max=HOUDINI_VERSION_PATCH_MAX))]
    pub patch: u16,
}

impl HoudiniInstance {
    pub const INSTALL_DIR: &str = cfg_select! {
        target_os = "windows" => {
            "C:/Program Files/Side Effects Software"
        }
        target_os = "macos" => {
            "/Applications/Houdini"
        }
        target_os = "linux" => {
            "/opt"
        }
    };

    fn dir_name(major: u16, minor: u16, patch: u16) -> String {
        cfg_select! {
            target_os = "windows" => { format!("Houdini {major}.{minor}.{patch}") }
            target_os = "macos"   => { format!("Houdini{major}.{minor}.{patch}") }
            _ =>                     { format!("hfs{major}.{minor}.{patch}") }
        }
    }

    const DIR_GLOB_PATTERN: &'static str = cfg_select! {
        target_os = "windows" => { "Houdini *.*.*" }
        target_os = "macos"   => { "Houdini*.*.*" }
        target_os = "linux" =>                     { "hfs*.*.*" }
    };

    fn version_from_dir_name(name: &str) -> mischief::Result<(u16, u16, u16)> {
        let version_str = cfg_select! {
            target_os = "windows" => { name.split(' ').nth(1).ok_or_else(|| mischief!("Invalid Houdini directory name: {}", name))? }
            target_os = "macos"   => { name.strip_prefix("Houdini").ok_or_else(|| mischief!("Invalid Houdini directory name: {}", name))? }
            _ =>                     { name.strip_prefix("hfs").ok_or_else(|| mischief!("Invalid Houdini directory name: {}", name))? }
        };
        let parts: Vec<u16> = version_str
            .split('.')
            .map(|s| s.parse().into_mischief())
            .collect::<mischief::Result<Vec<_>>>()?;
        match parts.as_slice() {
            &[major, minor, patch] => Ok((major, minor, patch)),
            _ => mischief::bail!("Invalid Houdini directory name: {}", name),
        }
    }

    pub fn from_version_string(version_string: &str) -> mischief::Result<Self> {
        let pattern = r"^(\d+)\.(\d+)\.(\d+)$";
        clerk::debug!("Houdini version string regex pattern: {}", pattern);
        let re: Regex = Regex::new(pattern).into_mischief()?;
        let caps = re.captures(version_string).ok_or_else(|| {
            mischief!(
                "Invalid version string: {}",
                version_string,
                help = "Try a string like: 20.5.123",
            )
        })?;
        let instance: HoudiniInstance = Self {
            major: caps.get(1).unwrap().as_str().parse::<u16>().unwrap(),
            minor: caps.get(2).unwrap().as_str().parse::<u16>().unwrap(),
            patch: caps.get(3).unwrap().as_str().parse::<u16>().unwrap(),
        };
        Ok(instance)
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
                let path = f.unwrap();
                let name = path.file_name().unwrap().to_string_lossy();
                let (major, minor, patch) = Self::version_from_dir_name(&name)?;
                Ok(Self {
                    major,
                    minor,
                    patch,
                })
            })
            .collect::<mischief::Result<Vec<Self>>>()?;

        hinstances.sort_by(|a, b| {
            b.major
                .cmp(&a.major)
                .then_with(|| b.minor.cmp(&a.minor).then_with(|| b.patch.cmp(&a.patch)))
        });

        if hinstances.is_empty() {
            mischief::bail!(
                "No Houdini installed.",
                help = format!("Check your Houdini Install path: \"{}\"", Self::INSTALL_DIR)
            )
        }

        Ok(hinstances)
    }

    pub fn latest_installed_version() -> mischief::Result<HoudiniInstance> {
        Ok(Self::list_installed()?[0])
    }

    pub fn check_is_installed(&self) -> mischief::Result<()> {
        let p =
            Path::new(Self::INSTALL_DIR).join(Self::dir_name(self.major, self.minor, self.patch));
        if !p.exists() {
            mischief::bail!(
                "Houdini {}.{}.{} is not installed.",
                self.major,
                self.minor,
                self.patch
            )
        }
        Ok(())
    }

    pub fn version_string(&self, patch: bool) -> String {
        if patch {
            format!("{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            format!("{}.{}", self.major, self.minor)
        }
    }

    pub fn hfs(&self) -> PathBuf {
        Path::new(Self::INSTALL_DIR).join(Self::dir_name(self.major, self.minor, self.patch))
    }

    pub fn cmake_prefix_path(&self) -> PathBuf {
        Path::new(Self::INSTALL_DIR)
            .join(Self::dir_name(self.major, self.minor, self.patch))
            .join("toolkit")
            .join("cmake")
    }
}
