use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum XEn {
    XAxisDisabled,
    XAxisEnabled,
}

impl Default for XEn {
    fn default() -> Self {
        XEn::XAxisEnabled
    }
}

const Y_EN_BIT_OFFSET: u8 = 1;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum YEn {
    YAxisDisabled,
    YAxisEnabled,
}

impl Default for YEn {
    fn default() -> Self {
        YEn::YAxisEnabled
    }
}

const Z_EN_BIT_OFFSET: u8 = 2;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ZEn {
    ZAxisDisabled,
    ZAxisEnabled,
}

impl Default for ZEn {
    fn default() -> Self {
        ZEn::ZAxisEnabled
    }
}

const L_P_EN_BIT_OFFSET: u8 = 3;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LPEn {
    LowPowerEnabled,
    HighResolutionNormalMode,
}

impl Default for LPEn {
    fn default() -> Self {
        LPEn::LowPowerEnabled
    }
}

const DATA_RATE_SELECTION_BIT_OFFSET: u8 = 4;
#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq)]
pub enum ODR {
    PowerDownMode = 0_u8,
    Hz1,
    Hz10,
    Hz25,
    Hz50,
    Hz100,
    Hz200,
    Hz400,
    LowPowerMode1Point6kHz,
    HrNormal1Pont344kHzLowPower5Point376kHz,
}

impl Default for ODR {
    fn default() -> Self {
        ODR::PowerDownMode
    }
}

#[derive(Clone, Copy, Default)]
pub struct CtrlReg1Value {
    x_en: XEn,
    y_en: YEn,
    z_en: ZEn,
    l_p_en: LPEn,
    output_data_rate: ODR,
}

impl CtrlReg1Value {
    pub(super) fn set_output_data_rate(&mut self, output_data_rate: ODR) {
        self.output_data_rate = output_data_rate;
    }

    pub(super) fn set_l_p_en(&mut self, l_p_en: LPEn) {
        self.l_p_en = l_p_en;
    }
}

impl CtrlReg1Value {
    pub fn x_en(&self) -> XEn {
        self.x_en
    }
    pub fn y_en(&self) -> YEn {
        self.y_en
    }
    pub fn z_en(&self) -> ZEn {
        self.z_en
    }
    pub fn l_p_en(&self) -> LPEn {
        self.l_p_en
    }
    pub fn output_data_rate(&self) -> ODR {
        self.output_data_rate
    }

    pub(super) fn get_raw_value(&self) -> u8 {
        (self.output_data_rate as u8) << DATA_RATE_SELECTION_BIT_OFFSET
            | (self.l_p_en as u8) << L_P_EN_BIT_OFFSET
            | (self.z_en as u8) << Z_EN_BIT_OFFSET
            | (self.y_en as u8) << Y_EN_BIT_OFFSET
            | self.x_en as u8
    }

    pub(super) fn from_raw_value(value: u8) -> Self {
        let x_en = if value & 1 == 1 {
            XEn::XAxisEnabled
        } else {
            XEn::XAxisDisabled
        };
        let y_en = if value >> Y_EN_BIT_OFFSET & 1 == 1 {
            YEn::YAxisEnabled
        } else {
            YEn::YAxisDisabled
        };
        let z_en = if value >> Z_EN_BIT_OFFSET & 1 == 1 {
            ZEn::ZAxisEnabled
        } else {
            ZEn::ZAxisDisabled
        };
        let l_p_en = if value >> L_P_EN_BIT_OFFSET & 1 == 1 {
            LPEn::LowPowerEnabled
        } else {
            LPEn::HighResolutionNormalMode
        };
        let output_data_rate =
            ODR::from_u8(value >> DATA_RATE_SELECTION_BIT_OFFSET).unwrap();

        CtrlReg1Value {
            x_en,
            y_en,
            z_en,
            l_p_en,
            output_data_rate,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value = 0b10_0101_u8;
        let ctrl_reg_1 = super::CtrlReg1Value::from_raw_value(raw_value);
        assert_eq!(ctrl_reg_1.x_en, super::XEn::XAxisEnabled);
        assert_eq!(ctrl_reg_1.y_en, super::YEn::YAxisDisabled);
        assert_eq!(ctrl_reg_1.z_en, super::ZEn::ZAxisEnabled);
        assert_eq!(ctrl_reg_1.l_p_en, super::LPEn::HighResolutionNormalMode);
        assert_eq!(ctrl_reg_1.output_data_rate, super::ODR::Hz10);
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut ctrl_reg_value = super::CtrlReg1Value::default();
        ctrl_reg_value.output_data_rate = super::ODR::Hz50;
        assert_eq!(ctrl_reg_value.get_raw_value(), 0b100_0111);
    }
}
