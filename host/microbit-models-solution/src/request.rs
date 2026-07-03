#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Request {
    Ping,
    RequestAccelerometer,
    SetBlinkFrequency(core::time::Duration),
}
