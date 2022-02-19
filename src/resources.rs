use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    Play,
    Menu,
}

pub struct Screen(pub Rect);

pub struct Difficulty {
    pub gravity_tick_speed: usize,
}

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
