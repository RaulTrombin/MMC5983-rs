#[cfg(target_os = "linux")]
fn main() {
    use linux_embedded_hal::{
        spidev::{SpiModeFlags, Spidev, SpidevOptions},
        Delay, SpidevDevice,
    };
    use mmc5983_rs::Mmc5983;

    let mut spi = Spidev::open("/dev/spidev1.1").unwrap();
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(1_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).unwrap();

    let mut delay = Delay;
    let mut mag = Mmc5983::new_with_spi(SpidevDevice::from(linux_embedded_hal::SpidevDevice(spi)));

    println!("{:?}", mag.product_id().unwrap());
    mag.init().unwrap();

    println!("Reading measurements in continuous mode...");
    println!("Press Ctrl-C to stop");

    println!("Calibrating offset: {:?}", mag.calibrate_offset(&mut delay));

    loop {
        if let Ok(field) = mag.get_calibrated_field() {
            let (x, y, z) = field.gauss();
            println!("Magnetic field: X={:.7} Y={:.7} Z={:.7} Gauss", x, y, z);
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {}
