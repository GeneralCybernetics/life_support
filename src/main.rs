#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Delay};
use embassy_stm32::adc::Adc;
use embassy_stm32::peripherals::{ADC1, ADC2, PA1, PA0, PC7, PC9};
use life_support::incubator::LinearActuatorSyringeDispenser;
use life_support::drivers::thermistor::Thermistor;
use static_cell::make_static;

use {defmt_rtt as _, panic_probe as _};

const LOG_TEMP_HZ: f32 = 5.0;

// need to define these because task functions can't be generic :(
#[allow(dead_code)]
type SyringeDispenserMC1 = PC7;
#[allow(dead_code)]
type SyringeDispenserMC2 = PC9;
#[allow(dead_code)]
type SyringeDispenserAdcInstance = ADC1;
#[allow(dead_code)]
type SyringeDispenserAdcPin = PA0;

type ThermistorAdcInstance = ADC2;
type ThermistorAdcPin = PA1;

#[embassy_executor::task]
async fn regulate_temp(thermistor: &'static mut Thermistor<'_, ThermistorAdcPin, ThermistorAdcInstance>) {
    loop {
        info!("temp: {} C", thermistor.temp_c());
        Timer::after(Duration::from_millis((1.0 / LOG_TEMP_HZ * 1000.0) as u64)).await;
    }
}


#[embassy_executor::task]
async fn replace_medium(syringe_dispenser: &'static mut LinearActuatorSyringeDispenser<'_, SyringeDispenserMC1, SyringeDispenserMC2, SyringeDispenserAdcPin, SyringeDispenserAdcInstance>) {
    loop 
        {
            syringe_dispenser.dispense_ul(750).await;
            Timer::after(Duration::from_millis(300)).await;
        }
}



#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");
    let p = embassy_stm32::init(Default::default());
    
    let mut delay_pot = Delay;
    let adc_pot = Adc::new(p.ADC1, &mut delay_pot);
    let adc_pot_pin = p.PA0;


    let mut delay_temp = Delay;
    let adc_temp = Adc::new(p.ADC2, &mut delay_temp);
    let adc_temp_pin = p.PA1;


    let mut syringe_dispenser = LinearActuatorSyringeDispenser::new(p.PC7, p.PC9, adc_pot_pin, adc_pot);
    let thermistor = make_static!(Thermistor::new(adc_temp_pin, adc_temp));

    spawner.spawn(regulate_temp(thermistor)).unwrap();

    // this demo dispences 750uL every 300ms. When it depletes the syringe, 
    // it resets to home, and repeats.
    loop {
        syringe_dispenser.dispense_ul(750).await;
        Timer::after(Duration::from_millis(300)).await;
    }
}
