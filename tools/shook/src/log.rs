use clerk::LevelFilter;
use clerk::tracing_subscriber::Layer;
use clerk::tracing_subscriber::layer::SubscriberExt;
use clerk::tracing_subscriber::util::SubscriberInitExt;
pub fn init_log(level: LevelFilter) {
    // clerk::tracing_subscriber::registry()
    //     .with(
    //         clerk::terminal_layer(true).with_filter(
    //             EnvFilter::builder()
    //                 .with_default_directive(
    //                     format!("{}={}", env!("CARGO_PKG_NAME"),
    // args.verbose.filter())                         .parse()
    //                         .unwrap(),
    //                 )
    //                 .from_env_lossy(),
    //         ),
    //     )
    //     .init();
    clerk::tracing_subscriber::registry()
        .with(clerk::terminal_layer(true).with_filter(clerk::level_filter(level)))
        .init();
}
