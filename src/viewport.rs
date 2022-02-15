use bracket_lib::prelude::*;

use crate::graphics::*;

pub struct Viewport<'a> {
    rect: Rect,
    ctx: &'a mut BTerm,
}

pub fn to_screen(position: &Point, rect: &Rect) -> Option<Point> {
    let screen_point = *position + Point::new(rect.x1, rect.y1);
    if rect.point_in_rect(screen_point) {
        Some(screen_point)
    } else {
        None
    }
}

impl<'a> Viewport<'a> {
    pub fn new(rect: Rect, ctx: &'a mut BTerm) -> Self {
        Self { rect, ctx }
    }

    pub fn draw(&mut self, pix: &Pixel) {
        if let Some(Point { x, y }) = to_screen(&pix.position, &self.rect) {
            self.ctx.set(x, y, pix.color, BLACK, to_cp437(pix.glyph));
        }
    }
}
