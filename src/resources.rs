use crate::prelude::*;

pub struct Screen(pub Rect);

pub struct Difficulty {
    pub gravity_tick_speed: usize,
}

pub struct BlockEntityStore {
    pub active: Option<[Entity; 4]>,
    pub preview: Option<[Entity; 4]>,
}

impl BlockEntityStore {
    pub fn new() -> Self {
        BlockEntityStore {
            active: None,
            preview: None,
        }
    }
}

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
