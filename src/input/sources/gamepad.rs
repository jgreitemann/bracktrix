use crate::input::*;
use gilrs::*;

pub struct GamepadInputSource {
    gilrs: Gilrs,
    active_gamepad: Option<GamepadId>,
}

impl GamepadInputSource {
    pub fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
            active_gamepad: None,
        }
    }
}

impl InputSource for GamepadInputSource {
    fn read(&mut self) -> RawInputSignal {
        use Button::*;

        if let Some(Event { id, .. }) = self.gilrs.next_event() {
            self.active_gamepad = Some(id);
        }

        let gamepad = self.active_gamepad.map(|id| self.gilrs.gamepad(id));

        let button = gamepad.and_then(|gamepad| {
            if gamepad.is_pressed(South) {
                Some(South)
            } else if gamepad.is_pressed(East) {
                Some(East)
            } else {
                gamepad
                    .button_data(DPadRight)
                    .and_then(|bdata| {
                        let value = bdata.value();
                        if value == 0.0f32 {
                            Some(DPadLeft)
                        } else if value == 1.0f32 {
                            Some(DPadRight)
                        } else {
                            None
                        }
                    })
                    .or_else(|| {
                        gamepad.button_data(DPadUp).and_then(|bdata| {
                            let value = bdata.value();
                            if value == 0.0f32 {
                                Some(DPadUp)
                            } else if value == 1.0f32 {
                                Some(DPadDown)
                            } else {
                                None
                            }
                        })
                    })
            }
        });

        RawInputSignal::from_gamepad(button)
    }
}
