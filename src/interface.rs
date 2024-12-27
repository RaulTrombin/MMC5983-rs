//! I2C/SPI interfaces
use maybe_async_cfg::maybe;

#[cfg(not(feature = "async"))]
use embedded_hal::{i2c, spi};
#[cfg(feature = "async")]
use embedded_hal_async::{i2c, spi};

use crate::{
    private,
    register_address::{RegRead, RegWrite},
    Error,
};

/// MMC5983 I2C address (7-bit)
pub(crate) const MMC5983_ADDR: u8 = 0b0110000;

/// I2C interface
#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// SPI interface
#[derive(Debug)]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// Write data
#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;

    /// Write to register
    async fn write_register<R: RegWrite>(&mut self, reg: R) -> Result<(), Self::Error>;
}

#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;

    async fn write_register<R: RegWrite>(&mut self, reg: R) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [R::ADDR, reg.data()];
        self.i2c
            .write(MMC5983_ADDR, &payload)
            .await
            .map_err(Error::Comm)
    }
}

#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
impl<SPI, CommE> WriteData for SpiInterface<SPI>
where
    SPI: spi::SpiDevice<u8, Error = CommE>,
{
    type Error = Error<CommE>;

    async fn write_register<R: RegWrite>(&mut self, reg: R) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [R::ADDR & !SPI_RW, reg.data()];
        self.spi.write(&payload).await.map_err(Error::Comm)
    }
}

/// Read data
#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
pub trait ReadData: private::Sealed {
    /// Error type
    type Error;

    /// Read a register
    async fn read_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error>;

    /// Read multiple consecutive registers
    async fn read_consecutive(
        &mut self,
        start_addr: u8,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error>;
}

const SPI_RW: u8 = 1 << 7;

#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;

    async fn read_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error> {
        let mut data = [0];
        self.i2c
            .write_read(MMC5983_ADDR, &[R::ADDR], &mut data)
            .await
            .map_err(Error::Comm)?;

        Ok(R::from_data(data[0]))
    }

    async fn read_consecutive(
        &mut self,
        start_addr: u8,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(MMC5983_ADDR, &[start_addr], buffer)
            .await
            .map_err(Error::Comm)
    }
}

#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
impl<SPI, CommE> ReadData for SpiInterface<SPI>
where
    SPI: spi::SpiDevice<u8, Error = CommE>,
{
    type Error = Error<CommE>;

    async fn read_register<R: RegRead>(&mut self) -> Result<R::Output, Self::Error> {
        let mut data = [SPI_RW | R::ADDR, 0];
        self.spi
            .transfer_in_place(&mut data)
            .await
            .map_err(Error::Comm)?;

        Ok(R::from_data(data[1]))
    }

    async fn read_consecutive(
        &mut self,
        start_addr: u8,
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        // Create a new buffer with space for the address byte plus the data
        let mut data = [0u8; 32]; // Size should be larger than any consecutive read we'll do
        if buffer.len() >= data.len() {
            return Err(Error::InvalidInputData);
        }

        // Setup the command byte and copy in any data
        data[0] = SPI_RW | start_addr;

        // Only use the part of the array we need
        let data_slice = &mut data[..=buffer.len()];

        self.spi
            .transfer_in_place(data_slice)
            .await
            .map_err(Error::Comm)?;

        buffer.copy_from_slice(&data_slice[1..]);
        Ok(())
    }
}
