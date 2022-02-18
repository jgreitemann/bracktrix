use crate::prelude::*;

pub struct Active;

pub struct Preview;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position(pub Point);

#[derive(Copy, Clone)]
pub struct Pivot(pub Point);

pub struct PixelRender {
    pub colors: ColorPair,
    pub glyph: FontCharType,
}
