use crate::enabled_enum::OnOff;

const TEMP_EN_BIT_OFFSET: u8 = 6;
const ADC_EN_BIT_OFFSET: u8 = 7;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct TempCfgRegValue {
    temp: OnOff,
    adc: OnOff,
}

impl TempCfgRegValue {
    pub fn set_temp(&mut self, temp: OnOff) {
        self.temp = temp;
    }
    pub fn set_adc(&mut self, adc: OnOff) {
        self.adc = adc;
    }
    pub fn temp(&self) -> OnOff {
        self.temp
    }
    pub fn adc(&self) -> OnOff {
        self.adc
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        let temp = if value >> TEMP_EN_BIT_OFFSET & 1 == 1 {
            OnOff::Enabled
        } else {
            OnOff::Disabled
        };
        let adc = if value >> ADC_EN_BIT_OFFSET & 1 == 1 {
            OnOff::Enabled
        } else {
            OnOff::Disabled
        };
        TempCfgRegValue { temp, adc }
    }
    pub(super) fn get_raw_value(&self) -> u8 {
        let mut result = 0_u8;
        if self.adc == OnOff::Enabled {
            result += 1 << ADC_EN_BIT_OFFSET;
        }
        if self.temp == OnOff::Enabled {
            result += TEMP_EN_BIT_OFFSET << 1;
        }
        result
    }
}
