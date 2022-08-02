const XL_BIT_OFFSET: u8 = 0;
const XH_BIT_OFFSET: u8 = 1;
const YL_BIT_OFFSET: u8 = 2;
const YH_BIT_OFFSET: u8 = 3;
const ZL_BIT_OFFSET: u8 = 4;
const ZH_BIT_OFFSET: u8 = 5;
const IA_BIT_OFFSET: u8 = 6;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct IntSrc {
    interrupt_active: bool,
    z_high: bool,
    z_low: bool,
    y_high: bool,
    y_low: bool,
    x_high: bool,
    x_low: bool,
}

impl IntSrc {
    pub fn is_interrupt_active(&self) -> bool {
        self.interrupt_active
    }
    pub fn is_z_high(&self) -> bool {
        self.z_high
    }
    pub fn is_z_low(&self) -> bool {
        self.z_low
    }
    pub fn is_y_high(&self) -> bool {
        self.y_high
    }
    pub fn is_y_low(&self) -> bool {
        self.y_low
    }
    pub fn is_x_high(&self) -> bool {
        self.x_high
    }
    pub fn is_x_low(&self) -> bool {
        self.x_low
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        IntSrc {
            interrupt_active: (value >> IA_BIT_OFFSET) & 1 == 1,
            z_high: (value >> ZH_BIT_OFFSET) & 1 == 1,
            z_low: (value >> ZL_BIT_OFFSET) & 1 == 1,
            y_high: (value >> YH_BIT_OFFSET) & 1 == 1,
            y_low: (value >> YL_BIT_OFFSET) & 1 == 1,
            x_high: (value >> XH_BIT_OFFSET) & 1 == 1,
            x_low: (value >> XL_BIT_OFFSET) & 1 == 1,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value = 0b0101_0100_u8;
        let int_src = super::IntSrc::from_raw_value(raw_value);
        assert!(int_src.is_y_low());
        assert!(int_src.is_z_low());
        assert!(int_src.is_interrupt_active());
    }
}
