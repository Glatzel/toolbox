use std::{path::PathBuf, sync::Arc};

use clap::Parser;
use dirs::home_dir;
#[derive(Debug, Parser)]
#[command(author = "Glatzel", version, long_about = None)]
pub struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[arg(long, short, default_value_t = 7681)]
    pub port: u16,
    #[arg(long, short,default_value_os_t=home_dir().unwrap_or_default())]
    pub working_directory: PathBuf,
    pub shell: String,
}
pub async fn main() -> mischief::Result<()> {
    let args = Args::parse();
    clerk::init_log_with_level(args.verbose.tracing_level_filter());
    let state = Arc::new(crate::server::AppContext { args });
    crate::server::start_server(state).await?;
    Ok(())
}
