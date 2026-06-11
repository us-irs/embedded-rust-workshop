#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use exercises as _;

use embassy_nrf::gpio::Output;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    defmt::println!("-- microbit v2 Blinky application --");

    let mut row1 = Output::new(
        periphs.P0_21,
        embassy_nrf::gpio::Level::Low,
        embassy_nrf::gpio::OutputDrive::Standard,
    );
    let _col1 = Output::new(
        periphs.P0_28,
        embassy_nrf::gpio::Level::Low,
        embassy_nrf::gpio::OutputDrive::Standard,
    );
    loop {
        row1.toggle();
        Timer::after_millis(200).await;
    }
}
