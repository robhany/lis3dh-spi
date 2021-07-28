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

#[derive(Default)]
pub struct Lis3dh {

}

impl Lis3dh {
    fn read_from_spi<CS, SPI, CsE, SpiE>(
        &mut self,
        cs: &mut CS,
        spi: &mut SPI,
        address_to_read: u8,
    ) -> Result<u8, Error<CsE, SpiE>>
        where
            CS: OutputPin<Error=CsE>,
            SPI: Transfer<u8, Error=SpiE> + Write<u8, Error=SpiE>,
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
            CS: OutputPin<Error=CsE>,
            SPI: Transfer<u8, Error=SpiE> + Write<u8, Error=SpiE>,
    {
        cs.set_low().map_err(Error::ChipSelectError)?;
        spi.write(&data).map_err(Error::SpiError)?;
        cs.set_high().map_err(Error::ChipSelectError)?;
        Ok(())
    }
}