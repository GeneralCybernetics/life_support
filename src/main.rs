#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Delay};
use embassy_stm32::adc::Adc;
use life_support::incubator::LinearActuatorSyringeDispenser;

use {defmt_rtt as _, panic_probe as _};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let p = embassy_stm32::init(Default::default());
    
    let mut delay = Delay;
    let adc = Adc::new(p.ADC1, &mut delay);
    let adc_pin = p.PA0;

    let mut syringe_dispenser = LinearActuatorSyringeDispenser::new(p.PC7, p.PC9, adc_pin, adc);

    // this demo dispences 750uL every 300ms. When it depletes the syringe, 
    // it resets to home, and repeats.

    loop {
        syringe_dispenser.dispense_ul(750).await;
        Timer::after(Duration::from_millis(300)).await;
    }
}
