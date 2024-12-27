use maybe_async_cfg::maybe;

#[cfg(not(feature = "async"))]
use embedded_hal::delay::DelayNs;
#[cfg(feature = "async")]
use embedded_hal_async::delay::DelayNs;

use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    mode,
    register_address::{
        InternalControl0, InternalControl1, InternalControl2, InternalControl3, ProductId1, Status,
    },
    BandwidthMode, Error, MagneticField, Mmc5983, PhantomData, ProductId, Status as DeviceStatus,
    Temperature,
};

impl<I2C> Mmc5983<I2cInterface<I2C>, mode::OneShot> {
    /// Create new instance of the MMC5983 device communicating through I2C.
    pub fn new_with_i2c(i2c: I2C) -> Self {
        Mmc5983 {
            iface: I2cInterface { i2c },
            ctrl_reg0: InternalControl0::default(),
            ctrl_reg1: InternalControl1::default(),
            ctrl_reg2: InternalControl2::default(),
            ctrl_reg3: InternalControl3::default(),
            offset: MagneticField {
                x: 131072,
                y: 131072,
                z: 131072,
            },
            _mode: PhantomData,
        }
    }
}

impl<I2C, MODE> Mmc5983<I2cInterface<I2C>, MODE> {
    /// Destroy driver instance, return I2C bus.
    pub fn destroy(self) -> I2C {
        self.iface.i2c
    }
}

impl<SPI> Mmc5983<SpiInterface<SPI>, mode::OneShot> {
    /// Create new instance of the MMC5983 device communicating through SPI.
    pub fn new_with_spi(spi: SPI) -> Self {
        Mmc5983 {
            iface: SpiInterface { spi },
            ctrl_reg0: InternalControl0::default(),
            ctrl_reg1: InternalControl1::default(),
            ctrl_reg2: InternalControl2::default(),
            ctrl_reg3: InternalControl3::default(),
            offset: MagneticField {
                x: 131072,
                y: 131072,
                z: 131072,
            },
            _mode: PhantomData,
        }
    }
}

impl<SPI, MODE> Mmc5983<SpiInterface<SPI>, MODE> {
    /// Destroy driver instance, return SPI bus.
    pub fn destroy(self) -> SPI {
        self.iface.spi
    }
}

#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
impl<DI, CommE, MODE> Mmc5983<DI, MODE>
where
    DI: ReadData<Error = Error<CommE>> + WriteData<Error = Error<CommE>>,
{
    /// Initialize the device
    pub async fn init(&mut self) -> Result<(), Error<CommE>> {
        let product_id = self.product_id().await?;
        if !product_id.is_correct() {
            return Err(Error::InvalidId(product_id));
        }
        // Software reset
        self.software_reset().await?;
        // Read OTP
        self.read_otp().await?;
        // Enable interrupt on measurement done
        self.enable_meas_done_interrupt().await?;
        // Set default bandwidth (100Hz)
        self.set_bandwidth(BandwidthMode::Hz100).await
    }

    /// Software reset
    async fn software_reset(&mut self) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg1 | InternalControl1::SW_RST;
        self.iface.write_register(reg).await?;
        self.ctrl_reg1 = InternalControl1::default();
        Ok(())
    }

    /// Read OTP memory
    async fn read_otp(&mut self) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg0 | InternalControl0::OTP_READ;
        self.iface.write_register(reg).await?;
        self.ctrl_reg0 = reg;
        Ok(())
    }

    /// Enable measurement done interrupt
    async fn enable_meas_done_interrupt(&mut self) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg0 | InternalControl0::INT_MEAS_DONE_EN;
        self.iface.write_register(reg).await?;
        self.ctrl_reg0 = reg;
        Ok(())
    }

    /// Set measurement bandwidth
    pub async fn set_bandwidth(&mut self, bw: BandwidthMode) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg1.with_bandwidth(bw);
        self.iface.write_register(reg).await?;
        self.ctrl_reg1 = reg;
        Ok(())
    }

    /// Perform SET operation (magnetize sensor in positive direction)
    pub async fn set<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg0 | InternalControl0::SET;
        self.iface.write_register(reg).await?;
        delay.delay_ns(1000).await; // SET pulse is 500ns, datasheet isn't clear regarding wait time.
        self.ctrl_reg0 = self.ctrl_reg0.difference(InternalControl0::SET);
        Ok(())
    }

    /// Perform RESET operation (magnetize sensor in negative direction)
    pub async fn reset<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg0 | InternalControl0::RESET;
        self.iface.write_register(reg).await?;
        delay.delay_ns(1000).await; // RESET pulse is 500ns.
        self.ctrl_reg0 = self.ctrl_reg0.difference(InternalControl0::RESET);
        Ok(())
    }

    /// Perform SET operation (magnetize sensor in positive direction)
    pub async fn set_extra<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg3 | InternalControl3::ST_ENP;
        self.iface.write_register(reg).await?;
        delay.delay_ns(500).await; // SET pulse is 500ns
        self.ctrl_reg0 = self.ctrl_reg0.difference(InternalControl0::SET);
        Ok(())
    }

    /// Perform SET operation (magnetize sensor in positive direction)
    pub async fn reset_extra<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg3 | InternalControl3::ST_ENM;
        self.iface.write_register(reg).await?;
        delay.delay_ns(500).await; // SET pulse is 500ns
        self.ctrl_reg0 = self.ctrl_reg0.difference(InternalControl0::SET);
        Ok(())
    }

    /// Get device status
    pub async fn status(&mut self) -> Result<DeviceStatus, Error<CommE>> {
        self.iface
            .read_register::<Status>()
            .await
            .map(DeviceStatus::new)
    }

    /// Get product ID
    pub async fn product_id(&mut self) -> Result<ProductId, Error<CommE>> {
        self.iface.read_register::<ProductId1>().await
    }

    /// Get measured temperature
    pub async fn temperature(&mut self) -> Result<Temperature, Error<CommE>> {
        // Trigger temperature measurement
        let reg = self.ctrl_reg0 | InternalControl0::TM_T;
        self.iface.write_register(reg).await?;

        // Wait for measurement completion
        while !self.status().await?.temp_done() {}

        // Read temperature
        let temp = self.iface.read_register::<Temperature>().await?;

        Ok(temp)
    }

    /// Read magnetic field measurement
    pub async fn read_magnetic_field(&mut self) -> Result<MagneticField, Error<CommE>> {
        let mut buffer = [0u8; 7]; // For all registers from Xout0 (0x00) to XYZout2 (0x06)
        self.iface.read_consecutive(0x00, &mut buffer).await?;

        // Extract the 16-bit values
        let x_msb = buffer[0] as u32;
        let x_lsb = buffer[1] as u32;
        let y_msb = buffer[2] as u32;
        let y_lsb = buffer[3] as u32;
        let z_msb = buffer[4] as u32;
        let z_lsb = buffer[5] as u32;

        // Extract 2-bit values from XYZout2
        let xyz_2bit = buffer[6];
        let x_2bit = (xyz_2bit >> 6) & 0b11;
        let y_2bit = (xyz_2bit >> 4) & 0b11;
        let z_2bit = (xyz_2bit >> 2) & 0b11;

        // Combine into 18-bit values
        let x = (x_msb << 10) | (x_lsb << 2) | x_2bit as u32;
        let y = (y_msb << 10) | (y_lsb << 2) | y_2bit as u32;
        let z = (z_msb << 10) | (z_lsb << 2) | z_2bit as u32;

        Ok(MagneticField { x, y, z })
    }

    /// Finds offset values from sensor, according page Page 17.
    pub async fn calibrate_offset<D: DelayNs>(
        &mut self,
        delay: &mut D,
    ) -> Result<MagneticField, Error<CommE>> {
        // SET measurement
        self.set(delay).await?;
        let reg = self.ctrl_reg0 | InternalControl0::TM_M;
        self.iface.write_register(reg).await?;
        while !self.status().await?.meas_done() {}
        let field1 = self.read_magnetic_field().await?;

        // RESET measurement
        self.reset(delay).await?;
        let reg = self.ctrl_reg0 | InternalControl0::TM_M;
        self.iface.write_register(reg).await?;
        while !self.status().await?.meas_done() {}
        let field2 = self.read_magnetic_field().await?;

        // Calculate offset
        let offset = MagneticField {
            x: ((field1.x_raw() + field2.x_raw()) / 2),
            y: ((field1.y_raw() + field2.y_raw()) / 2),
            z: ((field1.z_raw() + field2.z_raw()) / 2),
        };

        self.offset = offset;

        Ok(offset)
    }

    pub async fn get_calibrated_field(&mut self) -> Result<MagneticField, Error<CommE>> {
        let reg = self.ctrl_reg0 | InternalControl0::TM_M;
        self.iface.write_register(reg).await?;
        while !self.status().await?.meas_done() {}
        let raw = self.read_magnetic_field().await?;

        let offset = self.offset;
        Ok(MagneticField {
            x: raw.x_raw() - offset.x,
            y: raw.y_raw() - offset.y,
            z: raw.z_raw() - offset.z,
        })
    }
}

#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
impl<DI, CommE> Mmc5983<DI, mode::Continuous>
where
    DI: ReadData<Error = Error<CommE>> + WriteData<Error = Error<CommE>>,
{
    /// Get the measured magnetic field in continuous mode
    pub async fn magnetic_field(&mut self) -> Result<MagneticField, Error<CommE>> {
        while !self.status().await?.meas_done() {}
        self.read_magnetic_field().await
    }
}

impl<DI, CommE> Mmc5983<DI, mode::OneShot>
where
    DI: ReadData<Error = Error<CommE>> + WriteData<Error = Error<CommE>>,
{
    /// Get the measured magnetic field in one-shot mode
    #[cfg(not(feature = "async"))]
    pub fn magnetic_field(&mut self) -> nb::Result<MagneticField, Error<CommE>> {
        self.magnetic_field_inner()
    }

    /// Get the measured magnetic field in one-shot mode
    #[cfg(feature = "async")]
    pub async fn magnetic_field(&mut self) -> Result<MagneticField, Error<CommE>> {
        loop {
            match self.magnetic_field_inner().await {
                Ok(field) => return Ok(field),
                Err(nb::Error::WouldBlock) => continue,
                Err(nb::Error::Other(e)) => return Err(e),
            }
        }
    }

    #[maybe(
        sync(cfg(not(feature = "async")), keep_self,),
        async(cfg(feature = "async"), keep_self,)
    )]
    async fn magnetic_field_inner(&mut self) -> nb::Result<MagneticField, Error<CommE>> {
        let status = self.status().await?;
        if status.meas_done() {
            Ok(self.read_magnetic_field().await?)
        } else {
            // Trigger new measurement
            let reg = self.ctrl_reg0 | InternalControl0::TM_M;
            self.iface.write_register(reg).await?;
            Err(nb::Error::WouldBlock)
        }
    }
}
