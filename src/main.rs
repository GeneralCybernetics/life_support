#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{OutputType, Output, Level, Speed};
use embassy_time::{Duration, Timer, Delay};
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_stm32::adc::{AdcPin, Resolution, Adc, Instance};

use LifeSupport::LinearActuatorSyringeDispenser;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let linear_actuator_1 = Output::new(p.PC7, Level::Low, Speed::High);
    let linear_actuator_2 = Output::new(p.PC9, Level::Low, Speed::High);
    let mut delay = Delay;
    let mut adc = Adc::new(p.ADC1, &mut delay);
    let adc_pin = p.PA0;

    let mut syringe_dispenser = LinearActuatorSyringeDispenser::new(linear_actuator_1, linear_actuator_2, adc_pin, adc);

    // this demo dispences 750uL every 300ms. When it depletes the syringe, 
    // it resets to home, and repeats.

    loop {
        syringe_dispenser.dispense_ul(750).await;
        Timer::after(Duration::from_millis(300)).await;
    }
}