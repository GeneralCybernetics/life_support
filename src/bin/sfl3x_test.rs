#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Error, I2c};
use embassy_stm32::time::Hertz;
use {defmt_rtt as _, panic_probe as _};
use embassy_stm32::gpio::{Output, Level, Speed};
use embassy_time::{Duration, Timer};

const ADDRESS: u8 = 0x08; // is this correct 7 bit addressing??
const PROD_IDENTIFY: [u8; 4] = [0x36, 0x7C, 0xE1, 0x02];
const CMD_START_MEASUREMENT: [u8; 2] = [0x36, 0x08];

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    
    let mut test_output_1 = Output::new(p.PB10, Level::High, Speed::VeryHigh);
    let mut test_output_2 = Output::new(p.PB11, Level::High, Speed::VeryHigh);

    test_output_1.set_high();
    test_output_2.set_high();
    loop{}

    // let mut i2c = I2c::new_blocking(p.I2C2, p.PB10, p.PB11, Hertz(400_000), Default::default());

    

    // match i2c.blocking_write(8, &CMD_START_MEASUREMENT ) {
    //     Ok(()) => info!("Success"),
    //     Err(Error::Timeout) => error!("Operation timed out"),
    //     Err(e) => error!("I2c Error: {:?}", e),
    // };

    // let mut data = [0u8; 18];

    // match i2c.blocking_write_read(ADDRESS, &PROD_IDENTIFY, &mut data) {
    //     Ok(()) => info!("Success"),
    //     Err(Error::Timeout) => error!("Operation timed out"),
    //     Err(e) => error!("I2c Error: {:?}", e),
    // }

}