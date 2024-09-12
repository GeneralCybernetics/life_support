#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer, Delay};
use embassy_stm32::adc::Adc;
use embassy_stm32::peripherals::{ADC2, PA1, PC6};
use life_support::syringe_dispenser::LinearActuatorSyringeDispenser;
use life_support::temp_regulator::TemperatureRegulator;
use life_support::drivers::thermistor::Thermistor;
use embassy_stm32::gpio::{Output, Level, Speed};
use static_cell::StaticCell;

use {defmt_rtt as _, panic_probe as _};

const TEMP_CONTROL_LOOP_HZ: f32 = 5.0;

// Define type aliases because task functions can't be generic
type ThermistorAdcInstance = ADC2;
type ThermistorAdcPin = PA1;
type CoolEnable = PC6;

#[embassy_executor::task]
async fn cool_medium(
    thermistor: &'static mut Thermistor<'_, ThermistorAdcPin, ThermistorAdcInstance>,
    temp_regulator: &'static mut TemperatureRegulator<'_>,
) {
    loop {
        let temp = thermistor.temp_c();
        info!("temp: {} C", temp);
        temp_regulator.regulate_temp(temp);
        Timer::after(Duration::from_millis((1.0 / TEMP_CONTROL_LOOP_HZ * 1000.0) as u64)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");
    let p = embassy_stm32::init(Default::default());

    // Linear actuator feedback potentiometer ADC setup
    let mut delay_pot = Delay;
    let adc_pot = Adc::new(p.ADC1);
    let adc_pot_pin = p.PA0;

    // Convert GPIO pins to Output<'a> objects
    let motor_pin1 = Output::new(p.PC7, Level::Low, Speed::High); 
    let motor_pin2 = Output::new(p.PC9, Level::Low, Speed::High); 

    // Thermistor ADC setup
    let adc_temp = Adc::new(p.ADC2);
    let adc_temp_pin = p.PA1;

    let mut syringe_dispenser = LinearActuatorSyringeDispenser::new(motor_pin1, motor_pin2, adc_pot_pin, adc_pot);

    // Static cells for TemperatureRegulator and Thermistor without generic arguments
    static TEMP_REGULATOR: StaticCell<TemperatureRegulator> = StaticCell::new();
    static THERMISTOR: StaticCell<Thermistor< ThermistorAdcPin, ThermistorAdcInstance>> = StaticCell::new();

    // Initialize TemperatureRegulator and Thermistor in static storage
    let temp_regulator = TEMP_REGULATOR.init(TemperatureRegulator::new(p.PC6, 2.0));
    let thermistor = THERMISTOR.init(Thermistor::new(adc_temp_pin, adc_temp));

    // Pass mutable references into the async function
    spawner.spawn(cool_medium(thermistor, temp_regulator)).unwrap();

    // This demo dispenses 750uL every 300ms. When it depletes the syringe, 
    // it resets to home, and repeats.
    loop {
        // syringe_dispenser.dispense_ul(750).await;
        Timer::after(Duration::from_millis(300)).await;
    }
}
