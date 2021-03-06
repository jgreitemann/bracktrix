use crate::prelude::*;
use gilrs::Button;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameInput {
    ShiftLeft,
    ShiftRight,
    RotateCW,
    RotateCCW,
    SoftDrop,
    HardDrop,
    PauseGame,
}

impl DomainInput for GameInput {
    fn from_raw(raw: RawInput) -> Option<Self> {
        use GameInput::*;
        match raw {
            RawInput::KeyboardInput(key) => match key {
                VirtualKeyCode::Left => Some(ShiftLeft),
                VirtualKeyCode::Right => Some(ShiftRight),
                VirtualKeyCode::Up => Some(RotateCW),
                VirtualKeyCode::RControl => Some(RotateCCW),
                VirtualKeyCode::Down => Some(SoftDrop),
                VirtualKeyCode::Space => Some(HardDrop),
                VirtualKeyCode::Escape => Some(PauseGame),
                _ => None,
            },
            RawInput::GamepadInput(button) => match button {
                Button::DPadLeft => Some(ShiftLeft),
                Button::DPadRight => Some(ShiftRight),
                Button::East => Some(RotateCW),
                Button::South => Some(RotateCCW),
                Button::DPadDown => Some(SoftDrop),
                Button::DPadUp => Some(HardDrop),
                Button::Start => Some(PauseGame),
                _ => None,
            },
            RawInput::None => None,
        }
    }

    fn autorepeat_intervals(&self) -> [std::time::Duration; 2] {
        use std::time::Duration;
        use GameInput::*;
        match self {
            ShiftLeft | ShiftRight => [150, 35].map(|ms| Duration::from_millis(ms)),
            SoftDrop => [Duration::ZERO; 2],
            HardDrop | RotateCW | RotateCCW | PauseGame => [Duration::MAX; 2],
        }
    }
}

pub type GameInputState = DomainInputState<GameInput>;
