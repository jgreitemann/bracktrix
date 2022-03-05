use crate::input::*;

pub struct KeyboardInputSource;

impl InputSource for KeyboardInputSource {
    fn read(&mut self) -> RawInputSignal {
        let input = INPUT.lock();

        let key = [
            VirtualKeyCode::Left,
            VirtualKeyCode::Right,
            VirtualKeyCode::Up,
            VirtualKeyCode::Down,
            VirtualKeyCode::RControl,
            VirtualKeyCode::Space,
            VirtualKeyCode::Return,
            VirtualKeyCode::Escape,
        ]
        .into_iter()
        .find(|k| input.is_key_pressed(*k));

        RawInputSignal::from_keyboard(key)
    }
}
