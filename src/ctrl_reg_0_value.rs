use crate::enabled_enum;
use crate::enabled_enum::OnOff;
use crate::enabled_enum::OnOff::Enabled;

const CTRL_REG_0_DEFAULT_VALUE_LOWER_7_BIT: u8 = 0b001_0000;
const PULL_UP_CONFIG_BIT_OFFSET: u8 = 7;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct CtrlReg0Value {
    pull_up_connected_sdo_sa_0_pin: OnOff,
}

impl CtrlReg0Value {
    pub fn set_pull_up_connected_sdo_sa_0_pin(
        &mut self,
        pull_up_connected_sdo_sa_0_pin: OnOff,
    ) {
        self.pull_up_connected_sdo_sa_0_pin = pull_up_connected_sdo_sa_0_pin;
    }
    pub fn pull_up_connected_sdo_sa_0_pin(&self) -> OnOff {
        self.pull_up_connected_sdo_sa_0_pin
    }
    pub(super) fn get_raw_value(&self) -> u8 {
        let value = if self.pull_up_connected_sdo_sa_0_pin == Enabled {
            1
        } else {
            0
        };
        (value << PULL_UP_CONFIG_BIT_OFFSET)
            | CTRL_REG_0_DEFAULT_VALUE_LOWER_7_BIT
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        CtrlReg0Value {
            pull_up_connected_sdo_sa_0_pin:
                enabled_enum::get_state_from_bit_value(
                    value >> PULL_UP_CONFIG_BIT_OFFSET,
                ),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn conversion_from_raw_value_works() {
        let ctrl_reg_0_raw = 0b1001_0000;
        let ctrl_reg_0 = super::CtrlReg0Value::from_raw_value(ctrl_reg_0_raw);
        assert_eq!(
            ctrl_reg_0.pull_up_connected_sdo_sa_0_pin(),
            super::OnOff::Enabled
        );
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut ctrl_reg_0 = super::CtrlReg0Value::default();
        ctrl_reg_0.set_pull_up_connected_sdo_sa_0_pin(super::OnOff::Enabled);
        assert_eq!(ctrl_reg_0.get_raw_value(), 0b1001_0000);
    }
}
