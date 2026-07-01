#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embedded_io_async::Write;
use exercises as _;

use embassy_nrf::{buffered_uarte, peripherals, uarte};

embassy_nrf::bind_interrupts!(
    struct Irqs {
        UARTE0 => buffered_uarte::InterruptHandler<peripherals::UARTE0>;
    }
);

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let periphs = embassy_nrf::init(embassy_nrf::config::Config::default());

    defmt::println!("-- micro:bit UART echo application --");
    let mut uarte_config = uarte::Config::default();
    uarte_config.baudrate = uarte::Baudrate::Baud115200;

    let mut driver_rx_buf: [u8; 256] = [0; 256];
    let mut driver_tx_buf: [u8; 256] = [0; 256];
    let uart = buffered_uarte::BufferedUarte::new(
        periphs.UARTE0,
        periphs.TIMER0,
        periphs.PPI_CH0,
        periphs.PPI_CH1,
        periphs.PPI_GROUP0,
        periphs.P1_08,
        periphs.P0_06,
        Irqs,
        uarte_config,
        &mut driver_rx_buf,
        &mut driver_tx_buf,
    );
    let (mut uart_rx, mut uart_tx) = uart.split();
    let mut rx_buf: [u8; 64] = [0; 64];
    loop {
        match uart_rx.read(&mut rx_buf).await {
            Ok(read_bytes) => {
                defmt::info!("RX: {}", &rx_buf[0..read_bytes]);
                match uart_tx.write_all(&rx_buf[0..read_bytes]).await {
                    Ok(_) => (),
                    Err(_e) => (),
                }
            }
            Err(_e) => (),
        }
    }
}
