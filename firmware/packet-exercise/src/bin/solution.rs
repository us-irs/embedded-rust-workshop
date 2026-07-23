#![no_std]
#![no_main]

use bsp::led::SimpleLedMatrix;
use cobs::CobsDecoderHeapless;
use embassy_executor::Spawner;
use embassy_nrf::{buffered_uarte, peripherals, twim, uarte};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::{Delay, Duration, Timer};
use embedded_io_async::Write as _;
use microbit_models_solution as models;
use packet_exercise as _;
use spacepackets::CcsdsPacketReader;

embassy_nrf::bind_interrupts!(
    struct Irqs {
        UARTE0 => buffered_uarte::InterruptHandler<peripherals::UARTE0>;
        TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
    }
);

static LED_TOGGLE_FREQ_UPDATE: Signal<ThreadModeRawMutex, embassy_time::Duration> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let board = bsp::Microbit::default();

    defmt::println!("-- micro:bit packet and serialization application --");

    let mut tx_ram_buffer: [u8; 32] = [0; 32];
    let i2c_config = twim::Config::default();
    let i2c_bus = embassy_nrf::twim::Twim::new(
        board.twispi0,
        Irqs,
        board.i2c_int_sda,
        board.i2c_int_scl,
        i2c_config,
        &mut tx_ram_buffer,
    );

    // Create a driver for the sensor that we can use to read sensor values.
    let mut lsm303agr = lsm303agr::Lsm303agr::new_with_i2c(i2c_bus);
    lsm303agr
        .init()
        .await
        .expect("LSM303 initialization failed");
    lsm303agr
        .set_accel_mode_and_odr(
            &mut Delay,
            lsm303agr::AccelMode::Normal,
            Some(lsm303agr::AccelOutputDataRate::Hz100),
        )
        .await
        .expect("setting acceleration mode and ODR failed");

    // Create a UART driver we can use to receive telecommands and send telemetry.
    let mut uarte_config = uarte::Config::default();
    uarte_config.baudrate = uarte::Baudrate::Baud115200;

    let mut driver_rx_buf: [u8; 256] = [0; 256];
    let mut driver_tx_buf: [u8; 256] = [0; 256];
    let uart = buffered_uarte::BufferedUarte::new(
        board.uarte0,
        board.timer0,
        board.ppi_ch0,
        board.ppi_ch1,
        board.ppi_group0,
        board.uart_rx,
        board.uart_tx,
        Irqs,
        uarte_config,
        &mut driver_rx_buf,
        &mut driver_tx_buf,
    );

    let (mut uart_rx, mut uart_tx) = uart.split();

    spawner.spawn(led_task(board.display).expect("spawning led_task failed"));

    let mut rx_buf: [u8; 1024] = [0; 1024];
    let mut tm_buf: [u8; 1024] = [0; 1024];
    let mut encoded_tm_buf: [u8; cobs::max_encoding_length(1024)] =
        [0; cobs::max_encoding_length(1024)];
    let mut cobs_decoder: CobsDecoderHeapless<1024> = CobsDecoderHeapless::new();
    let mut request_queue = heapless::vec::Vec::new();
    loop {
        match uart_rx.read(&mut rx_buf).await {
            Ok(read_bytes) => {
                for byte in rx_buf[0..read_bytes].iter() {
                    match cobs_decoder.feed(*byte) {
                        Ok(Some(frame_len)) => {
                            handle_frame(&cobs_decoder.dest()[0..frame_len], &mut request_queue);
                        }
                        Ok(None) => (),
                        Err(_) => defmt::error!("COBS decode error"),
                    }
                }
                for request in request_queue.iter() {
                    let tm_len = match request {
                        models::request::Request::Ping => {
                            defmt::info!("received ping request");
                            create_telemetry(
                                &mut tm_buf,
                                models::response::Response::CommandCompleted,
                            )
                        }
                        models::request::Request::RequestAccelerometer => {
                            defmt::info!("received accelerometer read request");
                            match lsm303agr.acceleration().await {
                                Ok(data) => create_telemetry(
                                    &mut tm_buf,
                                    models::response::Response::AccelerometerData(
                                        models::response::AccelerometerData {
                                            x_mg: data.x_mg() as i16,
                                            y_mg: data.y_mg() as i16,
                                            z_mg: data.z_mg() as i16,
                                        },
                                    ),
                                ),
                                Err(_e) => {
                                    defmt::error!("Failed to read accelerometer data");
                                    0
                                }
                            }
                        }
                        models::request::Request::SetBlinkFrequency(duration) => {
                            defmt::info!(
                                "received set blink frequency request: {:?} ms",
                                duration.as_millis()
                            );
                            let embassy_duration =
                                Duration::from_millis(duration.as_millis() as u64);
                            LED_TOGGLE_FREQ_UPDATE.signal(embassy_duration);
                            create_telemetry(
                                &mut tm_buf,
                                models::response::Response::CommandCompleted,
                            )
                        }
                    };
                    if tm_len > 0 {
                        match cobs::try_encode_including_sentinels(
                            &tm_buf[0..tm_len],
                            &mut encoded_tm_buf,
                        ) {
                            Ok(encoded_len) => {
                                if let Err(e) =
                                    uart_tx.write_all(&encoded_tm_buf[0..encoded_len]).await
                                {
                                    defmt::error!("Failed to send telemetry: {:?}", e);
                                }
                            }
                            Err(_e) => defmt::error!("COBS encdoing buffer too small"),
                        }
                    }
                }
                request_queue.clear();
            }
            Err(_e) => (),
        }
    }
}

#[embassy_executor::task]
async fn led_task(mut display: SimpleLedMatrix) {
    let mut led_strip = display.line_strip(0).expect("line strip 0 should exist");
    let mut blink_freq = Duration::from_millis(200);
    loop {
        led_strip.toggle(0);

        match embassy_futures::select::select(
            Timer::after(blink_freq),
            LED_TOGGLE_FREQ_UPDATE.wait(),
        )
        .await
        {
            embassy_futures::select::Either::First(_) => (),
            embassy_futures::select::Either::Second(new_freq) => {
                blink_freq = new_freq;
            }
        }
    }
}

pub fn handle_frame(frame: &[u8], request_list: &mut heapless::Vec<models::request::Request, 8>) {
    match CcsdsPacketReader::new_with_checksum(frame) {
        Ok(reader) => {
            match postcard::from_bytes::<models::request::Request>(reader.packet_data()) {
                Ok(request) => {
                    if request_list.is_full() {
                        defmt::error!("Request queue is full, dropping request: {}", request);
                    }
                    request_list.push(request).unwrap()
                }
                Err(e) => {
                    defmt::error!("Failed to parse request: {:?}", e);
                }
            }
        }
        Err(e) => {
            defmt::error!("Failed to read packet: {:?}", e);
        }
    }
}

pub fn create_telemetry(tm_buf: &mut [u8], response: models::response::Response) -> usize {
    defmt::info!("Creating telemetry for response: {:?}", response);
    let response_size = postcard::experimental::serialized_size(&response);
    if let Err(e) = response_size {
        defmt::error!("Failed to get size of response: {}", e);
        return 0;
    }
    let packet_creator_result =
        spacepackets::CcsdsPacketCreatorWithReservedData::new_tm_with_checksum(
            spacepackets::SpHeader::new_from_apid(models::APID),
            response_size.unwrap(),
            tm_buf,
        );
    if let Err(e) = packet_creator_result {
        defmt::error!("Failed to create packet: {}", e);
        return 0;
    }
    let mut packet_creator = packet_creator_result.unwrap();

    if let Err(e) = postcard::to_slice(&response, packet_creator.packet_data_mut()) {
        defmt::error!("Failed to serialize response: {}", e);
        return 0;
    }
    packet_creator.finish()
}
