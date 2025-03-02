//! Reads data from a BME280 over i2c
//!
//! This assumes that a BME280 is connected with clk on PB6 and data on PB7.
//!
//! For the Adafruit breakout boards PB6 should be connected to SCK and PB7 to SDI
//!
//! This program writes the sensor values to the debug output provided by semihosting
//! you must enable semihosting in gdb with `monitor arm semihosting enable` I have it
//! added to my `.gdbinit`. Then the debug infomation will be printed in your openocd
//! terminal.
//!
//! This program dose not fit on my blue pill unless compiled in release mode
//! eg. `cargo run --example i2c-bme280 --features "stm32f103 bme280 rt" --release`
//! However as noted above the debug output with the read values will be in the openocd
//! terminal.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use bme280::i2c::BME280;
use cortex_m_rt::entry;
use embassy_stm32::time::Hertz;
use embassy_time::Delay;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let p = embassy_stm32::init(Default::default());
    let i2c = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        Hertz::khz(400),
        Default::default(),
    );

    let mut delay = Delay;
    // The Adafruit boards have address 0x77 without closing the jumper on the back, the BME280 lib connects to 0x77 with `new_secondary`, use
    // `new_primary` for 0x76 if you close the jumper/solder bridge.
    let mut bme280 = BME280::new_secondary(i2c);
    bme280
        .init(&mut delay)
        .map_err(|error| {
            error!(
                "Could not initialize bme280, Error: {}",
                Debug2Format(&error)
            );
        })
        .unwrap();
    loop {
        match bme280.measure(&mut delay) {
            Ok(measurements) => {
                info!("Relative Humidity = {}%", measurements.humidity);
                info!("Temperature = {} deg C", measurements.temperature);
                info!("Pressure = {} pascals", measurements.pressure)
            }
            Err(error) => {
                error!(
                    "Could not read bme280 due to error: {}",
                    Debug2Format(&error)
                );
            }
        }
    }
}
