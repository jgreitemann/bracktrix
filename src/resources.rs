use crate::prelude::*;

pub struct Screen(pub Rect);

pub struct Difficulty {
    pub gravity_tick_speed: usize,
}

pub struct BlockEntityStore {
    pub active: Vec<Entity>,
    pub preview: Vec<Entity>,
}

impl BlockEntityStore {
    pub fn new() -> Self {
        BlockEntityStore {
            active: Vec::new(),
            preview: Vec::new(),
        }
    }
}

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
