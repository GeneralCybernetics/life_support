#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{OutputType, Output, Level, Speed};
use embassy_stm32::time::khz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let pump_in_enable = Output::new(p.PC7, Level::High, Speed::High);
    let pump_out_enable = Output::new(p.PC9, Level::Low, Speed::High);

    let mut pwm_in = SimplePwm::new(p.TIM1, Some(pump_in_ctl), None, None, None, khz(10), Default::default()); 
    let mut pwm_out = SimplePwm::new(p.TIM3, None, Some(pump_out_ctl), None, None, khz(10), Default::default()); 


    let max = pwm_in.get_max_duty();
    pwm_in.enable(Channel::Ch1);
    pwm_out.enable(Channel::Ch2);


    info!("PWM initialized");
    info!("PWM max duty {}", max);

    loop {
        pwm_in.set_duty(Channel::Ch1, 255);
        pwm_out.set_duty(Channel::Ch2, 0);


        loop {}
    }
}