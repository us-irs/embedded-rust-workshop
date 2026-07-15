#![no_std]
#![no_main]

use bsp::led::SimpleLedMatrix;
use cobs::CobsDecoderHeapless;
use embassy_executor::Spawner;
use embassy_nrf::{buffered_uarte, peripherals, twim, uarte};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, signal::Signal};
use embassy_time::{Delay, Duration, Timer};
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

    let (mut uart_rx, _uart_tx) = uart.split();

    spawner.spawn(led_task(board.display).expect("spawning led_task failed"));

    // Step 4 (Done)
    //
    // 1. Parse for COBS encoded frames using the
    //    [CobsDecoderHeapless](https://docs.rs/cobs/latest/cobs/struct.CobsDecoderHeapless.html)
    //    structure.
    // 2. Try to interpret the frame as a CCSDS packet, using the spacepackets::CcsdsPacketReader
    // 3. Parse the contained models::request::Request structure
    // 4. Match on the request structure. Do not handle the requests yet.
    let mut rx_buf: [u8; 1024] = [0; 1024];
    let mut cobs_decoder: CobsDecoderHeapless<1024> = CobsDecoderHeapless::new();

    // TODO:
    //
    // Step 5
    //
    // 1. Re-factor the code to push the detected request into a heapless vector of requests
    // 2. Handle the list of requests after having handled all the bytes received from
    //    the UART.
    loop {
        match uart_rx.read(&mut rx_buf).await {
            Ok(read_bytes) => {
                for byte in rx_buf[0..read_bytes].iter() {
                    match cobs_decoder.feed(*byte) {
                        Ok(Some(frame_len)) => {
                            handle_frame(&cobs_decoder.dest()[0..frame_len]);
                        }
                        Ok(None) => (),
                        Err(_) => defmt::error!("COBS decode error"),
                    }
                }
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

pub fn handle_frame(frame: &[u8]) {
    match CcsdsPacketReader::new_with_checksum(frame) {
        Ok(reader) => {
            match postcard::from_bytes::<models::request::Request>(reader.packet_data()) {
                Ok(request) => match request {
                    models::request::Request::Ping => todo!(),
                    models::request::Request::RequestAccelerometer => todo!(),
                    models::request::Request::SetBlinkFrequency(_duration) => todo!(),
                },
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
