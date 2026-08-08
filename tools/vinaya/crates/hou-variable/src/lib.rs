use core::fmt::Display;

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
    pub const fn new(major: u8, minor: u8, patch: Option<u16>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const fn major(&self) -> u8 { self.major }

    pub const fn minor(&self) -> u8 { self.minor }

    pub const fn patch(&self) -> Option<u16> { self.patch }
}
impl TryFrom<&str> for HoudiniVersion {
    type Error = mischief::Report;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
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
impl Display for HoudiniVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.patch {
            Some(patch) => write!(f, "{}.{}.{}", self.major, self.minor, patch),
            None => write!(f, "{}.{}", self.major, self.minor),
        }
    }
}
