use embassy_stm32::gpio::{OutputType, Output, Level, Speed};


enum PumpState {
    Idle,
    PumpingIn,
    PumpingOut
}

struct PeristalticPump {
    out_1: Output,
    out_2: Output,
    enabe: Output,
    state: PumpState
}

// Pump In: In 1, Out 0
// Pump Out: In 0, Out 1

impl PeristalticPump {
    fn start_pump_in(&mut self) {

        assert!(state == PumpState::Idle, "Cannot start pump; already running");

        self.enable.set_high();
        self.out_1.set_high();
        self.out_2.set_low();
    }

    
    fn start_pump_out(&mut self) {

        assert!(state == PumpState::Idle, "Cannot start pump; already running");

        self.enable.set_high();
        self.out_1.set_low();
        self.out_2.set_high();
    }


    fn stop_pump_out(&mut self) {

        assert!(state == PumpState::Idle, "Cannot stop pump; not running");

        self.out_1.set_low();
        self.out_2.set_low();
        self.enable.set_low();
    }
}