pub const HOUDINI_OPTIONS: &str = "Houdini Options";
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgMajor {
    ///Houdini version major
    #[arg(help_heading=HOUDINI_OPTIONS,long)]
    major: u8,
}
impl ArgMajor {
    pub const fn value(self) -> u8 { self.major }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgMinor {
    ///Houdini version minor
    #[arg(help_heading=HOUDINI_OPTIONS,long)]
    minor: u8,
}
impl ArgMinor {
    pub const fn value(self) -> u8 { self.minor }
}
#[derive(clap::Args, Debug, Clone, Copy, Default)]
#[command(about = None, long_about = None)]
pub struct ArgPatch {
    ///Houdini version patch
    #[arg(help_heading=HOUDINI_OPTIONS,long)]
    patch: u16,
}
impl ArgPatch {
    pub const fn value(self) -> u16 { self.patch }
}
