pub const HOUDINI_OPTIONS: &str = "Houdini Options";
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgMajor {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version major")]
    major: u8,
}
impl ArgMajor {
    pub const fn value(self) -> u8 { self.major }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgMinor {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version minor")]
    minor: u8,
}
impl ArgMinor {
    pub const fn value(self) -> u8 { self.minor }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgPatch {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version patch")]
    patch: u16,
}
impl ArgPatch {
    pub const fn value(self) -> u16 { self.patch }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
pub struct ArgNoCheck {
    #[arg(help_heading=HOUDINI_OPTIONS,long, help = "No check if path or file is existed")]
    no_check: bool,
}
impl ArgNoCheck {
    pub const fn value(self) -> bool { self.no_check }
}
