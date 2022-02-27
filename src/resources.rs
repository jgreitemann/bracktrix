use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    Play,
    Menu,
}

pub struct Screen(pub Rect);

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}

pub struct GamepadKey {
    key: VirtualKeyCode,
    repetition: usize,
    time: std::time::Instant,
}

impl GamepadKey {
    #[cfg(feature = "gamepad")]
    pub fn new(key: VirtualKeyCode) -> Self {
        GamepadKey {
            key,
            repetition: 0,
            time: std::time::Instant::now(),
        }
    }

    #[cfg(feature = "gamepad")]
    pub fn matches(&self, other: VirtualKeyCode) -> bool {
        self.key == other
    }

    pub fn key(&mut self) -> Option<VirtualKeyCode> {
        let repeat_time = match self.key {
            VirtualKeyCode::Left | VirtualKeyCode::Right => match self.repetition {
                0 => std::time::Duration::from_millis(0),
                1 => std::time::Duration::from_millis(150),
                _ => std::time::Duration::from_millis(35),
            },
            _ => match self.repetition {
                0 => std::time::Duration::from_millis(0),
                _ => std::time::Duration::from_secs(3600),
            },
        };
        if self.time.elapsed() > repeat_time {
            self.repetition += 1;
            self.time = std::time::Instant::now();
            Some(self.key)
        } else {
            None
        }
    }
}
