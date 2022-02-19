use crate::prelude::*;

pub struct MenuItem {
    pub rank: usize,
}

pub struct ScoreboardItem {
    pub rect: Rect,
}

pub struct DisplayText(pub String);

#[derive(Copy, Clone)]
pub enum Metric {
    LinesCleared,
    NumberOfBracktrixes,
    TimeElapsed,
}
