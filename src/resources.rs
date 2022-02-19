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

pub struct Scoring {
    pub lines_cleared: usize,
    pub bracktrixes: usize,
    pub start_of_game: std::time::Instant,
}

impl Default for Scoring {
    fn default() -> Self {
        Scoring {
            lines_cleared: 0,
            bracktrixes: 0,
            start_of_game: std::time::Instant::now(),
        }
    }
}

impl Scoring {
    pub fn get(&self, metric: Metric) -> String {
        use Metric::*;
        match metric {
            LinesCleared => self.lines_cleared.to_string(),
            NumberOfBracktrixes => self.bracktrixes.to_string(),
            TimeElapsed => {
                let secs = self.start_of_game.elapsed().as_secs();
                format!("{}:{:02}", secs / 60, secs % 60)
            }
        }
    }
}

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
