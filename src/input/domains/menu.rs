use crate::prelude::*;
use std::time::Duration;

use gilrs::Button;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MenuInput {
    NavigateUp,
    NavigateDown,
    Choose,
}

impl DomainInput for MenuInput {
    fn from_raw(raw: RawInput) -> Option<Self> {
        use MenuInput::*;
        match raw {
            RawInput::KeyboardInput(key) => match key {
                VirtualKeyCode::Up => Some(NavigateUp),
                VirtualKeyCode::Down => Some(NavigateDown),
                VirtualKeyCode::Return => Some(Choose),
                _ => None,
            },
            RawInput::GamepadInput(button) => match button {
                Button::DPadUp => Some(NavigateUp),
                Button::DPadDown => Some(NavigateDown),
                Button::East => Some(Choose),
                _ => None,
            },
            RawInput::None => None,
        }
    }

    fn autorepeat_intervals(&self) -> [Duration; 2] {
        [150, 35].map(|ms| Duration::from_millis(ms))
    }
}

pub type MenuInputState = DomainInputState<MenuInput>;
