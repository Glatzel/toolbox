use validator::Validate;
pub const HOUDINI_VERSION_MAJOR_MIN: u8 = 15;
pub const HOUDINI_VERSION_MAJOR_MAX: u8 = 99;
pub const HOUDINI_VERSION_MINOR_MAX: u8 = 99;
pub const HOUDINI_VERSION_PATCH_MAX: u16 = 9999;
#[derive(Debug, Clone, Copy, Validate)]
pub struct HoudiniVersion {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN, max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u8,
    #[validate(range(min=0, max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u8,
    #[validate(range(min=0, max=HOUDINI_VERSION_PATCH_MAX))]
    pub patch: Option<u16>,
}
impl HoudiniVersion {
    pub fn new(major: u8, minor: u8, patch: Option<u16>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn major(&self) -> u8 { self.major }

    pub fn minor(&self) -> u8 { self.minor }

    pub fn patch(&self) -> Option<u16> { self.patch }

    pub fn to_string(&self) -> String {
        match self.patch {
            Some(patch) => format!("{}.{}.{}", self.major, self.minor, patch),
            None => format!("{}.{}", self.major, self.minor),
        }
    }
    pub fn from_str(s: &str) -> mischief::Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        match parts.len() {
            2 => Ok(Self::new(parts[0].parse()?, parts[1].parse()?, None)),
            3 => Ok(Self::new(
                parts[0].parse()?,
                parts[1].parse()?,
                Some(parts[2].parse()?),
            )),
            _ => mischief::bail!("invalid version string"),
        }
    }
}
#[derive(Debug, Clone, Copy, strum::EnumString, strum::IntoStaticStr, strum::AsRefStr)]
pub enum HoudiniPlatform {
    #[strum(serialize = "win64")]
    Win64,
    #[strum(serialize = "macos")]
    MacOS,
    #[strum(serialize = "macosx_arm64")]
    MacOsArm64,
    #[strum(serialize = "linux")]
    Linux,
    #[strum(serialize = "linux_arm64")]
    LinuxArm64,
}
#[derive(Debug, Clone, Copy, strum::EnumString, strum::IntoStaticStr, strum::AsRefStr)]
pub enum HoudiniDownloadProduct {
    #[strum(serialize = "houdini")]
    Houdini,
    #[strum(serialize = "houdini-py3")]
    HoudiniPy3,
    #[strum(serialize = "houdini-py37")]
    HoudiniPy37,
    #[strum(serialize = "houdini-py2")]
    HoudiniPy2,
    #[strum(serialize = "docker")]
    Docker,
    #[strum(serialize = "sidefxlabs")]
    Sidefxlabs,
    #[strum(serialize = "houdini-launcher")]
    HoudiniLauncher,
    #[strum(serialize = "houdini-launcher-py3")]
    HoudiniLauncherPy3,
    #[strum(serialize = "houdini-launcher-py37")]
    HoudiniLauncherPy37,
    #[strum(serialize = "launcher-iso")]
    LauncherIso,
    #[strum(serialize = "launcher-iso-py3")]
    LauncherIsoPy3,
    #[strum(serialize = "launcher-iso-py37")]
    LauncherIsoPy37,
    #[strum(serialize = "launcher-iso-py2")]
    LauncherIsoPy2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoudiniDownloadBuildVersion {
    Number(u16),
    Production,
}
impl core::str::FromStr for HoudiniDownloadBuildVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "production" {
            return Ok(Self::Production);
        }

        let n = s.parse::<u16>().map_err(|_| "invalid build number")?;
        Ok(Self::Number(n))
    }
}
#[derive(Debug, Clone, Copy, strum::EnumString, strum::IntoStaticStr, strum::AsRefStr)]
pub enum HoudiniLicenseProducts {
    #[strum(serialize = "HOUDINI-NC")]
    HoudiniNc,
    #[strum(serialize = "RENDER-NC")]
    RenderNc,
    #[strum(serialize = "HOUDINI-NC;RENDER-NC", serialize = "RENDER-NC;HOUDINI-NC")]
    All,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("production", HoudiniDownloadBuildVersion::Production)]
    #[case("123", HoudiniDownloadBuildVersion::Number(123))]
    fn test_build_version(#[case] input: &str, #[case] expected: HoudiniDownloadBuildVersion) {
        let version = HoudiniDownloadBuildVersion::from_str(input).unwrap();
        assert_eq!(version, expected);
    }
    #[test]
    fn rejects_invalid_number() {
        let err = HoudiniDownloadBuildVersion::from_str("abc");
        assert!(err.is_err())
    }
}
