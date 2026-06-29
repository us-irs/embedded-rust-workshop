#[derive(Debug, Copy, Clone, PartialEq, Eq, defmt::Format)]
#[repr(u8)]
pub enum Register {
    WhoAmIAcc = 0x0f,
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

/// Step 3 accelerometer driver with WHO_AM_I validation in the constructor.
pub struct Accelerometer<'d> {
    i2c: embassy_nrf::twim::Twim<'d>,
}

impl<'d> Accelerometer<'d> {
    pub async fn new(mut i2c: embassy_nrf::twim::Twim<'d>) -> Result<Self, InitError> {
        let mut buf = [0; 1];
        i2c.write_read(Self::ADDR, &[Register::WhoAmIAcc as u8], &mut buf)
            .await?;

        if buf[0] != Self::WHO_AM_I_VALUE {
            return Err(InitError::InvalidWhoAmI);
        }

        Ok(Self { i2c })
    }
}

impl Accelerometer<'_> {
    pub const ADDR: u8 = 0x19;
    pub const WHO_AM_I_VALUE: u8 = 0b0011_0011;
}
