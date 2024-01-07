// use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};

use embassy_stm32::gpio::{Pin, Output};

pub enum DRV8871State {
    Coast,
    Forward,
    Reverse,
    Brake,
}

// TODO: move this to PWM pins for frequency control
pub struct DRV8871<'a, Out1: Pin, Out2: Pin> {
    out_1: Output<'a, Out1>,
    out_2: Output<'a, Out2>,
    pub state: DRV8871State
}

impl<'a, Out1: Pin, Out2: Pin> DRV8871<'a, Out1, Out2> {

    pub fn new(out_1: Output<'a, Out1>, out_2: Output<'a, Out2>) -> DRV8871<'a, Out1, Out2> {
        return DRV8871 {
            out_1,
            out_2,
            state: DRV8871State::Coast
        }
    }

    pub fn forward(&mut self) {
        self.state = DRV8871State::Forward;
        self.out_1.set_high();
        self.out_2.set_low();
    }

    pub fn reverse(&mut self) {
        self.state = DRV8871State::Reverse;
        self.out_1.set_low();
        self.out_2.set_high();
    }

    pub fn brake(&mut self) {
        self.state = DRV8871State::Brake;
        self.out_1.set_high();
        self.out_2.set_high();
    }

    pub fn coast(&mut self) {
        self.state = DRV8871State::Coast;
        self.out_1.set_low();
        self.out_2.set_low();
    }
}
