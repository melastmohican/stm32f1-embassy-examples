#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_blocking_i2c_Read;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embassy_stm32::time::Hertz;
use panic_probe as _;

#[entry]
fn main() -> ! {
    info!("I2C scan...");

    // Get access to device peripherals
    let p = embassy_stm32::init(Default::default());
    let mut i2c = embassy_stm32::i2c::I2c::new_blocking(
        p.I2C1,
        p.PB6,
        p.PB7,
        Hertz::khz(400),
        Default::default(),
    );

    // Scan I2C addresses
    info!("Scan I2C addresses...");
    for addr in 0x01..=0x7F {
        //info!("Scanning 0x{:02X}", addr);
        let mut buf = [0u8; 1];
        match i2c.read(addr, &mut buf) {
            Ok(_) => info!("Device found at 0x{:02X}", addr),
            Err(_) => {} // No response, ignore
        }
    }

    info!("I2C scan complete.");
    loop {}
}
