use crate::prelude::*;

#[system]
pub fn gamepad_input(#[resource] gilrs: &mut Gilrs, #[resource] gamepad_key: &mut GamepadKey) {
    if let Some(Event { event, .. }) = gilrs.next_event() {
        use Button::*;
        match event {
            EventType::ButtonPressed(South, _) => {
                *gamepad_key = GamepadKey(Some(VirtualKeyCode::Down));
            }
            EventType::ButtonPressed(East, _) => {
                *gamepad_key = GamepadKey(Some(VirtualKeyCode::Up));
            }
            EventType::ButtonChanged(DPadRight, amount, _) => {
                if amount == 0.0 {
                    *gamepad_key = GamepadKey(Some(VirtualKeyCode::Left));
                } else if amount == 1.0 {
                    *gamepad_key = GamepadKey(Some(VirtualKeyCode::Right));
                }
            }
            EventType::ButtonChanged(DPadUp, amount, _) => {
                if amount == 0.0 {
                    *gamepad_key = GamepadKey(Some(VirtualKeyCode::Space));
                }
            }
            _ => {}
        }
    }
}
