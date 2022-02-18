use std::collections::HashSet;

use bracket_lib::prelude::*;

use crate::components::*;
use crate::graphics::*;
use crate::viewport::*;

const PREVIEW_WIDTH: usize = 6;
const PREVIEW_HEIGHT: usize = 5;

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

    pub fn preview_origin(&self) -> Point {
        Point::new((PREVIEW_WIDTH - 1) / 2, (PREVIEW_HEIGHT - 1) / 2)
    }

    pub fn border_entities(&self) -> Vec<(Position, PixelRender, NewViewport)> {
        let unique_border_points: HashSet<_> = rect_outer_border_points(&self.canvas_rect())
            .chain(rect_outer_border_points(&self.preview_rect()))
            .collect();

        unique_border_points
            .iter()
            .map(|&p| {
                (
                    Position(p),
                    PixelRender {
                        colors: ColorPair::new(BLACK, WHITE),
                        glyph: to_cp437(border_glyph(&p, &unique_border_points)),
                    },
                    NewViewport(self.screen_rect()),
                )
            })
            .collect()
    }

    fn hpad(&self) -> usize {
        (self.screen_width - self.canvas_width - PREVIEW_WIDTH - 3) / 2
    }

    fn vpad(&self) -> usize {
        (self.screen_height - self.canvas_height - 2) / 2
    }

    pub fn screen_rect(&self) -> Rect {
        Rect::with_size(0, 0, self.screen_width, self.screen_height)
    }

    pub fn canvas_rect(&self) -> Rect {
        Rect::with_size(
            self.hpad() + 1,
            self.vpad() + 1,
            self.canvas_width,
            self.canvas_height,
        )
    }

    pub fn preview_rect(&self) -> Rect {
        Rect::with_size(
            self.hpad() + self.canvas_width + 2,
            self.vpad() + 1,
            PREVIEW_WIDTH,
            PREVIEW_HEIGHT,
        )
    }
}
