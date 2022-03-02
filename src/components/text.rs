use crate::prelude::*;

pub struct MenuItem {
    pub rank: usize,
}

pub struct ScoreboardItem {
    pub rect: Rect,
}

pub struct DisplayText(pub String);

#[derive(Copy, Clone)]
pub enum ScoreStyle {
    Text,
    ProgressBar,
}

pub struct Score {
    pub metric: Metric,
    pub style: ScoreStyle,
}

pub struct Selectable;

#[derive(Copy, Clone)]
pub struct Focus {
    pub current: usize,
    count: usize,
}

impl Focus {
    pub fn new(count: usize) -> Self {
        Self { current: 0, count }
    }

    pub fn up(&mut self) {
        self.current = (self.current + self.count - 1) % self.count;
    }

    pub fn down(&mut self) {
        self.current = (self.current + 1) % self.count;
    }
}
