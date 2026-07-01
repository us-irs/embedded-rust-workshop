#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use embedded_hal_async::delay::DelayNs as _;
use exercises::{self as _, accelerometer_solution::Accelerometer};

use embassy_nrf::{peripherals, twim};

embassy_nrf::bind_interrupts!(
    struct Irqs {
        TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
    }
);

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    defmt::println!("-- micro:bit Accelerometer application --");

    let mut tx_ram_buffer: [u8; 32] = [0; 32];
    let i2c_config = twim::Config::default();
    let i2c_bus = embassy_nrf::twim::Twim::new(
        periphs.TWISPI0,
        Irqs,
        periphs.P0_16,
        periphs.P0_08,
        i2c_config,
        &mut tx_ram_buffer,
    );

    let accelerometer = Accelerometer::new(i2c_bus)
        .await
        .expect("creating motion sensor driver failed");
    // For normal mode, 1.6 ms turn-on time.
    Delay.delay_us(1600).await;

    loop {
        match accelerometer.read().await {
            Ok(reading) => {
                defmt::info!("Accelerations (mg): {}", &reading.xyz_mg());
            }
            Err(e) => {
                defmt::error!("i2c error: {}", e);
            }
        };
        Timer::after_millis(50).await;
    }
}
