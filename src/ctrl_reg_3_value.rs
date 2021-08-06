use crate::enabled_enum::OnOff;

const OVERRUN_BIT_OFFSET: u8 = 1;
const WTM_BIT_OFFSET: u8 = 2;
const DA_321_BIT_OFFSET: u8 = 3;
const ZYX_DA_BIT_OFFSET: u8 = 4;
const IA_2_BIT_OFFSET: u8 = 5;
const IA_1_BIT_OFFSET: u8 = 6;
const CLICK_BIT_OFFSET: u8 = 7;
#[derive(Default)]
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
    pub(super) fn from_raw_value(value: u8) -> Self {
        CtrlReg3Value {
            interrupt_1_click: CtrlReg3Value::get_interrupt_from_bit_value(
                value >> CLICK_BIT_OFFSET,
            ),
            interrupt_1_ia1: CtrlReg3Value::get_interrupt_from_bit_value(
                value >> IA_1_BIT_OFFSET,
            ),
            interrupt_1_ia2: CtrlReg3Value::get_interrupt_from_bit_value(
                value >> IA_2_BIT_OFFSET,
            ),
            interrupt_1_zyx_da: CtrlReg3Value::get_interrupt_from_bit_value(
                value >> ZYX_DA_BIT_OFFSET,
            ),
            interrupt_1_321_da: CtrlReg3Value::get_interrupt_from_bit_value(
                value >> DA_321_BIT_OFFSET,
            ),
            interrupt_1_fifo_watermark:
                CtrlReg3Value::get_interrupt_from_bit_value(
                    value >> WTM_BIT_OFFSET,
                ),
            interrupt_1_fifo_overrun:
                CtrlReg3Value::get_interrupt_from_bit_value(
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

    fn get_interrupt_from_bit_value(value: u8) -> OnOff {
        if value & 1 == 1 {
            OnOff::Enabled
        } else {
            OnOff::Disabled
        }
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
