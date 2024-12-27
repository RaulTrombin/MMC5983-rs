use bitflags::bitflags;

use crate::register_address::{ProductId1, RegRead};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<CommE> {
    /// I²C / SPI communication error
    Comm(CommE),
    /// Invalid input data provided
    InvalidInputData,
    /// Invalid input data provided
    InvalidId(ProductId),
}

impl<CommE> From<CommE> for Error<CommE> {
    fn from(e: CommE) -> Self {
        Self::Comm(e)
    }
}

/// Device operation modes
pub mod mode {
    /// Marker type for magnetometer in one-shot (single) mode.
    #[derive(Debug)]
    pub enum OneShot {}
    /// Marker type for magnetometer in continuous mode.
    #[derive(Debug)]
    pub enum Continuous {}
}

/// A ProductId - used to identify the device.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProductId {
    raw: u8,
}

impl ProductId {
    pub(crate) const fn from_bits_truncate(raw: u8) -> Self {
        Self { raw }
    }

    /// Raw product ID.
    pub const fn raw(&self) -> u8 {
        self.raw
    }

    /// Check if the ID corresponds to the expected value.
    pub const fn is_correct(&self) -> bool {
        self.raw == ProductId1::ID
    }
}

/// A magnetic field measurement.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MagneticField {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) z: u32,
}

impl RegRead<(u32, u32, u32)> for MagneticField {
    type Output = Self;

    /// X_OUT0 register starting address
    const ADDR: u8 = 0x00;

    #[inline(always)]
    fn from_data((x, y, z): (u32, u32, u32)) -> Self::Output {
        Self { x, y, z }
    }
}

impl MagneticField {
    const SENSITIVITY: f32 = 16384.0; // counts per Gauss in 18-bit mode
    const COUNTS_TO_GAUSS: f32 = 1.0 / Self::SENSITIVITY;

    /// Raw magnetic field in X-direction
    #[inline]
    pub fn x_raw(&self) -> u32 {
        self.x
    }

    /// Raw magnetic field in Y-direction
    #[inline]
    pub fn y_raw(&self) -> u32 {
        self.y
    }

    /// Raw magnetic field in Z-direction
    #[inline]
    pub fn z_raw(&self) -> u32 {
        self.z
    }

    /// Magnetic field in X-direction in Gauss with proper scaling
    #[inline]
    pub fn x_gauss(&self) -> f32 {
        self.x_raw() as i32 as f32 * Self::COUNTS_TO_GAUSS
    }

    /// Magnetic field in Y-direction in Gauss with proper scaling
    #[inline]
    pub fn y_gauss(&self) -> f32 {
        self.y_raw() as i32 as f32 * Self::COUNTS_TO_GAUSS
    }

    /// Magnetic field in Z-direction in Gauss with proper scaling
    #[inline]
    pub fn z_gauss(&self) -> f32 {
        self.z_raw() as i32 as f32 * Self::COUNTS_TO_GAUSS
    }

    /// Magnetic field in X-, Y- and Z-directions in Gauss.
    #[inline]
    pub fn gauss(&self) -> (f32, f32, f32) {
        (self.x_gauss(), self.y_gauss(), self.z_gauss())
    }
}

/// Magnetometer output data rate/bandwidth
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BandwidthMode {
    /// BW = 00, 100Hz bandwidth
    Hz100,
    /// BW = 01, 200Hz bandwidth
    Hz200,
    /// BW = 10, 400Hz bandwidth
    Hz400,
    /// BW = 11, 800Hz bandwidth
    Hz800,
}

/// Magnetometer operating mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagMode {
    /// Continuous measurement mode
    Continuous {
        /// Measurement frequency
        frequency: MagOutputDataRate,
        /// Set/Reset period
        set_period: Option<SetResetPeriod>,
    },
    /// Single measurement mode
    OneShot,
}

/// Magnetometer output data rate for continuous mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagOutputDataRate {
    /// 1 Hz
    Hz1,
    /// 10 Hz
    Hz10,
    /// 20 Hz
    Hz20,
    /// 50 Hz
    Hz50,
    /// 100 Hz
    Hz100,
    /// 200 Hz (when BW=01)
    Hz200,
    /// 1000 Hz (when BW=11)
    Hz1000,
}

impl MagOutputDataRate {
    /// Convert frequency to register bits
    pub(crate) fn bits(&self) -> u8 {
        match self {
            MagOutputDataRate::Hz1 => 0b001,
            MagOutputDataRate::Hz10 => 0b010,
            MagOutputDataRate::Hz20 => 0b011,
            MagOutputDataRate::Hz50 => 0b100,
            MagOutputDataRate::Hz100 => 0b101,
            MagOutputDataRate::Hz200 => 0b110,
            MagOutputDataRate::Hz1000 => 0b111,
        }
    }
}

/// Period for automatic SET operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetResetPeriod {
    /// Every measurement
    Every1 = 0,
    /// Every 25 measurements
    Every25 = 1,
    /// Every 75 measurements
    Every75 = 2,
    /// Every 100 measurements
    Every100 = 3,
    /// Every 250 measurements
    Every250 = 4,
    /// Every 500 measurements
    Every500 = 5,
    /// Every 1000 measurements
    Every1000 = 6,
    /// Every 2000 measurements
    Every2000 = 7,
}

bitflags! {
    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct StatusFlags: u8 {
        /// Measurement done
        const MEAS_M_DONE = 0b00000001;
        /// Temperature measurement done
        const MEAS_T_DONE = 0b00000010;
        /// OTP read done
        const OTP_READ_DONE = 0b00010000;
    }
}

/// Device status
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Status {
    flags: StatusFlags,
}

impl Status {
    pub(crate) const fn new(flags: StatusFlags) -> Self {
        Self { flags }
    }

    /// Check if magnetic measurement is complete
    #[inline]
    pub const fn meas_done(&self) -> bool {
        self.flags.contains(StatusFlags::MEAS_M_DONE)
    }

    /// Check if temperature measurement is complete
    #[inline]
    pub const fn temp_done(&self) -> bool {
        self.flags.contains(StatusFlags::MEAS_T_DONE)
    }

    /// Check if OTP read completed
    #[inline]
    pub const fn otp_read_done(&self) -> bool {
        self.flags.contains(StatusFlags::OTP_READ_DONE)
    }
}

/// A temperature measurement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature {
    pub(crate) raw: u8,
}

impl RegRead<u8> for Temperature {
    type Output = Self;

    /// TOUT register
    const ADDR: u8 = 0x07;

    #[inline]
    fn from_data(data: u8) -> Self::Output {
        Temperature { raw: data }
    }
}

impl Temperature {
    /// Raw temperature reading
    #[inline]
    pub const fn raw(&self) -> u8 {
        self.raw
    }

    /// Temperature in degrees Celsius
    #[inline]
    pub fn degrees_celsius(&self) -> f32 {
        // -75°C to 125°C range, ~0.8°C/LSB
        // 0x00 = -75°C
        -75.0 + (self.raw as f32) * 0.8
    }
}
