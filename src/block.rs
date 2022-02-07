use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
enum BlockShape {
    L,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    pub fn spawn() -> Self {
        Self::new(Point::new(SCREEN_WIDTH / 2, 1))
    }

    pub fn updated(&self, ctx: &BTerm, frame_idx: usize) -> Self {
        let mut updated = self.clone();

        if let Some(key) = ctx.key {
            updated.origin.x += match key {
                VirtualKeyCode::Left => -1,
                VirtualKeyCode::Right => 1,
                _ => 0,
            };

            updated.rotation = match key {
                VirtualKeyCode::Up => updated.rotation.rotate_counter_clockwise(),
                VirtualKeyCode::Down => updated.rotation.rotate_clockwise(),
                _ => updated.rotation,
            };
        }

        if frame_idx % 4 == 0 {
            updated.origin.y += 1;
        }

        updated
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for Pixel { position, color } in self.pixels() {
            ctx.set(position.x, position.y, color, BLACK, to_cp437('█'));
        }
    }

    pub fn points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        let points = [
            Point::new(0, -1),
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
        ];

        points
            .into_iter()
            .map(|p| self.rotation.apply_to(&p) + self.origin)
    }

    pub fn pixels<'a>(&'a self) -> impl Iterator<Item = Pixel> + 'a {
        self.points().map(|position| Pixel {
            position,
            color: RED,
        })
    }
}
