#![no_std]

use defmt::{info, panic, warn};
use embassy_stm32::gpio::{OutputType, AnyPin, Pin, Output, Level, Speed};
use embassy_stm32::adc::{AdcPin, Resolution, Adc, Instance};
use embassy_time::{Duration, Timer, Delay};

const LINEAR_ACTUATOR_STROKE_LENGTH_MM: u8 = 100;
const SYRINGE_STROKE_LENGTH_MM: u8 = 70;
const MM_PER_UL: f32 = 70.0 / 20_000.0; // 20mL / 70mm

pub struct LinearActuatorSyringeDispenser<'a, Out1: Pin, Out2: Pin, AdcReadPin: Pin + AdcPin<AdcInstance>, AdcInstance: Instance> {
    out_1: Output<'a, Out1>,
    out_2: Output<'a, Out2>,
    adc_pin: AdcReadPin,
    adc: Adc<'a, AdcInstance>,
    resolution: Resolution
}


impl<'a, Out1: Pin, Out2: Pin, AdcReadPin: Pin + AdcPin<AdcInstance>, AdcInstance: Instance> LinearActuatorSyringeDispenser<'a, Out1, Out2, AdcReadPin, AdcInstance> {

    pub fn new(out_1: Output<'a, Out1>, out_2: Output<'a, Out2>, adc_pin: AdcReadPin, mut adc: Adc<'a, AdcInstance>) -> Self {
        
        let resolution = Resolution::TwelveBit;

        adc.set_resolution(resolution);

        LinearActuatorSyringeDispenser {
            out_1,
            out_2,
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

        if distance_mm + self.get_displacement() > SYRINGE_STROKE_LENGTH_MM as f32 * 0.95 {
            info!("Attempted to dispense further than the stroke of the linear actuator ({} > {}). The syringe has been spent. Resetting...", distance_mm + self.get_displacement(), SYRINGE_STROKE_LENGTH_MM as f32 * 0.95);
            
            self.fully_retract().await;
        }

        if distance_mm + self.get_displacement() > SYRINGE_STROKE_LENGTH_MM as f32 * 0.80 {
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

            self.halt();
        }

        // pushing actuator out
        else {
            self.extrude();

            while self.get_displacement() < desired_displacement {
                Timer::after(Duration::from_micros(100)).await;
            }

            self.halt();
        }
    
    }

    fn extrude(&mut self) {
        self.out_1.set_high();
        self.out_2.set_low();
    }

    fn retract(&mut self) {
        self.out_1.set_low();
        self.out_2.set_high();
    }

    async fn fully_retract(&mut self) {
        self.retract();
        while self.get_displacement() > 0.0 + 0.001 {
            Timer::after(Duration::from_micros(100)).await;
        }
        self.halt();
    }

    fn halt(&mut self) {
        self.out_1.set_low();
        self.out_2.set_low();
    }
}

