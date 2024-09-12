use crate::drivers::drv8871::DRV8871;

use defmt::{info, panic, warn};
use embassy_stm32::gpio::Output;
use embassy_stm32::adc::{Adc, Instance, AdcChannel};
use embassy_time::{Duration, Timer};

const LINEAR_ACTUATOR_STROKE_LENGTH_MM: u8 = 100;
const SYRINGE_STROKE_LENGTH_MM: u8 = 70;
const MM_PER_UL: f32 = 70.0 / 20_000.0; // 20mL / 70mm

pub struct LinearActuatorSyringeDispenser<'a, AdcReadPin: AdcChannel<AdcInstance>, AdcInstance: Instance> {
    mc: DRV8871<'a>,
    adc_pin: AdcReadPin,
    adc: Adc<'a, AdcInstance>,
}

impl<'a, AdcReadPin: AdcChannel<AdcInstance>, AdcInstance: Instance> LinearActuatorSyringeDispenser<'a, AdcReadPin, AdcInstance> {

    pub fn new(mc_pin1: Output<'a>, mc_pin2: Output<'a>, adc_pin: AdcReadPin, mut adc: Adc<'a, AdcInstance>) -> Self {
        adc.set_resolution(embassy_stm32::adc::Resolution::BITS12); // Set ADC resolution to 12 bits

        LinearActuatorSyringeDispenser {
            mc: DRV8871::new(mc_pin1, mc_pin2),
            adc_pin,
            adc,
        }
    }  
    
    pub fn get_displacement(&mut self) -> f32 {
        let adc_reading = self.adc.blocking_read(&mut self.adc_pin) as f32;
        LINEAR_ACTUATOR_STROKE_LENGTH_MM as f32 * adc_reading / 4095.0 // 12-bit max value = 4095
    }

    pub async fn dispense_ul(&mut self, ul: u16) {
        let distance_mm = ul as f32 * MM_PER_UL;
        self.displace(distance_mm).await;
    }

    async fn displace(&mut self, distance_mm: f32) {
        info!("Displacing by {} mm", distance_mm);

        let current_displacement = self.get_displacement();
        let desired_displacement = current_displacement + distance_mm;

        if desired_displacement > SYRINGE_STROKE_LENGTH_MM as f32 * 0.90 {
            warn!("Displacement {} exceeds syringe stroke length. Resetting actuator.", desired_displacement);
            self.fully_retract().await;
        } else if desired_displacement > SYRINGE_STROKE_LENGTH_MM as f32 * 0.75 {
            warn!("Syringe is running low. Consider replacing it.");
        } else if distance_mm < 0.0 {
            panic!("Negative displacement is not allowed.");
        }

        // Displacement logic
        if distance_mm > 0.0 {
            self.extrude();
            while self.get_displacement() < desired_displacement {
                Timer::after(Duration::from_micros(100)).await;
            }
        } else {
            self.retract();
            while self.get_displacement() > desired_displacement {
                Timer::after(Duration::from_micros(100)).await;
            }
        }
        self.brake();
    }

    fn extrude(&mut self) {
        self.mc.forward();
    }

    fn retract(&mut self) {
        self.mc.reverse();
    }

    async fn fully_retract(&mut self) {
        self.retract();
        while self.get_displacement() > 0.0 + 0.001 {
            Timer::after(Duration::from_micros(100)).await;
        }
        self.brake();
    }

    fn brake(&mut self) {
        self.mc.brake();
    }
}
