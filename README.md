# MMC5983-rs

A pure Rust driver for the MEMSIC's MMC5983 3-axis magnetic sensor.

## Features

- I2C and SPI interface support
- Full 18-bit operation mode with 0.0625mG per LSB resolution
- Supports both one-shot and continuous measurement modes
- Built-in SET/RESET function for offset compensation
- Temperature sensor reading support
- Configurable bandwidth from 100Hz to 800Hz
- Adjustable output data rates up to 1000Hz in continuous mode
- Interrupt support for measurement completion
- Async support via `embedded-hal-async` (optional feature)

## Hardware Support

The MMC5983MA sensor features:

- ±8 Gauss full-scale range
- 18-bit resolution (0.0625mG/LSB)
- 0.4mG total RMS noise
- 0.5° heading accuracy
- Temperature sensor

## Usage

Add this to your `Cargo.toml`:

```shell
cargo add mmc5983_rs
```

### Example (I2C)

```rust
use mmc5983_rs::Mmc5983;

let mut mag = Mmc5983::new_with_i2c(i2c);

// Initialize the device
mag.init()?;

// Optional: Calibrate offset
let offset = mag.calibrate_offset(&mut delay)?;

// Read magnetic field (one-shot mode)
let field = mag.magnetic_field()?;
println!("Magnetic field: X={} Y={} Z={} Gauss",
    field.x_gauss(), field.y_gauss(), field.z_gauss());

// Read temperature
let temp = mag.temperature()?;
println!("Temperature: {}°C", temp.degrees_celsius());
```

### Example (Continuous Mode)

```rust
// Switch to continuous mode with 100Hz measurements
let mut mag = mag.into_continuous(MagOutputDataRate::Hz100, None)?;

// Read measurements continuously
loop {
    if let Ok(field) = mag.magnetic_field() {
        let (x, y, z) = field.gauss();
        println!("Magnetic field: X={} Y={} Z={} Gauss", x, y, z);
    }
}
```

### Async Support

Enable the async feature in your `Cargo.toml`:

```toml
[dependencies]
mmc5983_rs = { version = "0.0.1", features = ["async"] }
```

Then use with `embedded-hal-async`:

```rust
let mut mag = Mmc5983::new_with_i2c(i2c);
mag.init().await?;
let field = mag.magnetic_field().await?;
```

## Complete Examples

Check the [examples](./examples/) directory for more usage examples:

- `navigator.rs`: Example using Linux embedded HAL on Navigator board
- `microbit-v2.rs`: Example for BBC micro:bit v2 using Embassy ( Embedded Async)

## License

Licensed under MIT license.

## References

For more details about the sensor, see the [MMC5983MA datasheet](https://www.memsic.com/Public/Uploads/uploadfile/files/20220119/MMC5983MADatasheetRevA.pdf).
