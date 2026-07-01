use std::{
    io::Read as _,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::Context as _;
use clap::Parser as _;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    serial_port: Option<String>,
}

fn main() -> anyhow::Result<()> {
    host_client::setup_logger().with_context(|| "logger setup")?;
    println!("-- Embedded Rust Workshop host-client --");

    let kill_signal = Arc::new(AtomicBool::new(false));
    let ctrlc_kill_signal = kill_signal.clone();
    ctrlc::set_handler(move || {
        log::info!("Received Ctrl+C, shutting down...");
        ctrlc_kill_signal.store(true, Ordering::Relaxed);
    })
    .unwrap();

    let _cli = Cli::parse();
    let mut config_file =
        host_client::config_file_init().with_context(|| "config file initialization")?;
    let mut toml_str = String::new();
    config_file.read_to_string(&mut toml_str)?;
    let _config: host_client::toml::Config = toml::from_str(&toml_str)?;

    Ok(())
}
