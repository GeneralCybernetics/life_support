#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayUs;
use embassy_stm32::adc::{Adc, Temperature, Resolution, VrefInt};
use embassy_time::{Delay, Duration, Timer};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{OutputType, Speed, Level, Output};
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut p = embassy_stm32::init(Default::default());

    let pump_1 = Output::new(p.PC7, Level::Low, Speed::High);
    let pump_2 = Output::new(p.PC9, Level::Low, Speed::High);

    let mut delay = Delay;
    let mut adc = Adc::new(p.ADC1, &mut delay);
    adc.set_resolution(Resolution::TwelveBit);

    loop {
        
        info!("{}", adc.read(&mut p.PA0) as f32 / Resolution::TwelveBit.to_max_count() as f32);
        Timer::after(Duration::from_millis(50)).await;

    }
}