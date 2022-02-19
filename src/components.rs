use crate::prelude::*;

pub struct Active;

pub struct Preview;

pub struct Settled;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position(pub Point);

#[derive(Copy, Clone)]
pub struct Pivot(pub Point);

pub struct PixelRender {
    pub colors: ColorPair,
    pub glyph: FontCharType,
}

pub struct Flagged {
    pub frames_till_death: usize,
}

impl Flagged {
    pub fn new() -> Self {
        Flagged {
            frames_till_death: 100,
        }
    }
}
