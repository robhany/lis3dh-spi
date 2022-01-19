use crate::enabled_enum::OnOff;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SpiModeSelection {
    FourWireInterface,
    ThreeWireInterface,
}
impl Default for SpiModeSelection {
    fn default() -> Self {
        SpiModeSelection::FourWireInterface
    }
}

const SELF_TEST_BIT_OFFSET: u8 = 1;
#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
pub enum SelfTest {
    NormalMode,
    SelfTest0,
    SelfTest1,
}
impl Default for SelfTest {
    fn default() -> Self {
        SelfTest::NormalMode
    }
}

const HIGH_RESOLUTION_OUTPUT_MODE_BIT_OFFSET: u8 = 3;

const FULL_SCALE_SELECTION_OUTPUT_MODE_BIT_OFFSET: u8 = 4;
#[repr(u8)]
#[derive(Copy, Clone, Debug, FromPrimitive, PartialEq)]
pub enum FullScaleSelection {
    Gravity2G,
    Gravity4G,
    Gravity8G,
    Gravity16G,
}
impl Default for FullScaleSelection {
    fn default() -> Self {
        FullScaleSelection::Gravity2G
    }
}

const BLE_SETTING_BIT_OFFSET: u8 = 6;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Endianness {
    LSBLowerAddress,
    MSBLowerAddress,
}
impl Default for Endianness {
    fn default() -> Self {
        Endianness::LSBLowerAddress
    }
}

const BDU_SETTING_BIT_OFFSET: u8 = 7;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockDataUpdate {
    ContinuousUpdate,
    NotUpdated,
}
impl Default for BlockDataUpdate {
    fn default() -> Self {
        BlockDataUpdate::ContinuousUpdate
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct CtrlReg4Value {
    sim: SpiModeSelection,
    st: SelfTest,
    hr: OnOff,
    fs: FullScaleSelection,
    ble: Endianness,
    bdu: BlockDataUpdate,
}

impl CtrlReg4Value {
    pub(super) fn set_sim(&mut self, sim: SpiModeSelection) {
        self.sim = sim;
    }

    pub(super) fn set_st(&mut self, st: SelfTest) {
        self.st = st;
    }

    pub(super) fn set_hr(&mut self, hr: OnOff) {
        self.hr = hr;
    }

    pub(super) fn set_fs(&mut self, fs: FullScaleSelection) {
        self.fs = fs;
    }

    pub(super) fn set_ble(&mut self, ble: Endianness) {
        self.ble = ble;
    }

    pub(super) fn set_bdu(&mut self, bdu: BlockDataUpdate) {
        self.bdu = bdu;
    }

    pub(super) fn sim(&self) -> SpiModeSelection {
        self.sim
    }
    pub(super) fn st(&self) -> SelfTest {
        self.st
    }
    pub(super) fn hr(&self) -> OnOff {
        self.hr
    }
    pub(super) fn fs(&self) -> FullScaleSelection {
        self.fs
    }
    pub(super) fn ble(&self) -> Endianness {
        self.ble
    }
    pub(super) fn bdu(&self) -> BlockDataUpdate {
        self.bdu
    }

    pub(super) fn get_raw_value(&self) -> u8 {
        (self.bdu as u8) << BDU_SETTING_BIT_OFFSET
            | (self.ble as u8) << BLE_SETTING_BIT_OFFSET
            | (self.fs as u8) << FULL_SCALE_SELECTION_OUTPUT_MODE_BIT_OFFSET
            | (self.hr as u8) << HIGH_RESOLUTION_OUTPUT_MODE_BIT_OFFSET
            | (self.st as u8) << SELF_TEST_BIT_OFFSET
            | self.sim as u8
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        let sim = if value & 1 == 1 {
            SpiModeSelection::ThreeWireInterface
        } else {
            SpiModeSelection::FourWireInterface
        };
        let st =
            SelfTest::from_u8((value & 0b110) >> SELF_TEST_BIT_OFFSET).unwrap();
        let hr = if (value >> HIGH_RESOLUTION_OUTPUT_MODE_BIT_OFFSET) & 1 == 1 {
            OnOff::Enabled
        } else {
            OnOff::Disabled
        };
        let fs = FullScaleSelection::from_u8(
            (value & 0b0011_0000)
                >> FULL_SCALE_SELECTION_OUTPUT_MODE_BIT_OFFSET,
        )
        .unwrap();
        let ble = if value >> BLE_SETTING_BIT_OFFSET & 1 == 1 {
            Endianness::MSBLowerAddress
        } else {
            Endianness::LSBLowerAddress
        };
        let bdu = if value >> BDU_SETTING_BIT_OFFSET & 1 == 1 {
            BlockDataUpdate::NotUpdated
        } else {
            BlockDataUpdate::ContinuousUpdate
        };
        CtrlReg4Value {
            sim,
            st,
            hr,
            fs,
            ble,
            bdu,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value = 0b0101_0100_u8;
        let ctrl_reg_4 = super::CtrlReg4Value::from_raw_value(raw_value);
        assert_eq!(ctrl_reg_4.st, super::SelfTest::SelfTest1)
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut ctrl_reg_value = super::CtrlReg4Value::default();

        ctrl_reg_value.sim = super::SpiModeSelection::ThreeWireInterface;
        ctrl_reg_value.st = super::SelfTest::SelfTest1;
        ctrl_reg_value.hr = super::OnOff::Enabled;
        ctrl_reg_value.fs = super::FullScaleSelection::Gravity8G;
        ctrl_reg_value.ble = super::Endianness::MSBLowerAddress;
        assert_eq!(ctrl_reg_value.get_raw_value(), 0b0110_1101);
    }
}
