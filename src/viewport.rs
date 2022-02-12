use crate::Color;
use bracket_lib::prelude::*;

pub struct Viewport<'a> {
    rect: Rect,
    ctx: &'a mut BTerm,
}

impl<'a> Viewport<'a> {
    pub fn new(rect: Rect, ctx: &'a mut BTerm) -> Self {
        Self { rect, ctx }
    }

    pub fn set(&mut self, p: &Point, fg: Color, bg: Color, glyph: char) {
        let screen_point = *p + Point::new(self.rect.x1, self.rect.y1);
        if self.rect.point_in_rect(screen_point) {
            self.ctx
                .set(screen_point.x, screen_point.y, fg, bg, to_cp437(glyph));
        }
    }
}
