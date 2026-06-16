use core::cell::RefCell;

#[derive(Debug)]
pub struct InvalidWhoAmIAddrError;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    WhoAmIAcc = 0x0f,
    OutXLowAcc = 0x28,
    CtrlReg1 = 0x20,
    CtrlReg2 = 0x21,
    CtrlReg3 = 0x22,
    CtrlReg4 = 0x23,
    CtrlReg5 = 0x24,
}

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum FullScale {
    _2g = 2,
    _4g = 4,
    _8g = 8,
    _16g = 16,
}

/// Accelerometer mode
#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum Mode {
    /// Power down
    PowerDown,
    /// Low power (8-bit)
    LowPower,
    /// Normal mode (10-bit)
    Normal,
    /// High resolution (12-bit)
    HighResolution,
}

impl Mode {
    pub const fn resolution_shift(&self) -> i16 {
        match self {
            Mode::PowerDown => 0,
            Mode::HighResolution => 4,
            Mode::Normal => 6,
            Mode::LowPower => 8,
        }
    }

    pub const fn resolution_div(&self) -> i16 {
        1 << self.resolution_shift()
    }

    /// The table 14 specifies the scale output values at += 2g in mg/digit.
    ///
    /// At higher full scales, that value needs to be scaled as well. When using a full scale
    /// of 2, notice how the full scale cancels out with the division and you achieve the table
    /// values.
    pub const fn scale_multiplier(&self, full_scale: FullScale) -> u32 {
        let full_scale = full_scale as u32;
        match self {
            Mode::PowerDown => 1,
            Mode::LowPower => (16 * full_scale) / 2,
            Mode::Normal => (4 * full_scale) / 2,
            Mode::HighResolution => (1 * full_scale) / 2,
        }
    }
}

#[derive(Debug, defmt::Format)]
pub struct Readout {
    raw: ReadoutRaw,
    full_scale: FullScale,
    mode: Mode,
}

impl Readout {
    /// X axis readout in mg.
    pub const fn x_mg(&self) -> i32 {
        (self.raw.x >> self.mode.resolution_shift()) as i32
            * self.mode.scale_multiplier(self.full_scale) as i32
    }

    /// Y axis readout in mg.
    pub const fn y_mg(&self) -> i32 {
        (self.raw.y >> self.mode.resolution_shift()) as i32
            * self.mode.scale_multiplier(self.full_scale) as i32
    }

    /// Z axis readout in mg.
    pub const fn z_mg(&self) -> i32 {
        (self.raw.z >> self.mode.resolution_shift()) as i32
            * self.mode.scale_multiplier(self.full_scale) as i32
    }

    /// XYZ axis readout in mg.
    pub const fn xyz_mg(&self) -> (i32, i32, i32) {
        (self.x_mg(), self.y_mg(), self.z_mg())
    }
}

#[derive(Debug, defmt::Format)]
pub struct ReadoutRaw {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

pub type Error = embassy_nrf::twim::Error;

#[derive(Debug)]
pub enum InitError {
    InvalidWhoAmI,
    I2c(Error),
}

impl From<Error> for InitError {
    fn from(value: Error) -> Self {
        Self::I2c(value)
    }
}

/// Output data rate configuration.
#[bitbybit::bitenum(u4, exhaustive = false)]
#[derive(Debug)]
pub enum OdrConfig {
    PowerDown = 0b0000,
    Odr1Hz = 0b0001,
    Odr10Hz = 0b0010,
    Odr25Hz = 0b0011,
    Odr50Hz = 0b0100,
    Odr100Hz = 0b0101,
    Odr200Hz = 0b0110,
    Odr400Hz = 0b0111,
    LowPower1620Hz = 0b1000,
    HrNormal1344HzLowPower5376Hz = 0b1001,
}

#[bitbybit::bitfield(u8, default = 0x0, debug)]
pub struct ControlReg1 {
    #[bits(4..=7, rw)]
    odr: Option<OdrConfig>,
    #[bit(3, rw)]
    low_power_enable: bool,
    #[bit(2, rw)]
    z_enable: bool,
    #[bit(1, rw)]
    y_enable: bool,
    #[bit(0, rw)]
    x_enable: bool,
}

/// Driver for the LSM303AGR e-compass.
pub struct Accelerometer<'d> {
    i2c: RefCell<embassy_nrf::twim::Twim<'d>>,
    full_scale: FullScale,
    mode: Mode,
}

impl<'d> Accelerometer<'d> {
    pub async fn new(mut i2c: embassy_nrf::twim::Twim<'d>) -> Result<Self, InitError> {
        let mut buf = [0; 1];
        i2c.write_read(Self::ADDR, &[Register::WhoAmIAcc as u8], &mut buf)
            .await?;
        if buf[0] != Self::WHO_AM_I_VALUE {
            return Err(InitError::InvalidWhoAmI);
        }
        i2c.write(
            Self::ADDR,
            &[
                Register::CtrlReg1 as u8,
                ControlReg1::builder()
                    .with_odr(OdrConfig::Odr100Hz)
                    .with_low_power_enable(false)
                    .with_z_enable(true)
                    .with_y_enable(true)
                    .with_x_enable(true)
                    .build()
                    .raw_value(),
            ],
        )
        .await?;
        Ok(Self {
            i2c: RefCell::new(i2c),
            full_scale: FullScale::_2g,
            mode: Mode::Normal,
        })
    }
}

impl Accelerometer<'_> {
    pub const ADDR: u8 = 0x19;
    pub const WHO_AM_I_VALUE: u8 = 0b00110011;
    pub const AUTO_INCREMENT_MASK: u8 = 0x80;

    pub async fn read_raw(&self) -> Result<ReadoutRaw, Error> {
        let mut buf = [0; 6];
        self.i2c
            .borrow_mut()
            .write_read(
                Self::ADDR,
                &[Self::AUTO_INCREMENT_MASK | Register::OutXLowAcc as u8],
                &mut buf,
            )
            .await?;
        Ok(ReadoutRaw {
            x: i16::from_le_bytes([buf[0], buf[1]]),
            y: i16::from_le_bytes([buf[2], buf[3]]),
            z: i16::from_le_bytes([buf[4], buf[5]]),
        })
    }

    pub async fn read(&self) -> Result<Readout, Error> {
        Ok(Readout {
            raw: self.read_raw().await?,
            full_scale: self.full_scale,
            mode: self.mode,
        })
    }
}
