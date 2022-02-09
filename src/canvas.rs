use crate::prelude::*;

pub struct Pixel {
    pub position: Point,
    pub color: Color,
}

pub struct Canvas {
    pixels: Vec<Option<Color>>,
}

fn point_to_index(p: &Point) -> Option<usize> {
    if (0..SCREEN_WIDTH).contains(&p.x) && (0..SCREEN_HEIGHT).contains(&p.y) {
        Some((p.y * SCREEN_WIDTH + p.x) as usize)
    } else {
        None
    }
}

fn index_to_point(idx: usize) -> Point {
    Point::new(idx % SCREEN_WIDTH as usize, idx / SCREEN_WIDTH as usize)
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            pixels: vec![None; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
        }
    }

    pub fn is_empty(&self, p: &Point) -> bool {
        point_to_index(p).map(|idx| self.pixels[idx]) == Some(None)
    }

    pub fn bake(&mut self, new_pixels: impl Iterator<Item = Pixel>) {
        for (idx, color) in new_pixels.filter_map(|Pixel { position, color }| {
            point_to_index(&position).map(|idx| (idx, color))
        }) {
            self.pixels[idx] = Some(color);
        }
    }

    pub fn clear_full_rows(&mut self) {
        for row in (0..SCREEN_HEIGHT).rev() {
            let idx = point_to_index(&Point::new(0, row)).unwrap();
            if self.pixels[idx..idx + SCREEN_WIDTH as usize]
                .iter()
                .all(|opt| opt.is_some())
            {
                self.pixels.copy_within(0..idx, SCREEN_WIDTH as usize);
            }
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for (Point { x, y }, color) in self
            .pixels
            .iter()
            .enumerate()
            .filter_map(|(i, o)| o.map(|c| (index_to_point(i), c)))
        {
            ctx.set(x, y, color, BLACK, to_cp437('█'));
        }
    }
}
