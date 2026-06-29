#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::{gpio, gpiote};
use embassy_time::{Duration, Timer};
use exercises::{self as _, board};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut board = board::Microbit::default();

    defmt::println!("-- micro:bit multitasking and IPC application --");
    let mut line_strip = board
        .display
        .line_strip(0)
        .expect("line strip 0 should exist");
    let _button_left_async = gpiote::InputChannel::new(
        board.gpiote_ch0,
        board.btn_a,
        gpio::Pull::Up,
        gpiote::InputChannelPolarity::Toggle,
    );
    let _button_right_async = gpiote::InputChannel::new(
        board.gpiote_ch1,
        board.btn_b,
        gpio::Pull::Up,
        gpiote::InputChannelPolarity::Toggle,
    );

    // TODOs
    //
    // Step 1: Create a second task which prints "The left button was pressed" when the left
    //         button is pressed.
    // Step 2: Create a third task which is scheduled by an interrupt handler. It should print
    //         "The right button was pressed" when the right button is pressed.
    // Step 3: Change the second task. It should notify the main task about the button press
    //         using the embassy-sync Signal mechanism.
    // Step 4: Update the main task to switch off or toggle the second LED periodically depending
    //         on the signal state. Initialize the signal state to be the OFF state for the LED.
    // Step 5: Change the third task to notify the main task about the button press using the
    //         embassy-sync Channel mechanism. It should also measure the duration of the
    //         button press and send that via the channel.
    // Step 6: Update the main task to switch off or toggle the third LED periodically depending
    //         on the messages received from the third task. Start with an initial frequency of
    //         one second.
    //
    loop {
        line_strip.toggle(0);
        Timer::after(Duration::from_millis(500)).await;
    }
}
