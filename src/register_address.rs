use crate::types::{BandwidthMode, MagOutputDataRate, ProductId, SetResetPeriod, StatusFlags};

/// Trait for reading from registers
pub trait RegRead<D = u8> {
    type Output;
    const ADDR: u8;
    fn from_data(data: D) -> Self::Output;
}

/// Trait for writing to registers
pub trait RegWrite<D = u8>: RegRead<D> {
    fn data(&self) -> D;
}

macro_rules! register {
    (@impl_reg_read $ty:ident, $addr:literal, $output:ty) => {
        impl RegRead for $ty {
            type Output = $output;
            const ADDR: u8 = $addr;
            #[inline(always)]
            fn from_data(data: u8) -> Self::Output {
                <$output>::from_bits_truncate(data)
            }
        }
    };
    (@impl_raw_reg_read $ty:ident, $addr:literal, $output:ty) => {
        impl RegRead for $ty {
            type Output = $output;
            const ADDR: u8 = $addr;
            #[inline(always)]
            fn from_data(data: u8) -> Self::Output {
                data
            }
        }
    };
    (@impl_reg_write $ty:ident, $addr:literal, $output:ident) => {
        register!(@impl_reg_read $ty, $addr, Self);
        impl RegWrite for $ty {
            fn data(&self) -> u8 {
                self.bits()
            }
        }
    };
    (
        #[doc = $name:expr]
        $(#[$meta:meta])*
        $vis:vis type $ty:ident: $addr:literal = u8;
    ) => {
        #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
        $(#[$meta])*
        $vis enum $ty {}
        register!(@impl_raw_reg_read $ty, $addr, u8);
    };
    (
        #[doc = $name:expr]
        $(#[$meta:meta])*
        $vis:vis type $ty:ident: $addr:literal = $output:ty;
    ) => {
        #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
        $(#[$meta])*
        $vis enum $ty {}
        register!(@impl_reg_read $ty, $addr, $output);
    };
    (
        #[doc = $name:expr]
        $(#[$meta:meta])*
        $vis:vis struct $ty:ident: $addr:literal {
            $(const $bit_name:ident = $bit_val:expr;)*
        }
    ) => {
        ::bitflags::bitflags! {
            #[doc = concat!($name, " register (`", stringify!($addr), "`)")]
            $(#[$meta])*
            $vis struct $ty: u8 {
                $(const $bit_name = $bit_val;)*
            }
        }
        register!(@impl_reg_write $ty, $addr, Self);
    };
}

// Magnetic Field Output Registers
register! {
    /// Xout[17:10] register
    pub type Xout0: 0x00 = u8;
}

register! {
    /// Xout[9:2] register
    pub type Xout1: 0x01 = u8;
}

register! {
    /// Yout[17:10] register
    pub type Yout0: 0x02 = u8;
}

register! {
    /// Yout[9:2] register
    pub type Yout1: 0x03 = u8;
}

register! {
    /// Zout[17:10] register
    pub type Zout0: 0x04 = u8;
}

register! {
    /// Zout[9:2] register
    pub type Zout1: 0x05 = u8;
}

/// Two's complement of X/Y/Z[1:0] bits
#[derive(Debug, Copy, Clone)]
pub struct XYZout2(u8);

impl XYZout2 {
    /// Extract X[1:0] bits
    pub fn x_bits(&self) -> u8 {
        (self.0 >> 6) & 0b11
    }

    /// Extract Y[1:0] bits
    pub fn y_bits(&self) -> u8 {
        (self.0 >> 4) & 0b11
    }

    /// Extract Z[1:0] bits
    pub fn z_bits(&self) -> u8 {
        (self.0 >> 2) & 0b11
    }
}

impl RegRead for XYZout2 {
    type Output = Self;
    const ADDR: u8 = 0x06;

    fn from_data(data: u8) -> Self::Output {
        XYZout2(data)
    }
}

register! {
    /// Temperature output register
    pub type Tout: 0x07 = u8;
}

register! {
    /// Status register
    pub type Status: 0x08 = StatusFlags;
}

register! {
    /// Internal Control 0 register
    #[derive(Debug, Default, Copy, Clone)]
    pub struct InternalControl0: 0x09 {
        const OTP_READ = 0b01000000;
        const AUTO_SR = 0b00100000;
        const RESET = 0b00010000;
        const SET = 0b00001000;
        const INT_MEAS_DONE_EN = 0b00000100;
        const TM_T = 0b00000010;
        const TM_M = 0b00000001;
    }
}

register! {
    /// Internal Control 1 register
    #[derive(Debug, Default, Copy, Clone)]
    pub struct InternalControl1: 0x0A {
        const SW_RST = 0b10000000;
        const YZ_INHIBIT = 0b00110000;
        const X_INHIBIT = 0b00001000;
        const BW1 = 0b00000010;
        const BW0 = 0b00000001;

        const BW = Self::BW1.bits() | Self::BW0.bits();
    }
}

impl InternalControl1 {
    pub const fn with_bandwidth(self, bw: BandwidthMode) -> Self {
        let reg = self.difference(Self::BW);
        Self::from_bits_truncate(reg.bits() | (bw as u8))
    }
}

register! {
    /// Internal Control 2 register
    #[derive(Debug, Default, Copy, Clone)]
    pub struct InternalControl2: 0x0B {
        const EN_PRD_SET = 0b10000000;
        const PRD_SET2 = 0b00100000;
        const PRD_SET1 = 0b00010000;
        const PRD_SET0 = 0b00001000;
        const CMM_EN = 0b00000100;
        const CM_FREQ2 = 0b00000100;
        const CM_FREQ1 = 0b00000010;
        const CM_FREQ0 = 0b00000001;

        const PRD_SET = Self::PRD_SET2.bits() | Self::PRD_SET1.bits() | Self::PRD_SET0.bits();
        const CM_FREQ = Self::CM_FREQ2.bits() | Self::CM_FREQ1.bits() | Self::CM_FREQ0.bits();
    }
}

impl InternalControl2 {
    pub fn with_set_period(self, period: SetResetPeriod) -> Self {
        let reg = self.difference(Self::PRD_SET);
        Self::from_bits_truncate(reg.bits() | ((period as u8) << 3))
    }

    pub fn with_output_rate(self, rate: MagOutputDataRate) -> Self {
        let reg = self.difference(Self::CM_FREQ);
        Self::from_bits_truncate(reg.bits() | rate.bits())
    }
    /// Get current output data rate configuration
    pub fn output_rate(&self) -> MagOutputDataRate {
        let bits = self.intersection(Self::CM_FREQ).bits();
        match bits {
            0b001 => MagOutputDataRate::Hz1,
            0b010 => MagOutputDataRate::Hz10,
            0b011 => MagOutputDataRate::Hz20,
            0b100 => MagOutputDataRate::Hz50,
            0b101 => MagOutputDataRate::Hz100,
            0b110 => MagOutputDataRate::Hz200,
            0b111 => MagOutputDataRate::Hz1000,
            _ => MagOutputDataRate::Hz1, // Default to lowest rate
        }
    }

    /// Get current SET/RESET period configuration
    pub fn set_period(&self) -> SetResetPeriod {
        let bits = (self.intersection(Self::PRD_SET).bits() >> 3) & 0b111;
        match bits {
            0 => SetResetPeriod::Every1,
            1 => SetResetPeriod::Every25,
            2 => SetResetPeriod::Every75,
            3 => SetResetPeriod::Every100,
            4 => SetResetPeriod::Every250,
            5 => SetResetPeriod::Every500,
            6 => SetResetPeriod::Every1000,
            7 => SetResetPeriod::Every2000,
            _ => SetResetPeriod::Every1, // Unreachable due to 3-bit mask
        }
    }
}

register! {
    /// Internal Control 3 register
    #[derive(Debug, Default, Copy, Clone)]
    pub struct InternalControl3: 0x0C {
        const SPI_3W = 0b01000000;
        const ST_ENM = 0b00000010;
        const ST_ENP = 0b00000001;
    }
}

register! {
    /// Product ID register
    pub type ProductId1: 0x2F = ProductId;
}

impl ProductId1 {
    pub(crate) const ID: u8 = 0x30;
}
