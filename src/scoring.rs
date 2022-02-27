#[derive(Copy, Clone)]
pub enum Metric {
    Level,
    LevelUpFraction,
    Score,
    LinesCleared,
    NumberOfBracktrixes,
    TimeElapsed,
    BlocksPlaced,
}

pub struct Scoring {
    level: usize,
    goal_points: usize,
    score: usize,
    lines_cleared: usize,
    bracktrixes: usize,
    start_of_game: std::time::Instant,
    blocks_placed: usize,
    hard_dropping: bool,
}

impl Default for Scoring {
    fn default() -> Self {
        Scoring {
            level: 1,
            goal_points: 0,
            score: 0,
            lines_cleared: 0,
            bracktrixes: 0,
            start_of_game: std::time::Instant::now(),
            blocks_placed: 0,
            hard_dropping: false,
        }
    }
}

impl Scoring {
    pub fn get_text(&self, metric: Metric) -> String {
        use Metric::*;
        match metric {
            Level => self.level.to_string(),
            LevelUpFraction => format!("{:.2}%", 100. * self.level_up_fraction()),
            Score => self.score.to_string(),
            LinesCleared => self.lines_cleared.to_string(),
            NumberOfBracktrixes => self.bracktrixes.to_string(),
            TimeElapsed => {
                let secs = self.start_of_game.elapsed().as_secs();
                format!("{}:{:02}", secs / 60, secs % 60)
            }
            BlocksPlaced => self.blocks_placed.to_string(),
        }
    }

    pub fn get_fraction(&self, metric: Metric) -> f32 {
        use Metric::*;
        match metric {
            LevelUpFraction => self.level_up_fraction(),
            _ => 0f32,
        }
    }

    pub fn hard_drop(&mut self) {
        self.hard_dropping = true;
    }

    pub fn gravity_tick_speed(&self) -> usize {
        if self.hard_dropping {
            1
        } else {
            15 - self.level.min(14)
        }
    }

    fn needed_goal_points(&self) -> usize {
        5 * self.level
    }

    pub fn level_up_fraction(&self) -> f32 {
        self.goal_points as f32 / self.needed_goal_points() as f32
    }

    pub fn score_lines_cleared(&mut self, num_lines: usize) {
        let awarded_goal_points = match num_lines {
            0 => 0,
            1 => 1,
            2 => 3,
            3 => 5,
            4 => 8,
            _ => panic!(),
        };
        self.score += awarded_goal_points * 100 * self.level;
        self.goal_points += awarded_goal_points;

        if self.goal_points >= self.needed_goal_points() {
            self.goal_points = 0;
            self.level += 1;
        }

        self.lines_cleared += num_lines;

        if num_lines == 4 {
            self.bracktrixes += 1;
        }
    }

    pub fn score_block_dropped(&mut self) {
        if self.hard_dropping {
            self.score += 2;
        }
    }

    pub fn score_block_placed(&mut self) {
        self.blocks_placed += 1;
        self.hard_dropping = false;
    }
}
