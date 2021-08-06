#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OnOff {
    Disabled,
    Enabled,
}

impl Default for OnOff {
    fn default() -> Self {
        OnOff::Disabled
    }
}
