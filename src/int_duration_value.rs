/// Duration time is measured in N/ODR, where N is the content of the duration register.
/// Duration time steps and maximum values depend on the ODR chosen.
/// ODR is the data rate configuration set in CtrlReg4

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct IntDuration {
    duration: u8,
}

impl IntDuration {
    pub fn set_duration(&mut self, duration: u8) -> Result<(), &'static str> {
        if (duration & 0b1000_0000) == 0b1000_0000 {
            return Err("Interrupt duration can only contain 7 bit");
        }
        self.duration = duration;
        Ok(())
    }
    pub fn duration(&self) -> u8 {
        self.duration
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        IntDuration {
            duration: (value & 0b0111_1111),
        }
    }
    pub(super) fn get_raw_value(&self) -> u8 {
        self.duration
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value = 0b101_0100_u8;
        let int_duration = super::IntDuration::from_raw_value(raw_value);
        assert_eq!(int_duration.duration, raw_value)
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut int_duration = super::IntDuration::default();
        int_duration.set_duration(0b101_1010).unwrap();
        assert_eq!(int_duration.get_raw_value(), 0b101_1010);
    }

    #[test]
    fn setting_duration_fails_if_bit_8_is_set() {
        let mut int_duration = super::IntDuration::default();
        assert!(int_duration.set_duration(0b1101_1010).is_err());
    }
}
