#![no_std]
#![no_main]

use embassy_executor::Spawner;
use exercises as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    defmt::println!("-- microbit v2 UART echo application --");

    // TODOs
    //
    // Step 1: Create a UART driver. You can initialize the UART configuration with the default
    //         configure and then change the baudrate field to Baudrate::BAUD115200
    // Step 2: Split the driver into a RX with IDLE line detection and a RX handle.
    // Step 3: Create a buffer which is used to store received UART data.
    // Step 4: Inside the loop, use the `read_until_idle` method to receive data on the RX pin.
    // Step 5: On successfull reception of RX data, immediately write back the received data using
    //         the `write_all` method of the TX handle you created before.
    loop {}
}
