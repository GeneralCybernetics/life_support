use defmt::info;
use embassy_stm32::gpio::{Pin, Output, Level, Speed};

pub struct TemperatureRegulator<'a, CoolEnable: Pin> {
    cool_enable: Output<'a, CoolEnable>,
    temp_setpoint: f32
}

// simple bang-bang temperature regulator
impl <'a, CoolEnable: Pin> TemperatureRegulator<'a, CoolEnable> {
    pub fn new(cool_enable: CoolEnable, temp_setpoint: f32) -> Self {
        let cool_enable = Output::new(cool_enable, Level::Low, Speed::High);

        TemperatureRegulator {
            cool_enable,
            temp_setpoint
        }
    }

    pub async fn set_temp(&mut self, temp: f32) {
        info!("setting temp to {} C", temp);
        self.temp_setpoint = temp;
    }

    pub async fn regulate_temp(&mut self, temp: f32) {
        if temp > self.temp_setpoint {
            self.cool_on().await;
        } else {
            self.cool_off().await;
        }
    }

    async fn cool_off(&mut self) {
        self.cool_enable.set_low();
    }

    async fn cool_on(&mut self) {
        self.cool_enable.set_high();
    }
}
