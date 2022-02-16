use crate::prelude::*;

pub struct Position(pub Point);

pub struct Pivot(pub Point);

pub struct PixelRender {
    pub colors: ColorPair,
    pub glyph: FontCharType,
}

pub struct NewViewport(pub Rect);
