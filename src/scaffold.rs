use std::collections::HashSet;

use crate::prelude::*;

const PREVIEW_WIDTH: usize = 6;
const PREVIEW_HEIGHT: usize = 5;

pub struct Scaffold {
    pub screen_width: usize,
    pub screen_height: usize,
    pub canvas_width: usize,
    pub canvas_height: usize,
}

impl Scaffold {
    pub fn spawn_point(&self) -> Point {
        Point::new(self.hpad() + self.canvas_width / 2, self.vpad() + 2)
    }

    pub fn preview_origin(&self) -> Point {
        Point::new(
            self.hpad() + self.canvas_width + 2 + (PREVIEW_WIDTH - 1) / 2,
            self.vpad() + 1 + (PREVIEW_HEIGHT - 1) / 2,
        )
    }

    pub fn border_entities(&self) -> Vec<(Position, PixelRender)> {
        let unique_border_points: HashSet<_> = rect_outer_border_points(&self.canvas_rect())
            .chain(rect_outer_border_points(&self.preview_rect()))
            .collect();

        unique_border_points
            .iter()
            .map(|&p| {
                (
                    Position(p),
                    PixelRender {
                        colors: ColorPair::new(LIGHT_SLATE, BLACK),
                        glyph: to_cp437(border_glyph(&p, &unique_border_points)),
                    },
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

    pub fn score_rects(&self) -> impl Iterator<Item = Rect> {
        let x = TEXT_SCALE * (self.hpad() + 2 + self.canvas_width) + 1;
        let start_y = TEXT_SCALE * (self.vpad() + 3 + PREVIEW_HEIGHT);
        let width = TEXT_SCALE * self.screen_width - x - 1;
        (0..).map(move |i| Rect::with_size(x, start_y + (4 * i), width, 2))
    }
}
