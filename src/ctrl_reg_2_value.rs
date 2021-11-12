use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
const HPM_BIT_OFFSET: u8 = 6;
#[repr(u8)]
#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
enum HighPassFilterModeSelection {
    NormalResetByReadingReference,
    ReferenceSignalForFiltering,
    Normal,
    AutoResetOnInterrupt,
}
impl Default for HighPassFilterModeSelection {
    fn default() -> Self {
        HighPassFilterModeSelection::NormalResetByReadingReference
    }
}

/// High-pass filter cutoff frequency selection
/// The bandwidth of the high-pass filter depends on the selected ODR and on the settings of
/// the HPCFx bits of CTRL_REG2. The high-pass filter cutoff frequencies (ft) are shown in Hz
///
/// | HPCF    |   1Hz | 10Hz | 25Hz | 50 Hz | 100 Hz | 200 Hz | 400 Hz | 1.6 kHz | 5 kHz |
/// |:--------|:----- |:-----|:-----|:------|:-------|:-------|:-------|:------- |:------|
/// | 00      | 0.02  | 0.2  | 0.5  |  1    |   2    |   4    |    8   |    32   | 100   |
/// | 01      | 0.008 | 0.08 | 0.2  |  0.5  |   1    |   2    |    4   |    16   | 50    |
/// | 10      | 0.004 | 0.04 | 0.1  |  0.2  |   0.5  |   1    |    2   |     8   | 25    |
/// | 11      | 0.002 | 0.02 | 0.05 |  0.1  |   0.2  |   0.5  |    1   |     4   | 12    |
/// |:--------|:----- |:-----|:-----|:------|:-------|:-------|:-------|:------- |:------|
const HPCF_BIT_OFFSET: u8 = 4;
#[repr(u8)]
#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
enum HighPassFilterCutOffFrequencySelection {
    OneFiftieth,
    AHundredthOrOneHundredAndTwentyFifth,
    ATwHundredthOrOneTwoHundredAndFiftieth,
    AFourHundredthOrAFifeHundredth,
}
impl Default for HighPassFilterCutOffFrequencySelection {
    fn default() -> Self {
        HighPassFilterCutOffFrequencySelection::OneFiftieth
    }
}

const FDS_BIT_OFFSET: u8 = 3;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum FilteredDataSelection {
    InternalFilterBypassed,
    InternalFilterSentToFifo,
}
impl Default for FilteredDataSelection {
    fn default() -> Self {
        FilteredDataSelection::InternalFilterBypassed
    }
}

const HP_CLICK_BIT_OFFSET: u8 = 2;
const HP_IA2_BIT_OFFSET: u8 = 1;
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum HighPassFilter {
    FilterBypassed,
    FilterEnabled,
}
impl Default for HighPassFilter {
    fn default() -> Self {
        HighPassFilter::FilterBypassed
    }
}

#[derive(Clone, Copy, Default)]
pub struct CtrlReg2Value {
    hp_ia1: HighPassFilter,
    hp_ia2: HighPassFilter,
    hp_click: HighPassFilter,
    fds: FilteredDataSelection,
    hpcf: HighPassFilterCutOffFrequencySelection,
    hpm: HighPassFilterModeSelection,
}

impl CtrlReg2Value {
    pub(super) fn get_raw_value(&self) -> u8 {
        (self.hpm as u8) << HPM_BIT_OFFSET
            | (self.hpcf as u8) << HPCF_BIT_OFFSET
            | (self.fds as u8) << FDS_BIT_OFFSET
            | (self.hp_click as u8) << HP_CLICK_BIT_OFFSET
            | (self.hp_ia2 as u8) << HP_IA2_BIT_OFFSET
            | self.hp_ia1 as u8
    }

    pub(super) fn from_raw_value(value: u8) -> Self {
        let hp_ia1 = if value & 1 == 1 {
            HighPassFilter::FilterEnabled
        } else {
            HighPassFilter::FilterBypassed
        };
        let hp_ia2 = if value >> HP_IA2_BIT_OFFSET & 1 == 1 {
            HighPassFilter::FilterEnabled
        } else {
            HighPassFilter::FilterBypassed
        };
        let hp_click = if value >> HP_CLICK_BIT_OFFSET & 1 == 1 {
            HighPassFilter::FilterEnabled
        } else {
            HighPassFilter::FilterBypassed
        };
        let fds = if value >> FDS_BIT_OFFSET & 1 == 1 {
            FilteredDataSelection::InternalFilterSentToFifo
        } else {
            FilteredDataSelection::InternalFilterBypassed
        };
        let hpcf = HighPassFilterCutOffFrequencySelection::from_u8(
            (value & 0b11_0000) >> HPCF_BIT_OFFSET,
        )
        .unwrap();
        let hpm = HighPassFilterModeSelection::from_u8(
            (value & 0b1100_0000) >> HPM_BIT_OFFSET,
        )
        .unwrap();
        CtrlReg2Value {
            hp_ia1,
            hp_ia2,
            hp_click,
            fds,
            hpcf,
            hpm,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value = 0b0110_0101_u8;
        let ctrl_reg_2 = super::CtrlReg2Value::from_raw_value(raw_value);
        assert_eq!(ctrl_reg_2.hp_ia1, super::HighPassFilter::FilterEnabled);
        assert_eq!(ctrl_reg_2.hp_ia2, super::HighPassFilter::FilterBypassed);
        assert_eq!(ctrl_reg_2.hp_click, super::HighPassFilter::FilterEnabled);
        assert_eq!(
            ctrl_reg_2.fds,
            super::FilteredDataSelection::InternalFilterBypassed
        );
        assert_eq!(
            ctrl_reg_2.hpcf,
            super::HighPassFilterCutOffFrequencySelection::ATwHundredthOrOneTwoHundredAndFiftieth
        );
        assert_eq!(
            ctrl_reg_2.hpm,
            super::HighPassFilterModeSelection::ReferenceSignalForFiltering
        );
    }

    #[test]
    fn conversion_to_raw_value_works() {
        let mut ctrl_reg_value = super::CtrlReg2Value::default();
        assert_eq!(ctrl_reg_value.get_raw_value(), 0_u8);

        ctrl_reg_value.hp_ia1 = super::HighPassFilter::FilterEnabled;
        ctrl_reg_value.hp_ia2 = super::HighPassFilter::FilterEnabled;
        ctrl_reg_value.hp_click = super::HighPassFilter::FilterEnabled;
        ctrl_reg_value.fds =
            super::FilteredDataSelection::InternalFilterSentToFifo;
        ctrl_reg_value.hpcf =
            super::HighPassFilterCutOffFrequencySelection::ATwHundredthOrOneTwoHundredAndFiftieth;
        ctrl_reg_value.hpm =
            super::HighPassFilterModeSelection::ReferenceSignalForFiltering;
        assert_eq!(ctrl_reg_value.get_raw_value(), 0b0110_1111);
    }
}
