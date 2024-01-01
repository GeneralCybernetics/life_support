#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{OutputType, Output, Level, Speed};
use embassy_time::Duration;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_time::Timer;
use LifeSupport::PeristalticPump;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let pump_1 = Output::new(p.PC7, Level::Low, Speed::High);
    let pump_2 = Output::new(p.PC9, Level::Low, Speed::High);
    let pump_enable = Output::new(p.PA9, Level::Low, Speed::High);

    let mut pump = PeristalticPump::new(pump_1, pump_2, pump_enable);

    loop {
        pump.start_in();
        Timer::after(Duration::from_millis(1000)).await;
        pump.stop();
        Timer::after(Duration::from_millis(300)).await;
        pump.start_out();
        Timer::after(Duration::from_millis(1000)).await;
        pump.stop();
        Timer::after(Duration::from_millis(300)).await;
    }
}