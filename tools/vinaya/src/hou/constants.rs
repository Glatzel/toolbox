pub const HOUDINI_VERSION_MAJOR_MIN: u16 = 15;
pub const HOUDINI_VERSION_MAJOR_MAX: u16 = 99;
pub const HOUDINI_VERSION_MINOR_MIN: u16 = 0;
pub const HOUDINI_VERSION_MINOR_MAX: u16 = 99;
pub const HOUDINI_VERSION_PATCH_MIN: u16 = 0;
pub const HOUDINI_VERSION_PATCH_MAX: u16 = 9999;
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
}

#[derive(Debug, Clone, Copy, strum::EnumString, strum::IntoStaticStr, strum::AsRefStr)]
pub enum HoudiniProduct {
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
pub enum HoudiniBuildVersion {
    Number(u16),
    Production,
}
impl std::str::FromStr for HoudiniBuildVersion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "production" {
            return Ok(Self::Production);
        }

        let n = s.parse::<u16>().map_err(|_| "invalid build number")?;
        Ok(Self::Number(n))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("production", HoudiniBuildVersion::Production)]
    #[case("123", HoudiniBuildVersion::Number(123))]
    fn test_build_version(#[case] input: &str, #[case] expected: HoudiniBuildVersion) {
        let version = HoudiniBuildVersion::from_str(input).unwrap();
        assert_eq!(version, expected);
    }
    #[test]
    fn rejects_invalid_number() {
        let err = HoudiniBuildVersion::from_str("abc");
        assert!(err.is_err())
    }
}
