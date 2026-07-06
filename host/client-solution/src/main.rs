use client_solution as client;
use microbit_models_solution as models;
use spacepackets::{CcsdsPacketCreatorOwned, CcsdsPacketReader, SpHeader};
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
    /// Request accelerometer data.
    #[arg(long)]
    accelerometer: bool,
    /// Set new blink frequency.
    #[arg(long)]
    set_blink_freq_ms: Option<u32>,
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
    if cli.ping {
        let tc = create_tc(models::request::Request::Ping).with_context(|| "creating ping TC")?;
        serial_transport
            .send(&tc.to_vec())
            .with_context(|| "sending ping TC")?;
    }
    if cli.accelerometer {
        let tc = create_tc(models::request::Request::RequestAccelerometer)
            .with_context(|| "creating ping TC")?;
        serial_transport
            .send(&tc.to_vec())
            .with_context(|| "sending ping TC")?;
    }
    if let Some(new_blink_freq_ms) = cli.set_blink_freq_ms {
        let tc = create_tc(models::request::Request::SetBlinkFrequency(
            core::time::Duration::from_millis(new_blink_freq_ms as u64),
        ))
        .with_context(|| "creating ping TC")?;
        serial_transport
            .send(&tc.to_vec())
            .with_context(|| "sending ping TC")?;
    }

    loop {
        serial_transport
            .receive(
                |packet| match CcsdsPacketReader::new_with_checksum(packet) {
                    Ok(packet) => match parse_response(packet) {
                        Ok(response) => {
                            log::info!("RX response: {:?}", response);
                        }
                        Err(e) => {
                            log::error!("Failed to parse response: {:?}", e);
                        }
                    },
                    Err(e) => {
                        log::error!("Failed to read packet: {:?}", e);
                    }
                },
            )
            .with_context(|| "serial reception failed")?;
        if kill_signal.load(Ordering::Relaxed) {
            log::info!("Shutting down...");
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}

pub fn create_tc(request: models::request::Request) -> anyhow::Result<CcsdsPacketCreatorOwned> {
    let request_raw = postcard::to_allocvec(&request).unwrap();
    CcsdsPacketCreatorOwned::new_with_checksum(
        SpHeader::new_from_apid(models::APID),
        spacepackets::PacketType::Tc,
        &request_raw,
    )
    .with_context(|| "creating TC packet")
}

pub fn parse_response(reader: CcsdsPacketReader) -> anyhow::Result<models::response::Response> {
    let user_data = reader.packet_data();
    let response = postcard::from_bytes::<models::response::Response>(user_data)
        .with_context(|| "parsing TM response")?;
    Ok(response)
}
