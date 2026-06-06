pub const HOUDINI_OPTIONS: &str = "Houdini Options";
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgMajor {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version major")]
    major: u16,
}
impl ArgMajor {
    #[inline(always)]
    pub fn value(&self) -> u16 { self.major }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgMinor {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version minor")]
    minor: u16,
}
impl ArgMinor {
    #[inline(always)]
    pub fn value(&self) -> u16 { self.minor }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgPatch {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version patch")]
    patch: u16,
}
impl ArgPatch {
    #[inline(always)]
    pub fn value(&self) -> u16 { self.patch }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
pub struct ArgNoCheck {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "No check if path or file is existed")]
    no_check: bool,
}
impl ArgNoCheck {
    #[inline(always)]
    pub fn value(&self) -> bool { self.no_check }
}
