use crate::enabled_enum;
use crate::enabled_enum::OnOff;

const XLIE_BIT_OFFSET: u8 = 0;
const XHIE_BIT_OFFSET: u8 = 1;
const YLIE_BIT_OFFSET: u8 = 2;
const YHIE_BIT_OFFSET: u8 = 3;
const ZLIE_BIT_OFFSET: u8 = 4;
const ZHIE_BIT_OFFSET: u8 = 5;
const SIXD_BIT_OFFSET: u8 = 6;
const AOI_BIT_OFFSET: u8 = 7;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct IntCfg {
    aoi: OnOff,
    six_d: OnOff,
    zhie: OnOff,
    zlie: OnOff,
    yhie: OnOff,
    ylie: OnOff,
    xhie: OnOff,
    xlie: OnOff,
}

impl IntCfg {
    pub fn set_aoi(&mut self, aoi: OnOff) {
        self.aoi = aoi;
    }
    pub fn set_six_d(&mut self, six_d: OnOff) {
        self.six_d = six_d;
    }
    pub fn set_zhie(&mut self, zhie: OnOff) {
        self.zhie = zhie;
    }
    pub fn set_zlie(&mut self, zlie: OnOff) {
        self.zlie = zlie;
    }
    pub fn set_yhie(&mut self, yhie: OnOff) {
        self.yhie = yhie;
    }
    pub fn set_ylie(&mut self, ylie: OnOff) {
        self.ylie = ylie;
    }
    pub fn set_xhie(&mut self, xhie: OnOff) {
        self.xhie = xhie;
    }
    pub fn set_xlie(&mut self, xlie: OnOff) {
        self.xlie = xlie;
    }
    pub fn aoi(&self) -> OnOff {
        self.aoi
    }
    pub fn six_d(&self) -> OnOff {
        self.six_d
    }
    pub fn zhie(&self) -> OnOff {
        self.zhie
    }
    pub fn zlie(&self) -> OnOff {
        self.zlie
    }
    pub fn yhie(&self) -> OnOff {
        self.yhie
    }
    pub fn ylie(&self) -> OnOff {
        self.ylie
    }
    pub fn xhie(&self) -> OnOff {
        self.xhie
    }
    pub fn xlie(&self) -> OnOff {
        self.xlie
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        IntCfg {
            aoi: enabled_enum::get_state_from_bit_value(
                value >> AOI_BIT_OFFSET,
            ),
            six_d: enabled_enum::get_state_from_bit_value(
                value >> SIXD_BIT_OFFSET,
            ),
            zhie: enabled_enum::get_state_from_bit_value(
                value >> ZHIE_BIT_OFFSET,
            ),
            zlie: enabled_enum::get_state_from_bit_value(
                value >> ZLIE_BIT_OFFSET,
            ),
            yhie: enabled_enum::get_state_from_bit_value(
                value >> YHIE_BIT_OFFSET,
            ),
            ylie: enabled_enum::get_state_from_bit_value(
                value >> YLIE_BIT_OFFSET,
            ),
            xhie: enabled_enum::get_state_from_bit_value(
                value >> XHIE_BIT_OFFSET,
            ),
            xlie: enabled_enum::get_state_from_bit_value(
                value >> XLIE_BIT_OFFSET,
            ),
        }
    }
    pub(super) fn get_raw_value(&self) -> u8 {
        (self.aoi as u8) << AOI_BIT_OFFSET
            | (self.six_d as u8) << SIXD_BIT_OFFSET
            | (self.zhie as u8) << ZHIE_BIT_OFFSET
            | (self.zlie as u8) << ZLIE_BIT_OFFSET
            | (self.yhie as u8) << YHIE_BIT_OFFSET
            | (self.ylie as u8) << YLIE_BIT_OFFSET
            | (self.xhie as u8) << XHIE_BIT_OFFSET
            | (self.xlie as u8) << XLIE_BIT_OFFSET
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn conversion_from_raw_value_works() {
        let int_cfg_raw = 0b1010_0101_u8;
        let int_cfg = super::IntCfg::from_raw_value(int_cfg_raw);
        assert_eq!(int_cfg.aoi, super::OnOff::Enabled);
        assert_eq!(int_cfg.six_d, super::OnOff::Disabled);
        assert_eq!(int_cfg.zhie, super::OnOff::Enabled);
        assert_eq!(int_cfg.zlie, super::OnOff::Disabled);
        assert_eq!(int_cfg.yhie, super::OnOff::Disabled);
        assert_eq!(int_cfg.ylie, super::OnOff::Enabled);
        assert_eq!(int_cfg.xhie, super::OnOff::Disabled);
        assert_eq!(int_cfg.xlie, super::OnOff::Enabled);
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut int_cfg = super::IntCfg::default();
        int_cfg.six_d = super::OnOff::Enabled;
        int_cfg.zhie = super::OnOff::Enabled;
        int_cfg.zlie = super::OnOff::Enabled;
        int_cfg.ylie = super::OnOff::Enabled;
        assert_eq!(int_cfg.get_raw_value(), 0b0111_0100);
    }
}
