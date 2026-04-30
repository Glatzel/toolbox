use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use clap_verbosity_flag::InfoLevel;
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
use clerk::tracing_subscriber::{EnvFilter, Layer};
use dirs::home_dir;
#[derive(Debug, Parser)]
#[command(author = "Glatzel", version, long_about = None, styles=clap_style::styles())]
pub struct Args {
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity<InfoLevel>,
    #[arg(long, short, default_value_t = 7681)]
    pub port: u16,
    #[arg(long, short,default_value_os_t=home_dir().unwrap_or_default())]
    pub working_directory: PathBuf,
    #[cfg_attr(not(windows), arg(default_value_t=String::from("pwsh")))]
    #[cfg_attr(windows, arg(default_value_t=String::from("pwsh")))]
    pub cmd: String,
}
pub async fn main() -> mischief::Result<()> {
    let args = Args::parse();
    clerk::tracing_subscriber::registry()
        .with(
            clerk::terminal_layer(true).with_filter(
                EnvFilter::builder()
                    .with_default_directive(
                        format!("{}={}", env!("CARGO_PKG_NAME"), args.verbose.filter())
                            .parse()
                            .unwrap(),
                    )
                    .from_env_lossy(),
            ),
        )
        .init();
    let state = Arc::new(crate::server::AppContext { args });
    crate::server::start_server(state).await?;
    Ok(())
}
