use crate::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameMode {
    Play,
    Menu,
}

pub struct Screen(pub Rect);

pub struct Difficulty {
    pub gravity_tick_speed: usize,
    pub hard_drop: bool,
}

pub struct Scoring {
    score: usize,
    lines_cleared: usize,
    bracktrixes: usize,
    start_of_game: std::time::Instant,
}

impl Default for Scoring {
    fn default() -> Self {
        Scoring {
            score: 0,
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
            Score => self.score.to_string(),
            LinesCleared => self.lines_cleared.to_string(),
            NumberOfBracktrixes => self.bracktrixes.to_string(),
            TimeElapsed => {
                let secs = self.start_of_game.elapsed().as_secs();
                format!("{}:{:02}", secs / 60, secs % 60)
            }
        }
    }

    pub fn score_lines_cleared(&mut self, num_lines: usize) {
        self.score += match num_lines {
            0 => 0,
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800,
            _ => panic!(),
        };

        self.lines_cleared += num_lines;

        if num_lines == 4 {
            self.bracktrixes += 1;
        }
    }

    pub fn score_hard_drop(&mut self) {
        self.score += 2;
    }
}

pub struct BlockSpawnPoints {
    pub active_block_spawn: Point,
    pub preview_block_spawn: Point,
}
