#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{peripherals, twim};
use exercises as _;

embassy_nrf::bind_interrupts!(
    struct Irqs {
        TWISPI0 => twim::InterruptHandler<peripherals::TWISPI0>;
    }
);

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    defmt::println!("-- micro:bit Accelerometer application --");

    // Step 1 solution: Create the I2C/TWI driver.
    let mut tx_ram_buffer: [u8; 32] = [0; 32];
    let i2c_config = twim::Config::default();
    let _i2c_bus = embassy_nrf::twim::Twim::new(
        periphs.TWISPI0,
        Irqs,
        periphs.P0_16,
        periphs.P0_08,
        i2c_config,
        &mut tx_ram_buffer,
    );

    #[allow(clippy::empty_loop)]
    loop {}
}
