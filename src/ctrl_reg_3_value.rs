use crate::enabled_enum;
use crate::enabled_enum::OnOff;

const OVERRUN_BIT_OFFSET: u8 = 1;
const WTM_BIT_OFFSET: u8 = 2;
const DA_321_BIT_OFFSET: u8 = 3;
const ZYX_DA_BIT_OFFSET: u8 = 4;
const IA_2_BIT_OFFSET: u8 = 5;
const IA_1_BIT_OFFSET: u8 = 6;
const CLICK_BIT_OFFSET: u8 = 7;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct CtrlReg3Value {
    interrupt_1_click: OnOff,
    interrupt_1_ia1: OnOff,
    interrupt_1_ia2: OnOff,
    interrupt_1_zyx_da: OnOff,
    interrupt_1_321_da: OnOff,
    interrupt_1_fifo_watermark: OnOff,
    interrupt_1_fifo_overrun: OnOff,
}

impl CtrlReg3Value {
    pub fn set_interrupt_1_click(&mut self, interrupt_1_click: OnOff) {
        self.interrupt_1_click = interrupt_1_click;
    }
    pub fn set_interrupt_1_ia1(&mut self, interrupt_1_ia1: OnOff) {
        self.interrupt_1_ia1 = interrupt_1_ia1;
    }
    pub fn set_interrupt_1_ia2(&mut self, interrupt_1_ia2: OnOff) {
        self.interrupt_1_ia2 = interrupt_1_ia2;
    }
    pub fn set_interrupt_1_zyx_da(&mut self, interrupt_1_zyx_da: OnOff) {
        self.interrupt_1_zyx_da = interrupt_1_zyx_da;
    }
    pub fn set_interrupt_1_321_da(&mut self, interrupt_1_321_da: OnOff) {
        self.interrupt_1_321_da = interrupt_1_321_da;
    }
    pub fn set_interrupt_1_fifo_watermark(
        &mut self,
        interrupt_1_fifo_watermark: OnOff,
    ) {
        self.interrupt_1_fifo_watermark = interrupt_1_fifo_watermark;
    }
    pub fn set_interrupt_1_fifo_overrun(
        &mut self,
        interrupt_1_fifo_overrun: OnOff,
    ) {
        self.interrupt_1_fifo_overrun = interrupt_1_fifo_overrun;
    }
    pub fn interrupt_1_click(&self) -> OnOff {
        self.interrupt_1_click
    }
    pub fn interrupt_1_ia1(&self) -> OnOff {
        self.interrupt_1_ia1
    }
    pub fn interrupt_1_ia2(&self) -> OnOff {
        self.interrupt_1_ia2
    }
    pub fn interrupt_1_zyx_da(&self) -> OnOff {
        self.interrupt_1_zyx_da
    }
    pub fn interrupt_1_321_da(&self) -> OnOff {
        self.interrupt_1_321_da
    }
    pub fn interrupt_1_fifo_watermark(&self) -> OnOff {
        self.interrupt_1_fifo_watermark
    }
    pub fn interrupt_1_fifo_overrun(&self) -> OnOff {
        self.interrupt_1_fifo_overrun
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        CtrlReg3Value {
            interrupt_1_click: enabled_enum::get_state_from_bit_value(
                value >> CLICK_BIT_OFFSET,
            ),
            interrupt_1_ia1: enabled_enum::get_state_from_bit_value(
                value >> IA_1_BIT_OFFSET,
            ),
            interrupt_1_ia2: enabled_enum::get_state_from_bit_value(
                value >> IA_2_BIT_OFFSET,
            ),
            interrupt_1_zyx_da: enabled_enum::get_state_from_bit_value(
                value >> ZYX_DA_BIT_OFFSET,
            ),
            interrupt_1_321_da: enabled_enum::get_state_from_bit_value(
                value >> DA_321_BIT_OFFSET,
            ),
            interrupt_1_fifo_watermark: enabled_enum::get_state_from_bit_value(
                value >> WTM_BIT_OFFSET,
            ),
            interrupt_1_fifo_overrun: enabled_enum::get_state_from_bit_value(
                value >> OVERRUN_BIT_OFFSET,
            ),
        }
    }
    pub(super) fn get_raw_value(&self) -> u8 {
        (self.interrupt_1_click as u8) << CLICK_BIT_OFFSET
            | (self.interrupt_1_ia1 as u8) << IA_1_BIT_OFFSET
            | (self.interrupt_1_ia2 as u8) << IA_2_BIT_OFFSET
            | (self.interrupt_1_zyx_da as u8) << ZYX_DA_BIT_OFFSET
            | (self.interrupt_1_321_da as u8) << DA_321_BIT_OFFSET
            | (self.interrupt_1_fifo_watermark as u8) << WTM_BIT_OFFSET
            | (self.interrupt_1_fifo_overrun as u8) << OVERRUN_BIT_OFFSET
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value = 0b1010_0101_u8;
        let ctrl_reg_3 = super::CtrlReg3Value::from_raw_value(raw_value);
        assert_eq!(ctrl_reg_3.interrupt_1_click, super::OnOff::Enabled);
        assert_eq!(ctrl_reg_3.interrupt_1_ia1, super::OnOff::Disabled);
        assert_eq!(ctrl_reg_3.interrupt_1_ia2, super::OnOff::Enabled);
        assert_eq!(ctrl_reg_3.interrupt_1_zyx_da, super::OnOff::Disabled);
        assert_eq!(ctrl_reg_3.interrupt_1_321_da, super::OnOff::Disabled);
        assert_eq!(
            ctrl_reg_3.interrupt_1_fifo_watermark,
            super::OnOff::Enabled
        );
        assert_eq!(ctrl_reg_3.interrupt_1_fifo_overrun, super::OnOff::Disabled);
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut ctrl_reg_3_value = super::CtrlReg3Value::default();
        ctrl_reg_3_value.interrupt_1_ia1 = super::OnOff::Enabled;
        ctrl_reg_3_value.interrupt_1_ia2 = super::OnOff::Enabled;
        ctrl_reg_3_value.interrupt_1_zyx_da = super::OnOff::Enabled;
        ctrl_reg_3_value.interrupt_1_fifo_watermark = super::OnOff::Enabled;
        assert_eq!(ctrl_reg_3_value.get_raw_value(), 0b0111_0100);
    }
}
