use crate::prelude::*;

#[derive(Copy, Clone, Default)]
pub struct Active;

#[derive(Copy, Clone, Default)]
pub struct Preview;

#[derive(Copy, Clone, Default)]
pub struct Settled;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position(pub Point);

#[derive(Copy, Clone)]
pub struct Pivot(pub Point);

#[derive(Copy, Clone)]
pub struct PixelRender {
    pub colors: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Default)]
pub struct Flagged {
    pub animation_frame: usize,
}
