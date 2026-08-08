use core::fmt::Display;
use std::str::FromStr;

use validator::Validate;
pub const HOUDINI_VERSION_MAJOR_MIN: u8 = 15;
pub const HOUDINI_VERSION_MAJOR_MAX: u8 = 99;
pub const HOUDINI_VERSION_MINOR_MAX: u8 = 99;
pub const HOUDINI_VERSION_PATCH_MAX: u16 = 9999;
#[derive(Debug, Clone, Validate)]
pub struct HoudiniVersion {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN, max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u8,
    #[validate(range(min=0, max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u8,
    #[validate(range(min=0, max=HOUDINI_VERSION_PATCH_MAX))]
    pub patch: u16,
}
impl HoudiniVersion {
    pub const fn new(major: u8, minor: u8, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const fn major(&self) -> u8 { self.major }

    pub const fn minor(&self) -> u8 { self.minor }

    pub const fn patch(&self) -> u16 { self.patch }
}
impl FromStr for HoudiniVersion {
    type Err = mischief::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> { s.try_into() }
}
impl TryFrom<&str> for HoudiniVersion {
    type Error = mischief::Report;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split('.').collect();
        match parts.len() {
            3 => Ok(Self::new(
                parts[0].parse()?,
                parts[1].parse()?,
                parts[2].parse()?,
            )),
            _ => mischief::bail!("invalid version string"),
        }
    }
}

impl Display for HoudiniVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
#[derive(Debug, Clone, Validate)]
pub struct HoudiniVersionShort {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN, max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u8,
    #[validate(range(min=0, max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u8,
}
impl HoudiniVersionShort {
    pub const fn new(major: u8, minor: u8) -> Self { Self { major, minor } }

    pub const fn major(&self) -> u8 { self.major }

    pub const fn minor(&self) -> u8 { self.minor }
}
impl TryFrom<&str> for HoudiniVersionShort {
    type Error = mischief::Report;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = s.split('.').collect();
        match parts.len() {
            2 => Ok(Self::new(parts[0].parse()?, parts[1].parse()?)),
            _ => mischief::bail!("invalid version string"),
        }
    }
}
impl Display for HoudiniVersionShort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}
impl From<HoudiniVersion> for HoudiniVersionShort {
    fn from(v: HoudiniVersion) -> Self { Self::new(v.major, v.minor) }
}
impl FromStr for HoudiniVersionShort {
    type Err = mischief::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> { s.try_into() }
}
