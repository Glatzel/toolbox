mod common_arg;
mod houdini;
mod package;
mod preference;
mod sidefx;

use clap::{Parser, Subcommand};
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
pub(crate) use common_arg::{ArgMajor, ArgMinor, ArgNoCheck, ArgPatch, HOUDINI_OPTIONS};
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct VinayaArgs {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Houdini(houdini::Args),
    Package(package::Args),
    Preference(preference::Args),
    Sidefx(sidefx::Args),
}

async fn execute(commands: Commands) -> mischief::Result<()> {
    match commands {
        Commands::Houdini(cmd) => houdini::execute(cmd),
        Commands::Package(cmd) => package::execute(cmd),
        Commands::Preference(cmd) => preference::execute(cmd),
        Commands::Sidefx(cmd) => sidefx::execute(cmd).await,
    }
}
pub async fn main() {
    let args = VinayaArgs::parse();
    //config logger
    let log_level = match args.verbose.filter() {
        clap_verbosity_flag::VerbosityFilter::Off => clerk::LogLevel::OFF,
        clap_verbosity_flag::VerbosityFilter::Trace => clerk::LogLevel::TRACE,
        clap_verbosity_flag::VerbosityFilter::Debug => clerk::LogLevel::DEBUG,
        clap_verbosity_flag::VerbosityFilter::Info => clerk::LogLevel::INFO,
        clap_verbosity_flag::VerbosityFilter::Warn => clerk::LogLevel::WARN,
        clap_verbosity_flag::VerbosityFilter::Error => clerk::LogLevel::ERROR,
    };
    clerk::tracing_subscriber::registry()
        .with(clerk::layer::terminal_layer(log_level, true))
        .init();

    // run
    if let Err(err) = execute(args.command).await {
        eprintln!("{err:?}");
        std::process::exit(1);
    };
}
