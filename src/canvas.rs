use crate::prelude::*;

pub struct Pixel {
    pub position: Point,
    pub color: Color,
}

struct RowMajorMapping {
    width: usize,
    height: usize,
}

impl RowMajorMapping {
    fn point_to_index(&self, p: &Point) -> Option<usize> {
        let (x, y) = (p.x as usize, p.y as usize);
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn index_to_point(&self, idx: usize) -> Point {
        Point::new(idx % self.width, idx / self.width)
    }
}

pub struct Canvas {
    mapping: RowMajorMapping,
    pixels: Vec<Option<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            mapping: RowMajorMapping { width, height },
            pixels: vec![None; width * height],
        }
    }

    pub fn spawn_point(&self) -> Point {
        Point::new(self.mapping.width / 2, 1)
    }

    pub fn is_empty(&self, p: &Point) -> bool {
        self.mapping.point_to_index(p).map(|idx| self.pixels[idx]) == Some(None)
    }

    pub fn bake(&mut self, new_pixels: impl Iterator<Item = Pixel>) {
        for (idx, color) in new_pixels.filter_map(|Pixel { position, color }| {
            self.mapping
                .point_to_index(&position)
                .map(|idx| (idx, color))
        }) {
            self.pixels[idx] = Some(color);
        }
    }

    pub fn full_rows(&self) -> Vec<usize> {
        (0..self.mapping.height)
            .filter(|&row| {
                let idx = self.mapping.point_to_index(&Point::new(0, row)).unwrap();
                self.pixels[idx..idx + self.mapping.width]
                    .iter()
                    .all(|opt| opt.is_some())
            })
            .collect()
    }

    pub fn clear_rows<I: Iterator<Item = usize>>(&mut self, rows: I) {
        for row in rows {
            let idx = self.mapping.point_to_index(&Point::new(0, row)).unwrap();
            self.pixels.copy_within(0..idx, self.mapping.width);
            self.pixels[0..self.mapping.width].fill(None);
        }
    }

    pub fn render(&self, mut viewport: Viewport, animation_idx: usize) {
        for (p, color) in self
            .pixels
            .iter()
            .enumerate()
            .filter_map(|(i, o)| o.map(|c| (self.mapping.index_to_point(i), c)))
        {
            viewport.set(&p, color, BLACK, BLOCK_GLYPHS[0]);
        }

        if animation_idx > 0 {
            for y in self.full_rows() {
                for x in 0..self.mapping.width {
                    let p = Point::new(x, y);
                    let color = self.pixels[self.mapping.point_to_index(&p).unwrap()].unwrap();
                    viewport.set(
                        &p,
                        color,
                        BLACK,
                        BLOCK_GLYPHS[(animation_idx + y as usize) % BLOCK_GLYPHS.len()],
                    );
                }
            }
        }
    }
}
