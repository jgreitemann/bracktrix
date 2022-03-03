use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Menu {
    Main,
    GameOver,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    Play,
    Menu(Menu),
}

pub struct Screen(pub Rect);

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
