use crate::drivers::drv8871::DRV8871;

use embassy_stm32::gpio::{Pin, Output, Level, Speed};

enum PumpState {
    Idle,
    PumpingIn,
    PumpingOut
}

pub struct PeristalticPump<'a, MCPin1: Pin, MCPin2: Pin> {
    mc: DRV8871<'a, MCPin1, MCPin2>,
    state: PumpState
}

impl<'a, MCPin1: Pin, MCPin2: Pin> PeristalticPump<'a, MCPin1, MCPin2>{

    pub fn new(mc_pin1: MCPin1, mc_pin2: MCPin2) -> PeristalticPump<'a, MCPin1, MCPin2> {
        let mc_out1 = Output::new(mc_pin1, Level::Low, Speed::High);
        let mc_out2 = Output::new(mc_pin2, Level::Low, Speed::High);
        return PeristalticPump {
            mc: DRV8871::new(mc_out1, mc_out2),
            state: PumpState::Idle
        }
    }

    pub fn start_in(&mut self) {
        self.state = PumpState::PumpingIn;
        self.mc.reverse();
    }


    pub fn start_out(&mut self) {
        self.state = PumpState::PumpingOut;
        self.mc.forward();

    }

    pub fn stop(&mut self) {
        self.state = PumpState::Idle;
        self.mc.coast();
    }

    pub fn cycle(&mut self) {
        match self.state {
            PumpState::Idle => self.start_in(),
            PumpState::PumpingIn => self.start_out(),
            PumpState::PumpingOut => self.stop(),
        }
    }
}
