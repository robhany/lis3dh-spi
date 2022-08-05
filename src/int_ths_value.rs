/// The value passed to the threshold is multiplied with a factor that is dependant from the
/// the FullScaleSelection set in CtrlReg4.
///
/// If you want to set the threshold to 250 mg and the scale in register 4 is set to 2G you have to
/// send the value 0x10 to IntThs register
///
/// | FullScaleSelection|   Multiplier |
/// |:------------------|:-------------|
/// |  2G               |  16 mg       |
/// |  4G               |  32 mg       |
/// |  8G               |  62 mg       |
/// | 16G               | 186 mg       |
/// |:------------------|:-------------|

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct IntThs {
    threshold: u8,
}

impl IntThs {
    pub fn set_threshold(&mut self, threshold: u8) -> Result<(), &'static str> {
        if (threshold & 0b1000_0000) == 0b1000_0000 {
            return Err("Interrupt threshold can only contain 7 bit");
        }
        self.threshold = threshold;
        Ok(())
    }
    pub fn threshold(&self) -> u8 {
        self.threshold
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        IntThs {
            threshold: (value & 0b0111_1111),
        }
    }
    pub(super) fn get_raw_value(&self) -> u8 {
        self.threshold
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value = 0b101_0100_u8;
        let int_ths = super::IntThs::from_raw_value(raw_value);
        assert_eq!(int_ths.threshold(), raw_value)
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut int_ths = super::IntThs::default();
        int_ths.set_threshold(0b101_1010).unwrap();
        assert_eq!(int_ths.get_raw_value(), 0b101_1010);
    }

    #[test]
    fn setting_threshold_fails_if_bit_8_is_set() {
        let mut intths = super::IntThs::default();
        assert!(intths.set_threshold(0b1101_1010).is_err());
    }
}
