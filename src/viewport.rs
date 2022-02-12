use bracket_lib::prelude::*;

use crate::graphics::*;

pub struct Viewport<'a> {
    rect: Rect,
    ctx: &'a mut BTerm,
}

impl<'a> Viewport<'a> {
    pub fn new(rect: Rect, ctx: &'a mut BTerm) -> Self {
        Self { rect, ctx }
    }

    pub fn draw(&mut self, pix: &Pixel) {
        let screen_point = pix.position + Point::new(self.rect.x1, self.rect.y1);
        if self.rect.point_in_rect(screen_point) {
            self.ctx.set(
                screen_point.x,
                screen_point.y,
                pix.color,
                BLACK,
                to_cp437(pix.glyph),
            );
        }
    }
}
