use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Menu {
    Main,
    GameOver,
    Statistics,
    Pause,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    Play,
    Menu(Menu),
    Quitting,
}

pub struct Screen(pub Rect);

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
