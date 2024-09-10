use embassy_stm32::gpio::Output;

pub enum DRV8871State {
    Coast,
    Forward,
    Reverse,
    Brake,
}

pub struct DRV8871<'a> {
    out_1: Output<'a>,
    out_2: Output<'a>,
    pub state: DRV8871State,
}

impl<'a> DRV8871<'a> {
    pub fn new(out_1: Output<'a>, out_2: Output<'a>) -> DRV8871<'a> {
        return DRV8871 {
            out_1,
            out_2,
            state: DRV8871State::Coast,
        };
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
