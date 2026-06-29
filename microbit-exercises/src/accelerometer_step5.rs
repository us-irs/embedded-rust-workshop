use core::cell::RefCell;

#[derive(Debug, Copy, Clone, PartialEq, Eq, defmt::Format)]
#[repr(u8)]
pub enum Register {
    WhoAmIAcc = 0x0f,
    OutXLowAcc = 0x28,
    CtrlReg1 = 0x20,
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

#[derive(Debug, defmt::Format)]
pub struct ReadoutRaw {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// Step 5 accelerometer driver with raw XYZ readout.
pub struct Accelerometer<'d> {
    i2c: RefCell<embassy_nrf::twim::Twim<'d>>,
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
        })
    }
}

impl Accelerometer<'_> {
    pub const ADDR: u8 = 0x19;
    pub const WHO_AM_I_VALUE: u8 = 0b0011_0011;
    pub const AUTO_INCREMENT_MASK: u8 = 0x80;

    #[allow(clippy::await_holding_refcell_ref)]
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
}
