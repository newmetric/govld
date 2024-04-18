use govld::cli::Cli;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    color_eyre::install().unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer().with_thread_names(true))
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .init();

    Cli::run().unwrap();
}
