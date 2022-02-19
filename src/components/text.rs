pub struct MenuItem {
    pub rank: usize,
}

pub struct DisplayText(pub String);

#[derive(Copy, Clone)]
pub enum Metric {
    LinesCleared,
}
