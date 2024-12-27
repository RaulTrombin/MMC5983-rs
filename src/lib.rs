#![deny(unsafe_code)]
#![allow(async_fn_in_trait)]
#![no_std]

//! MMC5983MA Magnetometer Driver
//!
//! This is a platform agnostic Rust driver for the MMC5983MA magnetometer
//! using the embedded-hal traits.

mod device_impl;
pub mod interface;
mod magnetometer;
pub mod register_address;
mod types;

use core::marker::PhantomData;

pub use crate::types::{
    mode, BandwidthMode, Error, MagMode, MagOutputDataRate, MagneticField, ProductId,
    SetResetPeriod, Status, Temperature,
};

use crate::register_address::{
    InternalControl0, InternalControl1, InternalControl2, InternalControl3,
};

/// MMC5983MA device driver
#[derive(Debug)]
pub struct Mmc5983<DI, MODE> {
    /// Digital interface: I2C or SPI
    iface: DI,
    /// Internal control registers
    ctrl_reg0: InternalControl0,
    ctrl_reg1: InternalControl1,
    ctrl_reg2: InternalControl2,
    ctrl_reg3: InternalControl3,
    /// Driver internal data
    offset: MagneticField,
    /// Operating mode marker
    _mode: PhantomData<MODE>,
}

mod private {
    use crate::interface;
    pub trait Sealed {}
    impl<SPI> Sealed for interface::SpiInterface<SPI> {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
}
