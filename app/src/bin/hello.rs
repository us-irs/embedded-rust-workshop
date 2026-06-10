#![no_std]
#![no_main]

use rust_app as _;
use embassy_executor::Spawner;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    defmt::info!("Hello, world!");
    loop {}
}
