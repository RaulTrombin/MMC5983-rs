#![no_main]
#![no_std]

//! Example of using the async feature of the MMC5983 driver.
//! Uses [embassy](https://embassy.dev) for the HAL and the executor.
//!
//! Make sure [probe-rs](https://probe.rs) is installed.
//!
//! Run it using
//!
//! ```sh
//! cargo run --example microbit-v2 --target thumbv7em-none-eabihf --features async
//! ```

use embassy_nrf::{self as hal, twim::Twim};
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use hal::twim;
use mmc5983_rs::Mmc5983;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _; // Panic handler

hal::bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(_s: embassy_executor::Spawner) -> ! {
    // Init RTT control block
    rtt_init_print!();

    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use ``dp` to get a handle to the peripherals
    let dp = hal::init(Default::default());

    rprintln!("Starting");

    let config = twim::Config::default();
    let twim0 = Twim::new(dp.TWISPI0, Irqs, dp.P0_16, dp.P0_08, config);

    let mut sensor = Mmc5983::new_with_i2c(twim0);
    let id = sensor.product_id().await.unwrap();
    rprintln!("{:#02x?}", id);

    sensor.init().await.unwrap();
    let mut delay = Delay;

    let offset = sensor.calibrate_offset(&mut delay).await.unwrap();
    rprintln!("Calibrated with offset: {:?}", offset);

    loop {
        match sensor.magnetic_field().await {
            Ok(data) => {
                rprintln!(
                    "Magnetic field: x {} y {} z {}",
                    data.x_gauss(),
                    data.y_gauss(),
                    data.z_gauss()
                );
            }
            Err(err) => {
                rprintln!("Some error occurred {:?}", err)
            }
        }
        Delay.delay_ms(200).await;
    }
}
