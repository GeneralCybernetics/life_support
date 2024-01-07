#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use life_support::incubator::PeristalticPump;

use {defmt_rtt as _, panic_probe as _};


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let p = embassy_stm32::init(Default::default());
    let mut pump = PeristalticPump::new(p.PC7, p.PC9);

    let button = Input::new(p.PA0, Pull::Down);
    let mut button = ExtiInput::new(button, p.EXTI0);

    loop {
        button.wait_for_rising_edge().await;
        pump.cycle();
    }
}
