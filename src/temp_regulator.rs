use defmt::info;
use embassy_stm32::gpio::{Pin, Output, Level, Speed};

pub struct TemperatureRegulator<'d> {
    cool_enable: Output<'d>,
    temp_setpoint: f32
}

// simple bang-bang temperature regulator
impl<'d> TemperatureRegulator<'d> {
    pub fn new(cool_enable: impl Pin + 'd, temp_setpoint: f32) -> Self {
        let cool_enable = Output::new(cool_enable, Level::Low, Speed::High);

        TemperatureRegulator {
            cool_enable,
            temp_setpoint
        }
    }

    pub fn set_temp(&mut self, temp: f32) {
        info!("setting temp to {} C", temp);
        self.temp_setpoint = temp;
    }

    pub fn regulate_temp(&mut self, temp: f32) {
        if temp > self.temp_setpoint {
            self.cool_on();
        } else {
            self.cool_off();
        }
    }

    fn cool_off(&mut self) {
        self.cool_enable.set_low();
    }

    fn cool_on(&mut self) {
        self.cool_enable.set_high();
    }
}
