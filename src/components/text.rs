use crate::prelude::*;

pub struct MenuItem {
    pub menu: Menu,
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

pub struct Focus;

#[derive(Clone)]
pub enum Action {
    NotImplemented,
    StartGame,
    ResumeGame,
    GoToMenu(Menu),
    Quit,
}

pub struct Actionable(pub Action);
