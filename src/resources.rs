use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    Play,
    Menu,
}

pub struct Screen(pub Rect);

pub struct Difficulty {
    pub gravity_tick_speed: usize,
    pub quick_drop: bool,
}

#[derive(Default)]
pub struct Scoring {
    pub lines_cleared: usize,
}

impl Scoring {
    pub fn get(&self, metric: Metric) -> usize {
        use Metric::*;
        match metric {
            LinesCleared => self.lines_cleared,
        }
    }
}

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
