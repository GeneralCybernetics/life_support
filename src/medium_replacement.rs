#![no_std]

use embassy_stm32::gpio::{OutputType, Pin, Output, Level, Speed};


enum PumpState {
    Idle,
    PumpingIn,
    PumpingOut
}

pub struct PeristalticPump<'a, Out1: Pin, Out2: Pin, En: Pin> {
    out_1: Output<'a, Out1>,
    out_2: Output<'a, Out2>,
    enable: Output<'a, En>,
    state: PumpState
}

// Pump In: In 1, Out 0
// Pump Out: In 0, Out 1

impl<'a, Out1: Pin, Out2: Pin, En: Pin> PeristalticPump<'a, Out1, Out2, En> {

    pub fn new(out_1: Output<'a, Out1>, out_2: Output<'a, Out2>, enable: Output<'a, En>) -> PeristalticPump<'a, Out1, Out2, En> {
        
        return PeristalticPump {
            out_1,
            out_2,
            enable,
            state: PumpState::Idle
        }
    }

    pub fn start_in(&mut self) {

        match self.state {
            PumpState::Idle => {
                self.state = PumpState::PumpingIn;
                self.enable.set_high();
                self.out_1.set_high();
                self.out_2.set_low();
            },
            _ => panic!("Cannot start pump; already running")
        }

    }


    pub fn start_out(&mut self) {
        match self.state {
            PumpState::Idle => {
                self.state = PumpState::PumpingOut;
                self.enable.set_high();
                self.out_1.set_low();
                self.out_2.set_high();
            },
            _ => panic!("Cannot start pump; already running")
        }
    }


    pub fn stop(&mut self) {

        match self.state {
            PumpState::Idle => panic!("Cannot stop pump; already stopped"),
            _ => {
                self.state = PumpState::Idle;
                self.enable.set_low();
                self.out_1.set_low();
                self.out_2.set_high();
            }
        }
    }
}