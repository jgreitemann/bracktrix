mod gamepad;

pub use gamepad::*;

use crate::prelude::*;
use gilrs::Button;

#[derive(Copy, Clone, Debug)]
pub enum RawInput {
    KeyboardInput(VirtualKeyCode),
    GamepadInput(Button),
    None,
}

#[derive(Debug)]
pub struct RawInputSignal {
    input: RawInput,
    time: std::time::Instant,
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameInput {
    ShiftLeft,
    ShiftRight,
    RotateCW,
    RotateCCW,
    SoftDrop,
    HardDrop,
}

impl GameInput {
    pub fn from_raw(raw: RawInput) -> Option<Self> {
        use GameInput::*;
        match raw {
            RawInput::KeyboardInput(key) => match key {
                VirtualKeyCode::Left => Some(ShiftLeft),
                VirtualKeyCode::Right => Some(ShiftRight),
                VirtualKeyCode::Up => Some(RotateCW),
                VirtualKeyCode::RControl => Some(RotateCCW),
                VirtualKeyCode::Down => Some(SoftDrop),
                VirtualKeyCode::Space | VirtualKeyCode::Return => Some(HardDrop),
                _ => None,
            },
            RawInput::GamepadInput(button) => match button {
                Button::DPadLeft => Some(ShiftLeft),
                Button::DPadRight => Some(ShiftRight),
                Button::East => Some(RotateCW),
                Button::South => Some(RotateCCW),
                Button::DPadDown => Some(SoftDrop),
                Button::DPadUp => Some(HardDrop),
                _ => None,
            },
            RawInput::None => None,
        }
    }

    pub fn autorepeat_intervals(&self) -> [std::time::Duration; 2] {
        use std::time::Duration;
        use GameInput::*;
        match self {
            ShiftLeft | ShiftRight => [150, 35].map(|ms| Duration::from_millis(ms)),
            SoftDrop => [Duration::ZERO; 2],
            HardDrop | RotateCW | RotateCCW => [Duration::MAX; 2],
        }
    }
}

#[derive(Debug)]
pub struct GameInputState {
    input: Option<GameInput>,
    repetition: usize,
    time: std::time::Instant,
    consumed: bool,
}

impl GameInputState {
    pub fn new() -> Self {
        Self {
            input: None,
            repetition: 0,
            time: std::time::Instant::now(),
            consumed: true,
        }
    }

    pub fn process(&mut self, raw: &RawInputSignal) {
        let input = GameInput::from_raw(raw.input);
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

    pub fn get(&mut self) -> Option<GameInput> {
        if !std::mem::replace(&mut self.consumed, true) {
            self.input
        } else {
            None
        }
    }

    fn autorepeat_interval(&self) -> std::time::Duration {
        self.input.map_or(std::time::Duration::MAX, |i| {
            i.autorepeat_intervals()[self.repetition.min(1)]
        })
    }
}
