#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use exercises as _;

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

    let mut lsm303agr = lsm303agr::Lsm303agr::new_with_i2c(i2c_bus);
    lsm303agr
        .init()
        .await
        .expect("LSM303 initialization failed");
    lsm303agr
        .set_accel_mode_and_odr(
            &mut Delay,
            lsm303agr::AccelMode::Normal,
            Some(lsm303agr::AccelOutputDataRate::Hz100),
        )
        .await
        .expect("setting acceleration mode and ODR failed");

    loop {
        match lsm303agr.acceleration().await {
            Ok(reading) => {
                defmt::info!("Accelerations (mg): {}", &reading.xyz_mg());
            }
            Err(_e) => {
                defmt::error!("i2c error");
            }
        };
        Timer::after_millis(200).await;
    }
}
