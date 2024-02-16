use crate::drivers::drv8871::DRV8871;

use defmt::info;
use embassy_stm32::gpio::{Pin, Output, Level, Speed};

pub struct TemperatureRegulator<'a, MCPin1: Pin, MCPin2: Pin> {
    mc: DRV8871<'a, MCPin1, MCPin2>,
    temp_setpoint: f32
    
}

// simple bang-bang temperature regulator
impl <'a, MCPin1: Pin, MCPin2: Pin> TemperatureRegulator<'a, MCPin1, MCPin2> {
    pub fn new(mc_pin1: MCPin1, mc_pin2: MCPin2, temp_setpoint: f32) -> Self {
        let mc_out1 = Output::new(mc_pin1, Level::Low, Speed::High);
        let mc_out2 = Output::new(mc_pin2, Level::Low, Speed::High);

        TemperatureRegulator {
            mc: DRV8871::new(mc_out1, mc_out2),
            temp_setpoint
        }
    }

    pub async fn set_temp(&mut self, temp: f32) {
        info!("setting temp to {} C", temp);
        self.temp_setpoint = temp;
    }

    pub async fn regulate_temp(&mut self, temp: f32) {
        if temp > self.temp_setpoint {
            self.bang_cool().await;
        } else {
            self.bang_heat().await;
        }
    }

    async fn bang_heat(&mut self) {
        self.mc.forward();
    }

    async fn bang_cool(&mut self) {
        self.mc.reverse();
    }
}
