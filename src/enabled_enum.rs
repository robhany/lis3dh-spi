#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnOff {
    Disabled,
    Enabled,
}

impl Default for OnOff {
    fn default() -> Self {
        OnOff::Disabled
    }
}

pub(crate) fn get_state_from_bit_value(value: u8) -> OnOff {
    if value & 1 == 1 {
        OnOff::Enabled
    } else {
        OnOff::Disabled
    }
}
