use crate::drivers::drv8871::DRV8871;

use defmt::{info, panic, warn};
use embassy_stm32::gpio::{Pin, Output, Level, Speed};
use embassy_stm32::adc::{AdcPin, Resolution, Adc, Instance};
use embassy_time::{Duration, Timer};

const LINEAR_ACTUATOR_STROKE_LENGTH_MM: u8 = 100;
const SYRINGE_STROKE_LENGTH_MM: u8 = 70;
const MM_PER_UL: f32 = 70.0 / 20_000.0; // 20mL / 70mm

pub struct LinearActuatorSyringeDispenser<'a, MCPin1: Pin, MCPin2: Pin, AdcReadPin: Pin + AdcPin<AdcInstance>, AdcInstance: Instance> {
    mc: DRV8871<'a, MCPin1, MCPin2>,
    adc_pin: AdcReadPin,
    adc: Adc<'a, AdcInstance>,
    resolution: Resolution
}


impl<'a, MCPin1: Pin, MCPin2: Pin, AdcReadPin: Pin + AdcPin<AdcInstance>, AdcInstance: Instance> LinearActuatorSyringeDispenser<'a, MCPin1, MCPin2, AdcReadPin, AdcInstance> {

    pub fn new(mc_pin1: MCPin1, mc_pin2: MCPin2, adc_pin: AdcReadPin, mut adc: Adc<'a, AdcInstance>) -> Self {
        
        let resolution = Resolution::TwelveBit;

        let mc_out1 = Output::new(mc_pin1, Level::Low, Speed::High);
        let mc_out2 = Output::new(mc_pin2, Level::Low, Speed::High);

        adc.set_resolution(resolution);

        LinearActuatorSyringeDispenser {
            mc: DRV8871::new(mc_out1, mc_out2),
            adc_pin,
            adc,
            resolution
        }
    }  
    
    pub fn get_displacement(&mut self) -> f32 {
        LINEAR_ACTUATOR_STROKE_LENGTH_MM as f32 * self.adc.read(&mut self.adc_pin) as f32 / self.resolution.to_max_count() as f32
    }

    pub async fn dispense_ul(&mut self, ul: u16) {
        self.displace(ul as f32 * MM_PER_UL as f32).await;
    }

    async fn displace(&mut self, distance_mm: f32) {

        info!("displacing {} mm", distance_mm);

        if distance_mm + self.get_displacement() > SYRINGE_STROKE_LENGTH_MM as f32 * 0.90 {
            info!("Attempted to dispense further than the stroke of the linear actuator ({} > {}). The syringe has been spent. Resetting...", distance_mm + self.get_displacement(), SYRINGE_STROKE_LENGTH_MM as f32 * 0.95);
            
            self.fully_retract().await;
        }

        if distance_mm + self.get_displacement() > SYRINGE_STROKE_LENGTH_MM as f32 * 0.75 {
            warn!("Syringe is getting low. Replace it soon.");
        }

        if distance_mm < 0.0 {
            panic!("negative displacement is not supported. It could be, but there's no reason it would be used in this application.");
        }        

        let curr_displacement = self.get_displacement();
        let desired_displacement = curr_displacement + distance_mm;
        
        // pulling actuator in
        if distance_mm < 0.0 {
            self.retract();

            while self.get_displacement() > desired_displacement {
                Timer::after(Duration::from_micros(100)).await;
            }

            self.brake();
        }

        // pushing actuator out
        else {
            self.extrude();

            while self.get_displacement() < desired_displacement {
                Timer::after(Duration::from_micros(100)).await;
            }

            self.brake();
        }
    
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

