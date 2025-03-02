//! Rust I2C Scanner
//! https://github.com/JoaquinEduardoArreguez/stm32f1xx-rust-i2c-scanner
//! This assumes hal32f1xx I2C1 is used with standard mapping
//! SCL -> PB6
//! SDA -> PB7
//! This will check addresses 0 to 127 as I2C addresses are typically 7 bits long

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m::prelude::_embedded_hal_blocking_i2c_Write;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use embassy_stm32::time::Hertz;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = embassy_stm32::init(Default::default());
    let mut i2c = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        Hertz::khz(400),
        Default::default(),
    );

    // STARTING!
    info!("I2C Scan");


    info!("Start i2c scanning...");
    // I2C addresses are typically 7 bits long, 0..127
    for address in 0..=127 {
        match i2c.write(address, &[1]) {
            Ok(_) => {
                info!("Found device on address 0x{:02X}", address);
            }
            Err(_) => {}
        }
    }
    info!("Done!");
    loop {}
}