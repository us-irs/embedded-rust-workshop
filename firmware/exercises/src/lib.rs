#![no_std]

use defmt_rtt as _;
use embassy_nrf as _;
use panic_probe as _;

pub mod accelerometer;
pub mod accelerometer_solution;
pub mod accelerometer_step3;
pub mod accelerometer_step4;
pub mod accelerometer_step5;

pub mod board;
pub mod led;
