use client_solution as client;
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
    /// Serial port used for communication with the micro:bit v2
    #[arg(short, long)]
    serial_port: Option<String>,
    /// Send a ping to the device.
    #[arg(long)]
    ping: bool,
}

fn main() -> anyhow::Result<()> {
    client::setup_logger().with_context(|| "logger setup")?;
    println!("-- Embedded Rust Workshop host-client --");

    let kill_signal = Arc::new(AtomicBool::new(false));
    let ctrlc_kill_signal = kill_signal.clone();
    ctrlc::set_handler(move || {
        log::info!("Received Ctrl+C, shutting down...");
        ctrlc_kill_signal.store(true, Ordering::Relaxed);
    })
    .unwrap();

    let cli = Cli::parse();
    let mut config_file =
        client::config_file_init().with_context(|| "config file initialization")?;
    let mut toml_str = String::new();
    config_file.read_to_string(&mut toml_str)?;
    let config: client::toml::Config = toml::from_str(&toml_str)?;

    let serial_port = cli.serial_port.unwrap_or(config.serial_port);

    log::info!("Connecting to serial port: {}", serial_port);
    let mut serial_transport =
        tmtc_utils::transport::serial::PacketTransportSerialCobs::new_from_params(
            &serial_port,
            115200,
            4096,
        )
        .with_context(|| format!("opening serial port {}", serial_port))?;

    loop {
        serial_transport
            .receive(|_packet| {
                // TODO: Handle our decoded packets received from the firmware here.
            })
            .with_context(|| "serial reception failed")?;
        if kill_signal.load(Ordering::Relaxed) {
            log::info!("Shutting down...");
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
