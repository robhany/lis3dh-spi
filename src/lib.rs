#![no_std]
mod ctrl_reg_0_value;
mod ctrl_reg_1_value;
mod ctrl_reg_2_value;
mod ctrl_reg_3_value;
mod status_reg_aux_value;
mod temp_cfg_reg_value;

#[macro_use]
extern crate num_derive;
extern crate embedded_hal as hal;

use crate::ctrl_reg_3_value::CtrlReg3Value;
use ctrl_reg_0_value::CtrlReg0Value;
use ctrl_reg_1_value::CtrlReg1Value;
use ctrl_reg_2_value::CtrlReg2Value;
use hal::{
    blocking::spi::{Transfer, Write},
    digital::v2::OutputPin,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use status_reg_aux_value::StatusRegAuxValue;
use temp_cfg_reg_value::TempCfgRegValue;

const SPI_WRITE_BIT: u8 = 0x40;

#[derive(Debug)]
pub enum Error<CsE, SpiE> {
    ChipSelectError(CsE),
    SpiError(SpiE),
}

#[repr(u8)]
#[derive(FromPrimitive, PartialOrd, PartialEq)]
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

fn is_read_only(mut address: u8) -> bool {
    address &= !SPI_WRITE_BIT;
    let register = RegisterAddresses::from_u8(address).unwrap();
    (RegisterAddresses::StatusRegAux..RegisterAddresses::WhoAmI)
        .contains(&register)
        || (RegisterAddresses::StatusReg..RegisterAddresses::OutZH)
            .contains(&register)
        || RegisterAddresses::FifSrcReg.eq(&register)
        || RegisterAddresses::Int1Src.eq(&register)
        || RegisterAddresses::Int2Src.eq(&register)
        || RegisterAddresses::ClickSrc.eq(&register)
}

fn check_if_bit_is_set(value: u8, bit_position: u8) -> bool {
    ((value >> bit_position) & 0b1).eq(&0b1)
}

#[derive(Default)]
pub struct Lis3dh {
    ctrl_reg0: CtrlReg0Value,
    temp_cfg_reg: TempCfgRegValue,
    ctrl_reg1: CtrlReg1Value,
    ctrl_reg2: CtrlReg2Value,
    ctrl_reg3: CtrlReg3Value,
}

impl Lis3dh {
    pub fn set_ctrl_reg0(&mut self, value: CtrlReg0Value) {
        self.ctrl_reg0 = value;
    }

    pub fn set_temp_cfg_reg(&mut self, value: TempCfgRegValue) {
        self.temp_cfg_reg = value;
    }

    pub fn set_ctrl_reg1(&mut self, value: CtrlReg1Value) {
        self.ctrl_reg1 = value;
    }

    pub fn set_ctrl_reg2(&mut self, value: CtrlReg2Value) {
        self.ctrl_reg2 = value;
    }

    pub fn set_ctrl_reg3(&mut self, value: CtrlReg3Value) {
        self.ctrl_reg3 = value;
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
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg2 as u8 | SPI_WRITE_BIT,
                self.ctrl_reg2.get_raw_value(),
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg3 as u8 | SPI_WRITE_BIT,
                self.ctrl_reg3.get_raw_value(),
            ],
        )
    }

    pub fn get_ctrl_reg_3_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<CtrlReg3Value, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::CtrlReg3 as u8,
        )?;
        Ok(CtrlReg3Value::from_raw_value(value))
    }

    pub fn get_ctrl_reg_2_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<CtrlReg2Value, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::CtrlReg2 as u8,
        )?;
        Ok(CtrlReg2Value::from_raw_value(value))
    }

    pub fn get_temp_cfg_reg<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<TempCfgRegValue, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::TempCfgReg as u8,
        )?;
        Ok(TempCfgRegValue::from_raw_value(value))
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
    ) -> Result<StatusRegAuxValue, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::StatusRegAux as u8,
        )?;
        Ok(StatusRegAuxValue::from_raw_value(value))
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
        if is_read_only(*data.first().unwrap()) {
            panic!("Attempt to write to a read only register");
        }
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
        assert!(super::is_read_only(
            super::RegisterAddresses::FifSrcReg as u8
        ));
        assert!(!super::is_read_only(
            super::RegisterAddresses::CtrlReg1 as u8
        ));
    }

    #[test]
    fn checking_if_a_register_is_read_only_works_with_set_write_bit() {
        assert!(super::is_read_only(
            super::RegisterAddresses::FifSrcReg as u8 | super::SPI_WRITE_BIT
        ));
        assert!(!super::is_read_only(
            super::RegisterAddresses::CtrlReg1 as u8 | super::SPI_WRITE_BIT
        ));
    }
}
