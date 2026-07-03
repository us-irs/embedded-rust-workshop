#![no_std]

use arbitrary_int::u11;
pub mod request;
pub mod response;

pub const APID: u11 = u11::new(0x01);

#[cfg(test)]
mod tests {}
