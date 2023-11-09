use std::sync::Once;

use tracing_subscriber::EnvFilter;

static LOG_INIT: Once = Once::new();

pub fn init_backtrace() {
    LOG_INIT.call_once(|| {
        let filter = EnvFilter::new("info").add_directive("cool=trace".parse().unwrap());

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_env_filter(filter)
            .pretty()
            .init();

        color_eyre::install().unwrap();
    });
}
