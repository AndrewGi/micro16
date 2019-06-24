#[derive(Clone, Copy)]
pub enum Mode {
    IntervalTimer,
    PulseGeneration,
    EventCounter,
    PulseWidthMeasurement
}
pub struct Counter {
    latch: u16,
    counter: u16,
    mode: Mode
}
impl Counter {
    pub fn dec(&mut self) {
        if self.counter == 0 {
            //UNDERFLOW
            self.counter = self.latch;
        } else {
            self.counter -= 1;
        }
    }
    pub fn latch(&self) -> u16 {
        self.latch
    }
    pub fn mode(&self) -> Mode {
        self.mode
    }
    pub fn counter(&self) -> u16 {
        self.counter
    }
}