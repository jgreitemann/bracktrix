use std::collections::HashSet;

use bracket_lib::prelude::*;

use crate::geometry::*;
use crate::viewport::*;

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
        Viewport::new(self.canvas_rect(), ctx)
    }

    pub fn preview_viewport<'a>(&self, ctx: &'a mut BTerm) -> Viewport<'a> {
        Viewport::new(self.preview_rect(), ctx)
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let border_points: HashSet<_> = rect_outer_border_points(&self.canvas_rect())
            .chain(rect_outer_border_points(&self.preview_rect()))
            .collect();
        for Point { x, y } in border_points {
            ctx.set(x, y, WHITE, BLACK, to_cp437('═'));
        }
    }

    fn hpad(&self) -> usize {
        (self.screen_width - self.canvas_width - PREVIEW_WIDTH - 3) / 2
    }

    fn vpad(&self) -> usize {
        (self.screen_height - self.canvas_height - 2) / 2
    }

    fn canvas_rect(&self) -> Rect {
        Rect::with_size(
            self.hpad() + 1,
            self.vpad() + 1,
            self.canvas_width,
            self.canvas_height,
        )
    }

    fn preview_rect(&self) -> Rect {
        Rect::with_size(
            self.hpad() + self.canvas_width + 2,
            self.vpad() + 1,
            PREVIEW_WIDTH,
            PREVIEW_HEIGHT,
        )
    }
}
