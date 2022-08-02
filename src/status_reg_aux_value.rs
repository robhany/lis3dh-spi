const NEW_DATA_ON_1_AXIS_BIT_OFFSET: u8 = 0;
const NEW_DATA_ON_2_AXIS_BIT_OFFSET: u8 = 1;
const NEW_DATA_ON_3_AXIS_BIT_OFFSET: u8 = 2;
const NEW_DATA_ON_3_2_1_AXIS_BIT_OFFSET: u8 = 3;
const OVERRUN_ON_1_AXIS_BIT_OFFSET: u8 = 4;
const OVERRUN_ON_2_AXIS_BIT_OFFSET: u8 = 5;
const OVERRUN_ON_3_AXIS_BIT_OFFSET: u8 = 6;
const OVERRUN_ON_3_2_1_AXIS_BIT_OFFSET: u8 = 7;

#[derive(Default)]
pub struct StatusRegAuxValue {
    new_data_on1axis: bool,
    new_data_on2axis: bool,
    new_data_on3axis: bool,
    new_data_on3_2_1axis: bool,
    overrun_on1axis: bool,
    overrun_on2axis: bool,
    overrun_on3axis: bool,
    overrun_on3_2_1axis: bool,
    overrun_or_new_data: bool,
}

impl StatusRegAuxValue {
    pub fn has_overrun_or_new_data(&self) -> bool {
        self.overrun_or_new_data
    }
    pub fn has_new_data_on1axis(&self) -> bool {
        self.new_data_on1axis
    }
    pub fn has_new_data_on2axis(&self) -> bool {
        self.new_data_on2axis
    }
    pub fn has_new_data_on3axis(&self) -> bool {
        self.new_data_on3axis
    }
    pub fn has_new_data_on3_2_1axis(&self) -> bool {
        self.new_data_on3_2_1axis
    }
    pub fn has_overrun_on1axis(&self) -> bool {
        self.overrun_on1axis
    }
    pub fn has_overrun_on2axis(&self) -> bool {
        self.overrun_on2axis
    }
    pub fn has_overrun_on3axis(&self) -> bool {
        self.overrun_on3axis
    }
    pub fn has_overrun_on3_2_1axis(&self) -> bool {
        self.overrun_on3_2_1axis
    }
    pub(super) fn from_raw_value(value: u8) -> Self {
        StatusRegAuxValue {
            new_data_on1axis: super::check_if_bit_is_set(
                value,
                NEW_DATA_ON_1_AXIS_BIT_OFFSET,
            ),
            new_data_on2axis: super::check_if_bit_is_set(
                value,
                NEW_DATA_ON_2_AXIS_BIT_OFFSET,
            ),
            new_data_on3axis: super::check_if_bit_is_set(
                value,
                NEW_DATA_ON_3_AXIS_BIT_OFFSET,
            ),
            new_data_on3_2_1axis: super::check_if_bit_is_set(
                value,
                NEW_DATA_ON_3_2_1_AXIS_BIT_OFFSET,
            ),
            overrun_on1axis: super::check_if_bit_is_set(
                value,
                OVERRUN_ON_1_AXIS_BIT_OFFSET,
            ),
            overrun_on2axis: super::check_if_bit_is_set(
                value,
                OVERRUN_ON_2_AXIS_BIT_OFFSET,
            ),
            overrun_on3axis: super::check_if_bit_is_set(
                value,
                OVERRUN_ON_3_AXIS_BIT_OFFSET,
            ),
            overrun_on3_2_1axis: super::check_if_bit_is_set(
                value,
                OVERRUN_ON_3_2_1_AXIS_BIT_OFFSET,
            ),
            overrun_or_new_data: value > 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn conversion_from_raw_value_works() {
        let raw_value_with_2_axis_overrun_and_1_new_data_available =
            0b100001_u8;
        let status_reg_aux_values = super::StatusRegAuxValue::from_raw_value(
            raw_value_with_2_axis_overrun_and_1_new_data_available,
        );
        assert!(status_reg_aux_values.overrun_or_new_data);
        assert!(status_reg_aux_values.new_data_on1axis);
        assert!(status_reg_aux_values.overrun_on2axis);
        assert!(!status_reg_aux_values.overrun_on3_2_1axis);
    }
}
