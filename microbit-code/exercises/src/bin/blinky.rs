#![no_std]
#![no_main]

use exercises as _;
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    // TODOs
    //
    // Step 1: Initialize the hardware by using the `embassy_nrf::init` method.
    // Step 2: print some initial log to the console, for example the exercise name.
    // Step 3: Create an Output GPIO driver for ROW1
    // Step 4: Create an Output GPIO driver for COL1
    // Step 5: Set the GPIOs to the correct state to drive LED D2, which is the LED in
    //         the upper left corner of the LED matrix
    // Step 6: Inside the empty loop, toggle the correct output to toggle the LED.
    // Step 7: Led the LED blink with a certain frequency by using embassy_time::Timer

    loop {}
}
