#![no_std]
mod ctrl_reg_0_value;
mod ctrl_reg_1_value;
mod status_reg_aux_values;
mod temp_cfg_reg;

#[macro_use]
extern crate num_derive;
extern crate embedded_hal as hal;

use ctrl_reg_0_value::CtrlReg0Value;
use ctrl_reg_1_value::CtrlReg1Value;
use hal::{
    blocking::spi::{Transfer, Write},
    digital::v2::OutputPin,
};
use status_reg_aux_values::StatusRegAuxValues;
use temp_cfg_reg::TempCfgReg;

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

#[derive(Default)]
pub struct Lis3dh {
    ctrl_reg0: CtrlReg0Value,
    temp_cfg_reg: TempCfgReg,
    ctrl_reg1: CtrlReg1Value,
}

impl Lis3dh {
    pub fn set_ctrl_reg0(&mut self, ctrl_reg0: CtrlReg0Value) {
        self.ctrl_reg0 = ctrl_reg0;
    }

    pub fn set_temp_cfg_reg(&mut self, temp_cfg_reg: TempCfgReg) {
        self.temp_cfg_reg = temp_cfg_reg;
    }

    pub fn write_all_settings<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<(), Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg0 as u8 | SPI_WRITE_BIT,
                self.ctrl_reg0 as u8,
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::TempCfgReg as u8 | SPI_WRITE_BIT,
                self.temp_cfg_reg.get_raw_value(),
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg1 as u8 | SPI_WRITE_BIT,
                self.ctrl_reg1.get_raw_value(),
            ],
        )
    }

    pub fn get_temp_cfg_reg<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<TempCfgReg, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::TempCfgReg as u8,
        )?;
        Ok(TempCfgReg::from_raw_value(value))
    }

    pub fn get_ctrl_reg_0_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<CtrlReg0Value, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::CtrlReg0 as u8,
        )?;
        if value == CtrlReg0Value::PullUpDisconnectedSdoSa0Pin as u8 {
            return Ok(CtrlReg0Value::PullUpDisconnectedSdoSa0Pin);
        }
        Ok(CtrlReg0Value::PullUpDisconnectedSdoSa0Pin)
    }

    pub fn get_ctrl_reg_1_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<CtrlReg1Value, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::CtrlReg1 as u8,
        )?;
        Ok(CtrlReg1Value::from_raw_value(value))
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
        Ok(StatusRegAuxValues::from_raw_value(value))
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

    fn get_adc_value<CS, SPI, CsE, SpiE>(
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
}
