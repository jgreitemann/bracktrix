mod domains;
mod sources;

pub use domains::*;
pub use sources::*;

use crate::prelude::*;
use gilrs::Button;

#[derive(Copy, Clone, Debug)]
pub enum RawInput {
    None,
    KeyboardInput(VirtualKeyCode),
    GamepadInput(Button),
}

#[derive(Debug)]
pub struct RawInputSignal {
    input: RawInput,
    time: std::time::Instant,
}

impl Default for RawInputSignal {
    fn default() -> Self {
        Self {
            input: RawInput::None,
            time: std::time::Instant::now(),
        }
    }
}

impl RawInputSignal {
    pub fn from_keyboard(key: Option<VirtualKeyCode>) -> Self {
        Self {
            input: key.map_or(RawInput::None, |k| RawInput::KeyboardInput(k)),
            time: std::time::Instant::now(),
        }
    }

    pub fn from_gamepad(button: Option<Button>) -> Self {
        Self {
            input: button.map_or(RawInput::None, |b| RawInput::GamepadInput(b)),
            time: std::time::Instant::now(),
        }
    }

    pub fn or(self, other: RawInputSignal) -> Self {
        if let RawInput::None = self.input {
            other
        } else {
            self
        }
    }
}

pub trait InputSource {
    fn read(&mut self) -> RawInputSignal;
}

pub trait DomainInput: Sized + Clone + PartialEq {
    fn from_raw(raw: RawInput) -> Option<Self>;
    fn autorepeat_intervals(&self) -> [std::time::Duration; 2];
}

#[derive(Debug)]
pub struct DomainInputState<D: DomainInput> {
    input: Option<D>,
    repetition: usize,
    time: std::time::Instant,
    consumed: bool,
}

impl<D: DomainInput> DomainInputState<D> {
    pub fn new() -> Self {
        Self {
            input: None,
            repetition: 0,
            time: std::time::Instant::now(),
            consumed: true,
        }
    }

    pub fn process(&mut self, raw: &RawInputSignal) {
        let input = DomainInput::from_raw(raw.input);
        if input == self.input {
            if raw.time.duration_since(self.time) > self.autorepeat_interval() {
                self.repetition += 1;
            } else {
                return;
            }
        } else {
            self.input = input;
            self.repetition = 0;
        }
        self.time = raw.time;
        self.consumed = false;
    }

    pub fn get(&mut self) -> Option<D> {
        if !std::mem::replace(&mut self.consumed, true) {
            self.input.clone()
        } else {
            None
        }
    }

    fn autorepeat_interval(&self) -> std::time::Duration {
        self.input.as_ref().map_or(std::time::Duration::MAX, |i| {
            i.autorepeat_intervals()[self.repetition.min(1)]
        })
    }
}
