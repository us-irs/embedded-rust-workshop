#![no_std]
#![no_main]

use embassy_executor::Spawner;
use exercises as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    defmt::println!("-- micro:bit Accelerometer application --");

    // TODOs
    //
    // Step 1 - Create the I2C/TWI driver.
    // Step 2 - Create the basic driver object inside `src/accelerometers.rs` .
    // Step 3 - Add a constructor to that driver object.
    // Step 4 - Perform some basic minimal sensor configuration inside the constructor.
    // Step 5 - Read and return the raw accelerometer sensor values in a `read_raw` API method of the
    //          driver.
    // Step 6 - Add a data type which wraps the raw readout and is able to return the readout
    //          in the SI unit mg.
    // Step 7 - Read and print the accelerometer values in mg periodically inside your application.
    #[allow(clippy::empty_loop)]
    loop {}
}
