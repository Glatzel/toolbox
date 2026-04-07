mod common_arg;
mod houdini;
mod package;
mod preference;
mod sidefx;
use clap::{Parser, Subcommand};
use clerk::tracing_subscriber::Layer;
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
    clerk::tracing_subscriber::registry()
        .with(
            clerk::terminal_layer(true)
                .with_filter(clerk::level_filter(args.verbose.tracing_level_filter())),
        )
        .init();

    // run
    if let Err(err) = execute(args.command).await {
        eprintln!("{err:?}");
        std::process::exit(1);
    };
}
