use clap::ValueEnum;
use clap::builder::PossibleValue;

#[derive(
    Debug,
    Clone,
    Copy,
    strum::EnumString,
    strum::IntoStaticStr,
    strum::AsRefStr,
    serde::Serialize,
    clap::ValueEnum,
)]
pub enum SidefxPlatform {
    #[strum(serialize = "win64")]
    #[serde(rename = "win64")]
    #[value(alias = "win64", alias = "win")]
    Win64,
    #[strum(serialize = "macos")]
    #[serde(rename = "macos")]
    #[value(alias = "macos")]
    MacOS,
    #[strum(serialize = "macosx_arm64")]
    #[serde(rename = "macosx_arm64")]
    #[value(alias = "macosx-arm64")]
    MacosArm64,
    #[strum(serialize = "linux_x86_64")]
    #[serde(rename = "linux_x86_64")]
    #[value(alias = "linux-64", alias = "linux")]
    Linux,
    #[strum(serialize = "linux_arm64")]
    #[serde(rename = "linux_arm64")]
    #[value(alias = "linux-arm64", alias = "linux-aarch64")]
    LinuxArm64,
}

#[derive(Debug, Clone, Copy, strum::EnumString, strum::IntoStaticStr, strum::AsRefStr)]
pub enum SidefxDownloadProduct {
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
impl ValueEnum for SidefxDownloadProduct {
    fn value_variants<'a>() -> &'a [Self] {
        static VARIANTS: [SidefxDownloadProduct; 13] = [
            SidefxDownloadProduct::Houdini,
            SidefxDownloadProduct::HoudiniPy3,
            SidefxDownloadProduct::HoudiniPy37,
            SidefxDownloadProduct::HoudiniPy2,
            SidefxDownloadProduct::Docker,
            SidefxDownloadProduct::Sidefxlabs,
            SidefxDownloadProduct::HoudiniLauncher,
            SidefxDownloadProduct::HoudiniLauncherPy3,
            SidefxDownloadProduct::HoudiniLauncherPy37,
            SidefxDownloadProduct::LauncherIso,
            SidefxDownloadProduct::LauncherIsoPy3,
            SidefxDownloadProduct::LauncherIsoPy37,
            SidefxDownloadProduct::LauncherIsoPy2,
        ];

        &VARIANTS
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(Into::<&str>::into(self)))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidefxDownloadBuildVersion {
    Number(u16),
    Production,
}
impl core::str::FromStr for SidefxDownloadBuildVersion {
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
pub enum SidefxLicenseProducts {
    #[strum(serialize = "HOUDINI-NC")]
    HoudiniNc,
    #[strum(serialize = "RENDER-NC")]
    RenderNc,
    #[strum(serialize = "HOUDINI-NC;RENDER-NC", serialize = "RENDER-NC;HOUDINI-NC")]
    All,
}
impl ValueEnum for SidefxLicenseProducts {
    fn value_variants<'a>() -> &'a [Self] {
        static VARIANTS: [SidefxLicenseProducts; 3] = [
            SidefxLicenseProducts::HoudiniNc,
            SidefxLicenseProducts::RenderNc,
            SidefxLicenseProducts::All,
        ];

        &VARIANTS
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::HoudiniNc | Self::RenderNc => Some(PossibleValue::new(Into::<&str>::into(self))),
            Self::All => {
                Some(PossibleValue::new("HOUDINI-NC;RENDER-NC").alias("RENDER-NC;HOUDINI-NC"))
            }
        }
    }
}
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("production", SidefxDownloadBuildVersion::Production)]
    #[case("123", SidefxDownloadBuildVersion::Number(123))]
    fn test_build_version(#[case] input: &str, #[case] expected: SidefxDownloadBuildVersion) {
        let version = SidefxDownloadBuildVersion::from_str(input).unwrap();
        assert_eq!(version, expected);
    }
    #[test]
    fn rejects_invalid_number() {
        let err = SidefxDownloadBuildVersion::from_str("abc");
        assert!(err.is_err())
    }
}
