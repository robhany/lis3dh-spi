#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode {
    /// High-resolution mode (12-bit data output)
    HighResolution,
    /// Normal mode (10-bit data output)
    Normal,
    /// Low-power mode (8-bit data output)
    LowPower,
}
