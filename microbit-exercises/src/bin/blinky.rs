#![no_std]
#![no_main]

use embassy_executor::Spawner;
use exercises as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    // TODOs
    //
    // Step 1: Initialize the hardware by using the `embassy_nrf::init` method.
    // Step 2: Print some initial log to the console, for example the exercise name.
    // Step 3: Create Output GPIO drivers for ROW1 and COL1. You can use the datasheet provided at
    //         the repo root or the images inside the exercise book to extract the relevant pin
    //         information.
    // Step 4: Inside the empty loop, toggle the correct output (only one is required) to toggle the LED
    //         with a certain frequency by using embassy_time::Timer.

    loop {}
}
