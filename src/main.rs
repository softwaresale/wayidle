mod app;
mod args;
mod command;
mod config;
mod err;
mod idle;

use std::error::Error;

use clap::Parser;
use log::{LevelFilter, error, info};
use wayland_client::Connection;

use crate::{
    app::{WayIdleApp, WayIdleUserState},
    args::WayIdleArgs,
    config::WayIdleConfig,
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();

    let args = WayIdleArgs::parse();

    let wayidle_config = WayIdleConfig::load(args.config_file())?;

    info!("starting wayidle...");

    let conn = match Connection::connect_to_env() {
        Ok(conn) => {
            info!("successfully connected to wayland");
            conn
        }
        Err(err) => {
            error!("failed to connect to wayland: {}", err);
            return Err(err.into());
        }
    };

    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qhandle = event_queue.handle();

    let user_data = WayIdleUserState::from(&wayidle_config);
    let mut app = WayIdleApp::default();

    let _registry = display.get_registry(&qhandle, user_data);

    event_queue.roundtrip(&mut app)?;

    loop {
        event_queue.blocking_dispatch(&mut app)?;
    }
}
