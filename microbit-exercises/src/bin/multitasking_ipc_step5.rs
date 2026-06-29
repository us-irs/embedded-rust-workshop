#![no_std]
#![no_main]

use core::cell::RefCell;

use embassy_executor::Spawner;
use embassy_nrf::{gpio, gpiote, interrupt};
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use exercises::led::LedStrip;
use exercises::{self as _, board};

static INTERRUPT_EXECUTOR: embassy_executor::InterruptExecutor =
    embassy_executor::InterruptExecutor::new();
static SIGNAL_LEFT_BUTTON: Signal<ThreadModeRawMutex, ()> = Signal::new();
static CHANNEL_RIGHT_BUTTON: embassy_sync::channel::Channel<
    CriticalSectionRawMutex,
    embassy_time::Duration,
    4,
> = embassy_sync::channel::Channel::new();

#[derive(Debug, Clone, Copy)]
pub enum BlinkState {
    Toggling,
    Off,
}

impl BlinkState {
    pub fn toggle(&mut self) {
        *self = match self {
            BlinkState::Toggling => BlinkState::Off,
            BlinkState::Off => BlinkState::Toggling,
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut board = board::Microbit::default();

    defmt::println!("-- micro:bit multitasking and IPC application --");
    let line_strip = board
        .display
        .line_strip(0)
        .expect("line strip 0 should exist");
    let button_left_async = gpiote::InputChannel::new(
        board.gpiote_ch0,
        board.btn_a,
        gpio::Pull::Up,
        gpiote::InputChannelPolarity::Toggle,
    );
    let button_right_async = gpiote::InputChannel::new(
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

    let intr_sched_spawner = INTERRUPT_EXECUTOR.start(embassy_nrf::interrupt::Interrupt::QDEC);
    intr_sched_spawner.spawn(right_button_task(button_right_async).unwrap());

    spawner.spawn(left_button_task(button_left_async).expect("spawning task failed"));

    let line_strip_shared = RefCell::new(line_strip);
    loop {
        embassy_futures::select::select(
            main_led_task(&line_strip_shared),
            left_blinky(&line_strip_shared),
        )
        .await;
    }
}

async fn main_led_task<'a>(led_strip: &RefCell<LedStrip<'a>>) {
    loop {
        led_strip.borrow_mut().toggle(0);
        Timer::after(Duration::from_millis(500)).await;
    }
}

async fn left_blinky<'a>(led_strip: &RefCell<LedStrip<'a>>) {
    let mut blink_state = BlinkState::Off;
    loop {
        match embassy_futures::select::select(
            SIGNAL_LEFT_BUTTON.wait(),
            Timer::after(Duration::from_millis(250)),
        )
        .await
        {
            embassy_futures::select::Either::First(_) => {
                blink_state.toggle();
                if let BlinkState::Off = blink_state {
                    led_strip.borrow_mut().off(1);
                }
            }
            embassy_futures::select::Either::Second(_) => {
                if let BlinkState::Toggling = blink_state {
                    led_strip.borrow_mut().toggle(1);
                }
            }
        }
    }
}

#[embassy_executor::task]
async fn left_button_task(mut button_a: gpiote::InputChannel<'static>) {
    loop {
        button_a.wait_for_low().await;
        // Debounce the button.
        embassy_time::Timer::after(Duration::from_millis(20)).await;
        button_a.wait_for_high().await;
        // Debounce the button.
        embassy_time::Timer::after(Duration::from_millis(20)).await;
        SIGNAL_LEFT_BUTTON.signal(());
        defmt::info!("The left button was pressed");
    }
}

#[embassy_executor::task]
async fn right_button_task(mut button_b: gpiote::InputChannel<'static>) {
    loop {
        button_b.wait_for_low().await;
        let now = embassy_time::Instant::now();
        // Debounce the button.
        embassy_time::Timer::after(Duration::from_millis(20)).await;
        button_b.wait_for_high().await;
        let elapsed = embassy_time::Instant::now() - now;
        // Debounce the button.
        embassy_time::Timer::after(Duration::from_millis(20)).await;

        CHANNEL_RIGHT_BUTTON.sender().send(elapsed).await;
        defmt::info!("The right button was pressed");
    }
}

#[interrupt]
fn QDEC() {
    // Safety: We only call this inside the interrupt handler.
    unsafe {
        INTERRUPT_EXECUTOR.on_interrupt();
    }
}
