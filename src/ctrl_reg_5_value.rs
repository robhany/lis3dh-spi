use crate::enabled_enum;
use crate::enabled_enum::OnOff;

const D4D_INT2_BIT_OFFSET: u8 = 0;
const LIR_INT2_BIT_OFFSET: u8 = 1;
const D4D_INT1_BIT_OFFSET: u8 = 2;
const LIR_INT1_BIT_OFFSET: u8 = 3;
const FIFO_EN_BIT_OFFSET: u8 = 6;
const BOOT_BIT_OFFSET: u8 = 7;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BootMode {
    NormalMode,
    RebootMemoryContent,
}

impl Default for BootMode {
    fn default() -> Self {
        BootMode::NormalMode
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct CtrlReg5Value {
    boot: BootMode,
    fifo: OnOff,
    latch_int_on_int_1_src: OnOff,
    d4_detection_on_int_1: OnOff,
    latch_int_on_int_2_src: OnOff,
    d4_detection_on_int_2: OnOff,
}

impl CtrlReg5Value {
    pub fn set_boot(&mut self, boot: BootMode) {
        self.boot = boot;
    }
    pub fn set_fifo(&mut self, fifo: OnOff) {
        self.fifo = fifo;
    }
    pub fn set_latch_int_on_int_1_src(
        &mut self,
        latch_int_on_int_1_src: OnOff,
    ) {
        self.latch_int_on_int_1_src = latch_int_on_int_1_src;
    }
    pub fn set_d4_detection_on_int_1(&mut self, d4_detection_on_int_1: OnOff) {
        self.d4_detection_on_int_1 = d4_detection_on_int_1;
    }
    pub fn set_latch_int_on_int_2_src(
        &mut self,
        latch_int_on_int_2_src: OnOff,
    ) {
        self.latch_int_on_int_2_src = latch_int_on_int_2_src;
    }
    pub fn set_d4_detection_on_int_2(&mut self, d4_detection_on_int_2: OnOff) {
        self.d4_detection_on_int_2 = d4_detection_on_int_2;
    }
    pub fn boot(&self) -> BootMode {
        self.boot
    }
    pub fn fifo(&self) -> OnOff {
        self.fifo
    }
    pub fn latch_int_on_int_1_src(&self) -> OnOff {
        self.latch_int_on_int_1_src
    }
    pub fn d4_detection_on_int_1(&self) -> OnOff {
        self.d4_detection_on_int_1
    }
    pub fn latch_int_on_int_2_src(&self) -> OnOff {
        self.latch_int_on_int_2_src
    }
    pub fn d4_detection_on_int_2(&self) -> OnOff {
        self.d4_detection_on_int_2
    }
    pub(super) fn get_raw_value(&self) -> u8 {
        (self.boot as u8) << BOOT_BIT_OFFSET
            | (self.fifo as u8) << FIFO_EN_BIT_OFFSET
            | (self.latch_int_on_int_1_src as u8) << LIR_INT1_BIT_OFFSET
            | (self.d4_detection_on_int_1 as u8) << D4D_INT1_BIT_OFFSET
            | (self.latch_int_on_int_2_src as u8) << LIR_INT2_BIT_OFFSET
            | (self.d4_detection_on_int_2 as u8) << LIR_INT2_BIT_OFFSET
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        CtrlReg5Value {
            boot: if (value >> BOOT_BIT_OFFSET) == 0 {
                BootMode::NormalMode
            } else {
                BootMode::RebootMemoryContent
            },
            fifo: enabled_enum::get_state_from_bit_value(
                value >> FIFO_EN_BIT_OFFSET,
            ),
            latch_int_on_int_1_src: enabled_enum::get_state_from_bit_value(
                value >> LIR_INT1_BIT_OFFSET,
            ),
            d4_detection_on_int_1: enabled_enum::get_state_from_bit_value(
                value >> D4D_INT1_BIT_OFFSET,
            ),
            latch_int_on_int_2_src: enabled_enum::get_state_from_bit_value(
                value >> LIR_INT2_BIT_OFFSET,
            ),
            d4_detection_on_int_2: enabled_enum::get_state_from_bit_value(
                value >> D4D_INT2_BIT_OFFSET,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ctrl_reg_5_value::BootMode;
    use crate::enabled_enum::OnOff;

    #[test]
    fn conversion_from_raw_value_works() {
        let ctrl_reg_5_value_raw = 0b1000_0101_u8;
        let ctrl_reg_5_value =
            super::CtrlReg5Value::from_raw_value(ctrl_reg_5_value_raw);
        assert_eq!(
            ctrl_reg_5_value,
            super::CtrlReg5Value {
                boot: BootMode::RebootMemoryContent,
                fifo: OnOff::Disabled,
                latch_int_on_int_1_src: OnOff::Disabled,
                d4_detection_on_int_1: OnOff::Enabled,
                latch_int_on_int_2_src: OnOff::Disabled,
                d4_detection_on_int_2: OnOff::Enabled,
            }
        );
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let ctrl_reg_5_value = super::CtrlReg5Value {
            boot: BootMode::RebootMemoryContent,
            fifo: OnOff::Disabled,
            latch_int_on_int_1_src: OnOff::Disabled,
            d4_detection_on_int_1: OnOff::Enabled,
            latch_int_on_int_2_src: OnOff::Disabled,
            d4_detection_on_int_2: OnOff::Disabled,
        };
        assert_eq!(ctrl_reg_5_value.get_raw_value(), 0b1000_0100);
    }
}
