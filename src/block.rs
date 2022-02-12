use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
enum BlockShape {
    L,
    InvL,
    Square,
    S,
    Z,
    T,
    I,
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
            shape: match rng.range(0, 7) {
                0 => BlockShape::L,
                1 => BlockShape::InvL,
                2 => BlockShape::Square,
                3 => BlockShape::S,
                4 => BlockShape::Z,
                5 => BlockShape::T,
                6 => BlockShape::I,
                _ => panic!(),
            },
            origin,
            rotation: Rotation::Deg0,
        }
    }

    pub fn with_keys_applied(mut self, ctx: &BTerm) -> Self {
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

        self
    }

    pub fn with_gravity_applied(mut self, frame_idx: usize) -> Self {
        if frame_idx % 4 == 0 {
            self.origin.y += 1;
        }

        self
    }

    pub fn render(&self, mut viewport: Viewport) {
        for pix in self.pixels() {
            viewport.draw(&pix);
        }
    }

    pub fn points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        let points = match self.shape {
            BlockShape::L => [
                Point::new(0, -1),
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(1, 1),
            ],
            BlockShape::InvL => [
                Point::new(0, -1),
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(-1, 1),
            ],
            BlockShape::Square => [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(1, -1),
            ],
            BlockShape::S => [
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(1, -1),
                Point::new(-1, 0),
            ],
            BlockShape::Z => [
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(-1, -1),
                Point::new(1, 0),
            ],
            BlockShape::T => [
                Point::new(0, -1),
                Point::new(0, 0),
                Point::new(-1, 0),
                Point::new(1, 0),
            ],
            BlockShape::I => [
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
            ],
        };

        points
            .into_iter()
            .map(|p| self.rotation.apply_to(&p) + self.origin)
    }

    pub fn pixels<'a>(&'a self) -> impl Iterator<Item = Pixel> + 'a {
        let color = match self.shape {
            BlockShape::L => ORANGE3,
            BlockShape::InvL => BLUE3,
            BlockShape::Square => YELLOW3,
            BlockShape::S => GREEN3,
            BlockShape::Z => RED3,
            BlockShape::T => PURPLE3,
            BlockShape::I => TURQUOISE3,
        };
        self.points().map(move |position| Pixel {
            position,
            color,
            glyph: '█',
        })
    }
}
