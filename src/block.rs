use crate::prelude::*;

enum BlockShape {
    L,
}

#[derive(Copy, Clone)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    fn rotate_clockwise(&self) -> Self {
        use Rotation::*;
        match self {
            Deg0 => Deg90,
            Deg90 => Deg180,
            Deg180 => Deg270,
            Deg270 => Deg0,
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        use Rotation::*;
        match self {
            Deg0 => Deg270,
            Deg90 => Deg0,
            Deg180 => Deg90,
            Deg270 => Deg180,
        }
    }

    fn apply_to(&self, p: &Point) -> Point {
        match self {
            Rotation::Deg0 => p.clone(),
            Rotation::Deg90 => Point::new(-p.y, p.x),
            Rotation::Deg180 => Point::new(-p.x, -p.y),
            Rotation::Deg270 => Point::new(p.y, -p.x),
        }
    }
}

pub struct Block {
    shape: BlockShape,
    origin: Point,
    rotation: Rotation,
}

impl Block {
    pub fn new(origin: Point) -> Self {
        let mut rng = RandomNumberGenerator::new();
        Self {
            shape: match rng.range(0, 1) {
                0 => BlockShape::L,
                _ => panic!(),
            },
            origin,
            rotation: Rotation::Deg0,
        }
    }

    pub fn update(&mut self, ctx: &BTerm, frame_idx: usize) {
        if let Some(key) = ctx.key {
            self.origin.x += match key {
                VirtualKeyCode::Left => -1,
                VirtualKeyCode::Right => 1,
                _ => 0,
            };

            self.rotation = match key {
                VirtualKeyCode::Up => self.rotation.rotate_counter_clockwise(),
                VirtualKeyCode::Down => self.rotation.rotate_clockwise(),
                _ => self.rotation,
            };
        }

        if frame_idx % 4 == 0 {
            self.origin.y += 1;
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let points = [
            Point::new(0, -1),
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ];

        for Pixel { position, color } in self.as_pixels() {
            ctx.set(position.x, position.y, color, BLACK, to_cp437('█'));
        }
    }

    pub fn as_pixels<'a>(&'a self) -> impl Iterator<Item = Pixel> + 'a {
        let points = [
            Point::new(0, -1),
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ];

        points
            .into_iter()
            .map(|p| self.rotation.apply_to(&p) + self.origin)
            .map(|position| Pixel {
                position,
                color: RED,
            })
    }
}
