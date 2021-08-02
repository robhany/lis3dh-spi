#![no_std]
extern crate embedded_hal as hal;
use hal::{
    blocking::spi::{Transfer, Write},
    digital::v2::OutputPin,
};

const SPI_WRITE_BIT: u8 = 0x40;

#[derive(Debug)]
pub enum Error<CsE, SpiE> {
    ChipSelectError(CsE),
    SpiError(SpiE),
}

#[repr(u8)]
#[derive(PartialOrd, PartialEq)]
enum RegisterAddresses {
    StatusRegAux = 0x07,
    OutAdc1L,
    OutAdc1H,
    OutAdc2L,
    OutAdc2H,
    OutAdc3L,
    OutAdc3H,
    WhoAmI = 0xF,
    CtrlReg0 = 0x1E,
    TempCfgReg,
    CtrlReg1,
    CtrlReg2,
    CtrlReg3,
    CtrlReg4,
    CtrlReg5,
    CtrlReg6,
    Reference,
    StatusReg,
    OutXL,
    OutXH,
    OutYL,
    OutYH,
    OutZL,
    OutZH,
    FifoCtrlReg,
    FifSrcReg,
    Int1Cfg,
    Int1Src,
    Int1Threshold,
    Int1Duration,
    Int2Cfg,
    Int2Src,
    Int2Threshold,
    Int2Duration,
    ClickCfg,
    ClickSrc,
    ClickThreshold,
    TimeLimit,
    TimeLatency,
    TimeWindow,
    ActivationThreshold,
    ActivationDuration,
}

fn is_read_only(address: RegisterAddresses) -> bool {
    (RegisterAddresses::StatusRegAux..RegisterAddresses::WhoAmI)
        .contains(&address)
        || (RegisterAddresses::StatusReg..RegisterAddresses::OutZH)
            .contains(&address)
        || RegisterAddresses::FifSrcReg.eq(&address)
        || RegisterAddresses::Int1Src.eq(&address)
        || RegisterAddresses::Int2Src.eq(&address)
        || RegisterAddresses::ClickSrc.eq(&address)
}

fn check_if_bit_is_set(value: u8, bit_position: u8) -> bool {
    ((value >> bit_position) & 0b1).eq(&0b1)
}

#[repr(u8)]
pub enum CtrReg0Value {
    PullUpConnectedSdoSa0Pin,
    PullUpDisconnectedSdoSa0Pin = 16,
}

impl Default for CtrReg0Value {
    fn default() -> Self {
        CtrReg0Value::PullUpDisconnectedSdoSa0Pin
    }
}

enum StatusRegAuxDataBitOffset {
    NewDataOn1Axis,
    NewDataOn2Axis,
    NewDataOn3Axis,
    NewDataOn3_2_1Axis,
    OverrunOn1Axis,
    OverrunOn2Axis,
    OverrunOn3Axis,
    OverrunOn3_2_1Axis,
}

#[derive(Default)]
pub struct StatusRegAuxValues {
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

impl StatusRegAuxValues {
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
    fn update_values_with_spi_result(&mut self, value: u8) {
        self.new_data_on1axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::NewDataOn1Axis as u8,
        );
        self.new_data_on2axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::NewDataOn2Axis as u8,
        );
        self.new_data_on3axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::NewDataOn3Axis as u8,
        );
        self.new_data_on3_2_1axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::NewDataOn3_2_1Axis as u8,
        );
        self.overrun_on1axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::OverrunOn1Axis as u8,
        );
        self.overrun_on2axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::OverrunOn2Axis as u8,
        );
        self.overrun_on3axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::OverrunOn3Axis as u8,
        );
        self.overrun_on3_2_1axis = check_if_bit_is_set(
            value,
            StatusRegAuxDataBitOffset::OverrunOn3_2_1Axis as u8,
        );
        self.overrun_or_new_data = value > 0;
    }
}

#[derive(Default)]
pub struct Lis3dh {
    ctrl_reg0: CtrReg0Value,
}

impl Lis3dh {
    pub fn get_ctrl_reg_0_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<CtrReg0Value, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::CtrlReg0 as u8,
        )?;
        if value == CtrReg0Value::PullUpDisconnectedSdoSa0Pin as u8 {
            return Ok(CtrReg0Value::PullUpDisconnectedSdoSa0Pin);
        }
        Ok(CtrReg0Value::PullUpDisconnectedSdoSa0Pin)
    }

    pub fn get_status_reg_aux_values<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<StatusRegAuxValues, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::StatusRegAux as u8,
        )?;
        let mut status_reg_aux_values = StatusRegAuxValues::default();
        status_reg_aux_values.update_values_with_spi_result(value);
        Ok(status_reg_aux_values)
    }

    pub fn get_adc1_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<u16, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        self.get_adc_value(
            cs,
            spi,
            RegisterAddresses::OutAdc1L as u8,
            RegisterAddresses::OutAdc1H as u8,
        )
    }

    pub fn get_adc2_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<u16, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        self.get_adc_value(
            cs,
            spi,
            RegisterAddresses::OutAdc2L as u8,
            RegisterAddresses::OutAdc2H as u8,
        )
    }
    pub fn get_adc3_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<u16, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        self.get_adc_value(
            cs,
            spi,
            RegisterAddresses::OutAdc3L as u8,
            RegisterAddresses::OutAdc3H as u8,
        )
    }

    pub fn get_adc_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
        low_byte_address: u8,
        high_byte_address: u8,
    ) -> Result<u16, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let low_byte =
            self.read_single_byte_from_spi(cs, spi, low_byte_address)?;
        let high_byte =
            self.read_single_byte_from_spi(cs, spi, high_byte_address)?;

        Ok(((high_byte as u16) << 8) | low_byte as u16)
    }

    pub fn get_who_am_i<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<u8, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        self.read_single_byte_from_spi(cs, spi, RegisterAddresses::WhoAmI as u8)
    }

    fn read_single_byte_from_spi<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
        address_to_read: u8,
    ) -> Result<u8, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let mut read_buffer = [address_to_read, 0xff];
        cs.set_low().map_err(Error::ChipSelectError)?;
        spi.transfer(&mut read_buffer).map_err(Error::SpiError)?;
        cs.set_high().map_err(Error::ChipSelectError)?;
        Ok(read_buffer[1])
    }

    fn write_to_spi<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
        data: [u8; 2],
    ) -> Result<(), Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        cs.set_low().map_err(Error::ChipSelectError)?;
        spi.write(&data).map_err(Error::SpiError)?;
        cs.set_high().map_err(Error::ChipSelectError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    #[test]
    fn register_address_enum_has_expected_u8_representation() {
        assert_eq!(super::RegisterAddresses::ActivationDuration as u8, 0x3F);
    }

    #[test]
    fn checking_if_a_register_is_read_only_works() {
        assert!(super::is_read_only(super::RegisterAddresses::FifSrcReg));
    }

    #[test]
    fn conversion_from_raw_value_to_status_reg_aux_values_works() {
        let mut status_reg_aux_values = super::StatusRegAuxValues::default();
        let raw_value_with_2_axis_overrun_and_1_new_data_available =
            0b100001_u8;
        status_reg_aux_values.update_values_with_spi_result(
            raw_value_with_2_axis_overrun_and_1_new_data_available,
        );
        assert!(status_reg_aux_values.overrun_or_new_data);
        assert!(status_reg_aux_values.new_data_on1axis);
        assert!(status_reg_aux_values.overrun_on2axis);
        assert!(!status_reg_aux_values.overrun_on3_2_1axis);
    }
}
