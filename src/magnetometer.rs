use maybe_async_cfg::maybe;

use crate::{
    interface::{ReadData, WriteData},
    mode,
    register_address::InternalControl2,
    types::SetResetPeriod,
    Error, MagMode, MagOutputDataRate, Mmc5983,
};

#[maybe(
    sync(cfg(not(feature = "async")), keep_self,),
    async(cfg(feature = "async"), keep_self,)
)]
impl<DI, CommE> Mmc5983<DI, mode::OneShot>
where
    DI: ReadData<Error = Error<CommE>> + WriteData<Error = Error<CommE>>,
{
    /// Change the magnetometer to continuous measurement mode
    ///
    /// # Arguments
    /// * `frequency` - The measurement frequency in continuous mode
    /// * `set_period` - Optional period for automatic SET/RESET operations
    pub async fn into_continuous(
        mut self,
        frequency: MagOutputDataRate,
        set_period: Option<SetResetPeriod>,
    ) -> Result<Mmc5983<DI, mode::Continuous>, Error<CommE>> {
        // Enable automatic SET/RESET if a period is specified
        if let Some(period) = set_period {
            let reg = self.ctrl_reg2.with_set_period(period) | InternalControl2::EN_PRD_SET;
            self.iface.write_register(reg).await?;
            self.ctrl_reg2 = reg;
        }

        // Set continuous mode with specified frequency
        let reg = self.ctrl_reg2.with_output_rate(frequency) | InternalControl2::CMM_EN;
        self.iface.write_register(reg).await?;
        self.ctrl_reg2 = reg;

        Ok(Mmc5983 {
            iface: self.iface,
            ctrl_reg0: self.ctrl_reg0,
            ctrl_reg1: self.ctrl_reg1,
            ctrl_reg2: self.ctrl_reg2,
            ctrl_reg3: self.ctrl_reg3,
            offset: self.offset,
            _mode: core::marker::PhantomData,
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
    /// Change the magnetometer back to one-shot mode
    pub async fn into_oneshot(mut self) -> Result<Mmc5983<DI, mode::OneShot>, Error<CommE>> {
        // Disable continuous mode and automatic SET/RESET
        let reg = self
            .ctrl_reg2
            .difference(InternalControl2::CMM_EN | InternalControl2::EN_PRD_SET);
        self.iface.write_register(reg).await?;
        self.ctrl_reg2 = reg;

        Ok(Mmc5983 {
            iface: self.iface,
            ctrl_reg0: self.ctrl_reg0,
            ctrl_reg1: self.ctrl_reg1,
            ctrl_reg2: self.ctrl_reg2,
            ctrl_reg3: self.ctrl_reg3,
            offset: self.offset,
            _mode: core::marker::PhantomData,
        })
    }

    /// Change the continuous mode measurement frequency
    pub async fn set_frequency(
        &mut self,
        frequency: MagOutputDataRate,
    ) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg2.with_output_rate(frequency);
        self.iface.write_register(reg).await?;
        self.ctrl_reg2 = reg;
        Ok(())
    }

    /// Enable automatic SET/RESET operations with specified period
    pub async fn enable_auto_set_reset(
        &mut self,
        period: SetResetPeriod,
    ) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg2.with_set_period(period) | InternalControl2::EN_PRD_SET;
        self.iface.write_register(reg).await?;
        self.ctrl_reg2 = reg;
        Ok(())
    }

    /// Disable automatic SET/RESET operations
    pub async fn disable_auto_set_reset(&mut self) -> Result<(), Error<CommE>> {
        let reg = self.ctrl_reg2.difference(InternalControl2::EN_PRD_SET);
        self.iface.write_register(reg).await?;
        self.ctrl_reg2 = reg;
        Ok(())
    }

    /// Get current measurement mode configuration
    pub fn get_mode_config(&self) -> MagMode {
        let frequency = self.ctrl_reg2.output_rate();
        let auto_set = self.ctrl_reg2.contains(InternalControl2::EN_PRD_SET);
        let set_period = if auto_set {
            Some(self.ctrl_reg2.set_period())
        } else {
            None
        };

        MagMode::Continuous {
            frequency,
            set_period,
        }
    }
}
