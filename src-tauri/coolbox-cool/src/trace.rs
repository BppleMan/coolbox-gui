use std::sync::Once;

use tracing_subscriber::EnvFilter;

static LOG_INIT: Once = Once::new();

pub fn init_backtrace() {
    LOG_INIT.call_once(|| {
        let filter = EnvFilter::new("trace")
            .add_directive("hot_lib_reloader=debug".parse().unwrap())
            .add_directive("winit=info".parse().unwrap())
            .add_directive("egui=info".parse().unwrap())
            .add_directive("eframe=info".parse().unwrap());

        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_env_filter(filter)
            .pretty()
            .init();

        color_eyre::install().unwrap();
    });
}
