use embassy_nrf::{Peri, peripherals};

use crate::led::SimpleLedMatrix;

/// Represents all the peripherals and pins available for the BBC micro:bit.
pub struct Microbit {
    /// LED matrix display
    pub display: SimpleLedMatrix,
    /// Button 'A'
    pub btn_a: Peri<'static, peripherals::P0_14>,
    /// Button 'B'
    pub btn_b: Peri<'static, peripherals::P0_23>,
    /// UART0 peripheral
    pub uarte0: Peri<'static, peripherals::UARTE0>,
    /// UART1 peripheral
    pub uarte1: Peri<'static, peripherals::UARTE1>,
    /// TIMER0 peripheral
    pub timer0: Peri<'static, peripherals::TIMER0>,
    /// Speaker pin
    pub speaker: Peri<'static, peripherals::P0_00>,
    /// Microphone pin
    pub microphone: Peri<'static, peripherals::P0_05>,
    /// Microphone pin enable
    pub micen: Peri<'static, peripherals::P0_20>,

    /// P0 connector pin
    pub p0: Peri<'static, peripherals::P0_02>,
    /// P1 connector pin
    pub p1: Peri<'static, peripherals::P0_03>,
    /// P2 connector pin
    pub p2: Peri<'static, peripherals::P0_04>,
    /// P8 connector pin
    pub p8: Peri<'static, peripherals::P0_10>,
    /// P9 connector pin
    pub p9: Peri<'static, peripherals::P0_09>,
    /// P12 connector pin
    pub p12: Peri<'static, peripherals::P0_12>,
    /// P13 connector pin
    pub p13: Peri<'static, peripherals::P0_17>,
    /// P14 connector pin
    pub p14: Peri<'static, peripherals::P0_01>,
    /// P15 connector pin
    pub p15: Peri<'static, peripherals::P0_13>,
    /// P16 connector pin
    pub p16: Peri<'static, peripherals::P1_02>,
    /// P19 connector pin
    pub p19: Peri<'static, peripherals::P0_26>,
    /// P20 connector pin
    pub p20: Peri<'static, peripherals::P1_00>,

    /// Internal I2C/TWI SCL to accelerometer & debug MCU
    pub i2c_int_scl: Peri<'static, peripherals::P0_08>,
    /// Internal I2C/TWI SDA to accelerometer & debug MCU
    pub i2c_int_sda: Peri<'static, peripherals::P0_16>,

    /// UART TX to debug MCU
    pub uart_int_tx: Peri<'static, peripherals::P1_08>,
    /// UART RX to debug MCU
    pub uart_int_rx: Peri<'static, peripherals::P0_06>,

    /// SPI0/I2C0 peripheral
    pub twispi0: Peri<'static, peripherals::TWISPI0>,
    /// SPI1/I2C1 peripheral
    pub twispi1: Peri<'static, peripherals::TWISPI1>,
    /// SPI2 peripheral
    pub spi2: Peri<'static, peripherals::SPI2>,
    /// SPI3 peripheral
    pub spi3: Peri<'static, peripherals::SPI3>,
    /// PWM0 peripheral
    pub pwm0: Peri<'static, peripherals::PWM0>,
    /// PWM1 peripheral
    pub pwm1: Peri<'static, peripherals::PWM1>,
    /// PWM2 peripheral
    pub pwm2: Peri<'static, peripherals::PWM2>,
    /// PWM3 peripheral
    pub pwm3: Peri<'static, peripherals::PWM3>,
    /// PPI channel 0
    pub ppi_ch0: Peri<'static, peripherals::PPI_CH0>,
    /// PPI channel 1
    pub ppi_ch1: Peri<'static, peripherals::PPI_CH1>,
    /// Random number generator
    pub rng: Peri<'static, peripherals::RNG>,
    /// Analog digital converter
    pub saadc: Peri<'static, peripherals::SAADC>,
    pub gpiote_ch0: Peri<'static, peripherals::GPIOTE_CH0>,
    pub gpiote_ch1: Peri<'static, peripherals::GPIOTE_CH1>,
    pub gpiote_ch2: Peri<'static, peripherals::GPIOTE_CH2>,
    pub gpiote_ch3: Peri<'static, peripherals::GPIOTE_CH3>,
}

impl Microbit {
    /// Create a new instance based on HAL configuration
    pub fn new(config: embassy_nrf::config::Config) -> Self {
        let p = embassy_nrf::init(config);

        Self {
            display: SimpleLedMatrix::new(
                (p.P0_21, p.P0_22, p.P0_15, p.P0_24, p.P0_19),
                (p.P0_28, p.P0_11, p.P0_31, p.P1_05, p.P0_30),
            ),
            btn_a: p.P0_14,
            btn_b: p.P0_23,
            uarte0: p.UARTE0,
            uarte1: p.UARTE1,
            timer0: p.TIMER0,
            speaker: p.P0_00,
            microphone: p.P0_05,
            micen: p.P0_20,
            p0: p.P0_02,
            p1: p.P0_03,
            p2: p.P0_04,
            p8: p.P0_10,
            p9: p.P0_09,
            p12: p.P0_12,
            p13: p.P0_17,
            p14: p.P0_01,
            p15: p.P0_13,
            p16: p.P1_02,
            p19: p.P0_26,
            p20: p.P1_00,
            i2c_int_scl: p.P0_08,
            i2c_int_sda: p.P0_16,
            uart_int_tx: p.P1_08,
            uart_int_rx: p.P0_06,
            ppi_ch0: p.PPI_CH0,
            ppi_ch1: p.PPI_CH1,
            twispi0: p.TWISPI0,
            twispi1: p.TWISPI1,
            spi2: p.SPI2,
            spi3: p.SPI3,
            pwm0: p.PWM0,
            pwm1: p.PWM1,
            pwm2: p.PWM2,
            pwm3: p.PWM3,
            rng: p.RNG,
            saadc: p.SAADC,
            gpiote_ch0: p.GPIOTE_CH0,
            gpiote_ch1: p.GPIOTE_CH1,
            gpiote_ch2: p.GPIOTE_CH2,
            gpiote_ch3: p.GPIOTE_CH3,
        }
    }
}

impl Default for Microbit {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
