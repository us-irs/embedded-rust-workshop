#![no_std]
#![no_main]

use embassy_executor::Spawner;
use rust_app as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    defmt::info!("Hello, world!");
    loop {}
}
