#![no_std]
pub mod ctrl_reg_0_value;
pub mod ctrl_reg_1_value;
pub mod ctrl_reg_2_value;
pub mod ctrl_reg_3_value;
pub mod ctrl_reg_4_value;
pub mod enabled_enum;
mod mode;
mod status_reg_aux_value;
mod temp_cfg_reg_value;

#[macro_use]
extern crate num_derive;
extern crate embedded_hal as hal;
use core::fmt::Debug;
use ctrl_reg_0_value::CtrlReg0Value;
use ctrl_reg_1_value::CtrlReg1Value;
use ctrl_reg_2_value::CtrlReg2Value;
use ctrl_reg_3_value::CtrlReg3Value;
use ctrl_reg_4_value::CtrlReg4Value;
use hal::{
    blocking::spi::{Transfer, Write},
    digital::v2::OutputPin,
};
use micromath::vector::{I16x3, I32x3};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use status_reg_aux_value::StatusRegAuxValue;
use temp_cfg_reg_value::TempCfgRegValue;

pub const SPI_READ_BIT: u8 = 0x80;

#[derive(Debug)]
pub enum Error<CsE, SpiE> {
    ChipSelectError(CsE),
    SpiError(SpiE),
}

#[repr(u8)]
#[derive(FromPrimitive, PartialOrd, PartialEq)]
pub enum RegisterAddresses {
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

fn is_read_only(address: u8) -> bool {
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
    ctrl_reg4: CtrlReg4Value,
}

impl Lis3dh {
    pub fn set_output_data_rate(
        &mut self,
        output_data_rate: ctrl_reg_1_value::ODR,
    ) {
        self.ctrl_reg1.set_output_data_rate(output_data_rate);
    }

    pub fn set_l_p_en(&mut self, l_p_en: ctrl_reg_1_value::LPEn) {
        self.ctrl_reg1.set_l_p_en(l_p_en);
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
            [RegisterAddresses::CtrlReg0 as u8, self.ctrl_reg0 as u8],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::TempCfgReg as u8,
                self.temp_cfg_reg.get_raw_value(),
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg1 as u8,
                self.ctrl_reg1.get_raw_value(),
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg2 as u8,
                self.ctrl_reg2.get_raw_value(),
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg3 as u8,
                self.ctrl_reg3.get_raw_value(),
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::CtrlReg4 as u8,
                self.ctrl_reg4.get_raw_value(),
            ],
        )?;
        self.write_to_spi(
            cs,
            spi,
            [
                RegisterAddresses::TempCfgReg as u8,
                self.temp_cfg_reg.get_raw_value(),
            ],
        )
    }

    pub fn get_ctrl_reg_4_value<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<CtrlReg4Value, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let value = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::CtrlReg4 as u8,
        )?;
        Ok(CtrlReg4Value::from_raw_value(value))
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

    pub fn get_accel_norm<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<I32x3, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let mode = self.get_mode(cs, spi)?;
        let range = self.get_ctrl_reg_4_value(cs, spi)?.fs();

        let multiplier = match (mode, range) {
            (
                mode::Mode::HighResolution,
                ctrl_reg_4_value::FullScaleSelection::Gravity2G,
            ) => 1,
            (
                mode::Mode::HighResolution,
                ctrl_reg_4_value::FullScaleSelection::Gravity4G,
            ) => 1,
            (
                mode::Mode::HighResolution,
                ctrl_reg_4_value::FullScaleSelection::Gravity8G,
            ) => 4,
            (
                mode::Mode::HighResolution,
                ctrl_reg_4_value::FullScaleSelection::Gravity16G,
            ) => 12,
            (
                mode::Mode::Normal,
                ctrl_reg_4_value::FullScaleSelection::Gravity2G,
            ) => 4,
            (
                mode::Mode::Normal,
                ctrl_reg_4_value::FullScaleSelection::Gravity4G,
            ) => 8,
            (
                mode::Mode::Normal,
                ctrl_reg_4_value::FullScaleSelection::Gravity8G,
            ) => 16,
            (
                mode::Mode::Normal,
                ctrl_reg_4_value::FullScaleSelection::Gravity16G,
            ) => 48,
            (
                mode::Mode::LowPower,
                ctrl_reg_4_value::FullScaleSelection::Gravity2G,
            ) => 16,
            (
                mode::Mode::LowPower,
                ctrl_reg_4_value::FullScaleSelection::Gravity4G,
            ) => 32,
            (
                mode::Mode::LowPower,
                ctrl_reg_4_value::FullScaleSelection::Gravity8G,
            ) => 64,
            (
                mode::Mode::LowPower,
                ctrl_reg_4_value::FullScaleSelection::Gravity16G,
            ) => 192,
        };

        let shift: u8 = match mode {
            mode::Mode::HighResolution => 4, // High Resolution:  12-bit
            mode::Mode::Normal => 6,         // Normal:           10-bit
            mode::Mode::LowPower => 8,       // Low Power:         8-bit
        };

        let acc_raw = self.get_accel_raw(cs, spi)?;
        let x = (acc_raw.x >> shift) as i32 * multiplier;
        let y = (acc_raw.y >> shift) as i32 * multiplier;
        let z = (acc_raw.z >> shift) as i32 * multiplier;

        Ok(I32x3 { x, y, z })
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

    pub fn get_register_raw_value<CS, SPI, CsE, SpiE>(
        &mut self,
        address: RegisterAddresses,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<u8, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        self.read_single_byte_from_spi(cs, spi, address as u8)
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
        let mut read_buffer = [address_to_read | SPI_READ_BIT, 0xff];
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

    fn get_mode<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<mode::Mode, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let low_power_set = self.get_ctrl_reg_1_value(cs, spi)?.l_p_en()
            == ctrl_reg_1_value::LPEn::LowPowerEnabled;
        let high_resolution_output_set =
            self.get_ctrl_reg_4_value(cs, spi)?.hr()
                == enabled_enum::OnOff::Enabled;

        let mode = match (low_power_set, high_resolution_output_set) {
            (true, false) => mode::Mode::LowPower,
            (false, false) => mode::Mode::Normal,
            (false, true) => mode::Mode::HighResolution,
            _ => panic!("impossible mode"),
        };
        Ok(mode)
    }

    fn get_accel_raw<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
    ) -> Result<I16x3, Error<CsE, SpiE>>
    where
        CS: OutputPin<Error = CsE>,
        SPI: Transfer<u8, Error = SpiE> + Write<u8, Error = SpiE>,
    {
        let x_lo = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::OutXL as u8,
        )?;
        let x_hi = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::OutXH as u8,
        )?;
        let y_lo = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::OutYL as u8,
        )?;
        let y_hi = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::OutYH as u8,
        )?;
        let z_lo = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::OutZL as u8,
        )?;
        let z_hi = self.read_single_byte_from_spi(
            cs,
            spi,
            RegisterAddresses::OutZH as u8,
        )?;

        let x = i16::from_le_bytes([x_lo, x_hi]);
        let y = i16::from_le_bytes([y_lo, y_hi]);
        let z = i16::from_le_bytes([z_lo, z_hi]);

        Ok(I16x3 { x, y, z })
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

}
