use crate::viewport::*;
use bracket_lib::prelude::*;

const PREVIEW_WIDTH: usize = 4;
const PREVIEW_HEIGHT: usize = 3;

pub struct Scaffold {
    pub screen_width: usize,
    pub screen_height: usize,
    pub canvas_width: usize,
    pub canvas_height: usize,
}

impl Scaffold {
    pub fn canvas_viewport<'a>(&self, ctx: &'a mut BTerm) -> Viewport<'a> {
        Viewport::new(
            Rect::with_size(
                self.hpad() + 1,
                self.vpad() + 1,
                self.canvas_width,
                self.canvas_height,
            ),
            ctx,
        )
    }

    pub fn preview_viewport<'a>(&self, ctx: &'a mut BTerm) -> Viewport<'a> {
        Viewport::new(
            Rect::with_size(
                self.hpad() + self.canvas_width + 2,
                self.vpad() + 1,
                PREVIEW_WIDTH,
                PREVIEW_HEIGHT,
            ),
            ctx,
        )
    }

    fn hpad(&self) -> usize {
        (self.screen_width - self.canvas_width - PREVIEW_WIDTH - 3) / 2
    }

    fn vpad(&self) -> usize {
        (self.screen_height - self.canvas_height - 2) / 2
    }
}
