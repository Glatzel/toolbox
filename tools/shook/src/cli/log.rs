use clerk::NotInSpanFilter;
use clerk::tracing_subscriber::Layer;
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
use kioyu::{KIOYU_JOB_SPAN, kioyu_layers};

use super::Args;
use crate::cli::RunArgs;
pub fn init_log(args: &Args) {
    let level = args.verbose.tracing_level_filter();
    match &args.commands {
        super::Commands::Init => clerk::tracing_subscriber::registry()
            .with(clerk::terminal_layer(true).with_filter(level))
            .init(),

        super::Commands::Run(RunArgs { config }) => {
            let log_dir = config
                .parent()
                .expect("Config not exist.")
                .join("log")
                .to_path_buf();
            clerk::tracing_subscriber::registry()
                .with(kioyu_layers(log_dir).with_filter(level))
                .with(
                    clerk::terminal_layer(true)
                        .with_filter(NotInSpanFilter(KIOYU_JOB_SPAN))
                        .with_filter(level),
                )
                .init()
        }
    };
}
