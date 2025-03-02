//! Example of using I2C.
//! Scans available I2C devices on bus and print the result.
//! https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/i2c_scanner.rs

#![no_std]
#![no_main]

use core::ops::Range;
use cortex_m::prelude::_embedded_hal_blocking_i2c_Write;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embassy_stm32::time::Hertz;
use panic_probe as _;

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x7F;

#[entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut i2c = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        Hertz::khz(400),
        Default::default(),
    );

    info!("Start i2c scanning...");
    for addr in 0x00_u8..0x7F {
        // Write the empty array and check the slave response.
        if VALID_ADDR_RANGE.contains(&addr) && i2c.write(addr, &[1]).is_ok() {
            info!("Found device on address 0x{:02X}", addr);
        }
    }
    info!("Done!");

    loop {}
}
