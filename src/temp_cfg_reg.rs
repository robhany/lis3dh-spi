const TEMP_EN_BIT_OFFSET: u8 = 6;
#[derive(PartialEq)]
#[repr(u8)]
pub enum TempEn {
    TemperatureDisabled,
    TemperatureEnabled,
}
impl Default for TempEn {
    fn default() -> Self {
        TempEn::TemperatureDisabled
    }
}

const ADC_EN_BIT_OFFSET: u8 = 7;
#[derive(PartialEq)]
#[repr(u8)]
pub enum AdcEn {
    AdcDisabled,
    AdcEnabled,
}
impl Default for AdcEn {
    fn default() -> Self {
        AdcEn::AdcDisabled
    }
}

#[derive(Default)]
pub struct TempCfgReg {
    temp: TempEn,
    adc: AdcEn,
}

impl TempCfgReg {
    pub fn from_raw_value(value: u8) -> Self {
        let temp = if value >> TEMP_EN_BIT_OFFSET & 1 == 1 {
            TempEn::TemperatureEnabled
        } else {
            TempEn::TemperatureDisabled
        };
        let adc = if value >> ADC_EN_BIT_OFFSET & 1 == 1 {
            AdcEn::AdcEnabled
        } else {
            AdcEn::AdcDisabled
        };
        TempCfgReg { temp, adc }
    }

    pub(super) fn get_raw_value(&self) -> u8 {
        let mut result = 0_u8;
        if self.adc == AdcEn::AdcEnabled {
            result += 1 << ADC_EN_BIT_OFFSET;
        }
        if self.temp == TempEn::TemperatureEnabled {
            result += TEMP_EN_BIT_OFFSET << 1;
        }
        result
    }
}
