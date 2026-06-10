#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::Timer;
use rust_app as _;

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
    let mut col1 = Output::new(
        periphs.P0_28,
        embassy_nrf::gpio::Level::Low,
        embassy_nrf::gpio::OutputDrive::Standard,
    );
    row1.set_high();
    col1.set_low();
    loop {
        row1.toggle();
        Timer::after_millis(200).await;
    }
}
