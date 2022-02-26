use crate::prelude::*;

#[system]
pub fn gamepad_input(
    #[state] active_gamepad: &mut Option<GamepadId>,
    #[resource] gilrs: &mut Gilrs,
    #[resource] gamepad_key: &mut Option<GamepadKey>,
) {
    use Button::*;
    let key = gilrs
        .next_event()
        .and_then(|Event { id, event, .. }| {
            *active_gamepad = Some(id);
            match event {
                EventType::ButtonPressed(South, _) => Some(VirtualKeyCode::Down),
                EventType::ButtonPressed(East, _) => Some(VirtualKeyCode::Up),
                _ => None,
            }
        })
        .or_else(|| {
            let gamepad = active_gamepad.map(|id| gilrs.gamepad(id));
            gamepad.and_then(|gamepad| {
                gamepad
                    .button_data(DPadRight)
                    .and_then(|bdata| {
                        let value = bdata.value();
                        if value == 0.0f32 {
                            Some(VirtualKeyCode::Left)
                        } else if value == 1.0f32 {
                            Some(VirtualKeyCode::Right)
                        } else {
                            None
                        }
                    })
                    .or_else(|| {
                        gamepad.button_data(DPadUp).and_then(|bdata| {
                            let value = bdata.value();
                            if value == 0.0f32 {
                                Some(VirtualKeyCode::Space)
                            } else {
                                None
                            }
                        })
                    })
            })
        });

    *gamepad_key = key.map(|key| match std::mem::replace(gamepad_key, None) {
        Some(gamepad_key) if gamepad_key.matches(key) => gamepad_key,
        None | Some(_) => GamepadKey::new(key),
    });
}
