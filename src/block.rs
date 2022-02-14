use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum BlockShape {
    L,
    J,
    O,
    S,
    Z,
    T,
    I,
}

impl BlockShape {
    pub fn random() -> Self {
        let mut rng = RandomNumberGenerator::new();
        match rng.range(0, 7) {
            0 => BlockShape::L,
            1 => BlockShape::J,
            2 => BlockShape::O,
            3 => BlockShape::S,
            4 => BlockShape::Z,
            5 => BlockShape::T,
            6 => BlockShape::I,
            _ => panic!(),
        }
    }

    fn color(&self) -> Color {
        match self {
            BlockShape::L => ORANGE3,
            BlockShape::J => BLUE3,
            BlockShape::O => YELLOW3,
            BlockShape::S => GREEN3,
            BlockShape::Z => RED3,
            BlockShape::T => PURPLE3,
            BlockShape::I => TURQUOISE3,
        }
    }

    fn points(&self) -> [Point; 4] {
        match self {
            BlockShape::L => [
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, -1),
            ],
            BlockShape::J => [
                Point::new(-1, -1),
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0),
            ],
            BlockShape::O => [
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
                Point::new(-1, -1),
                Point::new(0, -1),
                Point::new(1, -1),
                Point::new(2, -1),
            ],
        }
    }

    fn rotation_offset(&self) -> Point {
        use BlockShape::*;
        match self {
            L | J | S | Z | T => Point::zero(),
            O | I => Point::new(1, -1),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
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
    pub fn new(shape: BlockShape, origin: Point) -> Self {
        Self {
            shape,
            origin,
            rotation: Rotation::Deg0,
        }
    }

    pub fn shape(&self) -> BlockShape {
        self.shape
    }

    pub fn rotate_clockwise(&mut self) {
        use Rotation::*;
        self.rotation = match self.rotation {
            Deg0 => Deg90,
            Deg90 => Deg180,
            Deg180 => Deg270,
            Deg270 => Deg0,
        };
    }

    pub fn rotate_counter_clockwise(&mut self) {
        use Rotation::*;
        self.rotation = match self.rotation {
            Deg0 => Deg270,
            Deg90 => Deg0,
            Deg180 => Deg90,
            Deg270 => Deg180,
        };
    }

    pub fn with_keys_applied(mut self, ctx: &BTerm) -> Self {
        if let Some(key) = ctx.key {
            self.origin.x += match key {
                VirtualKeyCode::Left => -1,
                VirtualKeyCode::Right => 1,
                _ => 0,
            };

            match key {
                VirtualKeyCode::Up => self.rotate_counter_clockwise(),
                VirtualKeyCode::Down => self.rotate_clockwise(),
                _ => {}
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
        self.shape.points().into_iter().map(|p| {
            (self
                .rotation
                .apply_to(&(p * 2 - self.shape.rotation_offset()))
                + self.shape.rotation_offset())
                / 2
                + self.origin
        })
    }

    pub fn pixels<'a>(&'a self) -> impl Iterator<Item = Pixel> + 'a {
        let color = self.shape.color();
        self.points().map(move |position| Pixel {
            position,
            color,
            glyph: '█',
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    struct Rotating(Block);

    impl Iterator for Rotating {
        type Item = Block;

        fn next(&mut self) -> Option<Self::Item> {
            let copy = self.0.clone();
            self.0.rotate_clockwise();
            Some(copy)
        }
    }

    fn test_rotation_states(shape: BlockShape, expected: [[&str; 4]; 4]) {
        let expected_strs: [String; 4] =
            [0, 1, 2, 3].map(|i| expected.map(|line| line[i]).join("\n"));

        for (block, exp) in Rotating(Block::new(shape, Point::new(2, 2))).zip(expected_strs) {
            assert_eq!(str_from_points(block.points(), 6, 4), exp);
        }
    }

    #[test]
    fn rotation_states() {
        test_rotation_states(
            BlockShape::L,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░░█░░", "░░█░░░", "░░░░░░", "░██░░░"],
                ["░███░░", "░░█░░░", "░███░░", "░░█░░░"],
                ["░░░░░░", "░░██░░", "░█░░░░", "░░█░░░"],
            ],
        );
        test_rotation_states(
            BlockShape::J,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░█░░░░", "░░██░░", "░░░░░░", "░░█░░░"],
                ["░███░░", "░░█░░░", "░███░░", "░░█░░░"],
                ["░░░░░░", "░░█░░░", "░░░█░░", "░██░░░"],
            ],
        );
        test_rotation_states(
            BlockShape::O,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░██░░", "░░██░░", "░░██░░", "░░██░░"],
                ["░░██░░", "░░██░░", "░░██░░", "░░██░░"],
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
            ],
        );
        test_rotation_states(
            BlockShape::S,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░██░░", "░░█░░░", "░░░░░░", "░█░░░░"],
                ["░██░░░", "░░██░░", "░░██░░", "░██░░░"],
                ["░░░░░░", "░░░█░░", "░██░░░", "░░█░░░"],
            ],
        );
        test_rotation_states(
            BlockShape::Z,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░██░░░", "░░░█░░", "░░░░░░", "░░█░░░"],
                ["░░██░░", "░░██░░", "░██░░░", "░██░░░"],
                ["░░░░░░", "░░█░░░", "░░██░░", "░█░░░░"],
            ],
        );
        test_rotation_states(
            BlockShape::T,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░█░░░", "░░█░░░", "░░░░░░", "░░█░░░"],
                ["░███░░", "░░██░░", "░███░░", "░██░░░"],
                ["░░░░░░", "░░█░░░", "░░█░░░", "░░█░░░"],
            ],
        );
        test_rotation_states(
            BlockShape::I,
            [
                ["░░░░░░", "░░░█░░", "░░░░░░", "░░█░░░"],
                ["░████░", "░░░█░░", "░░░░░░", "░░█░░░"],
                ["░░░░░░", "░░░█░░", "░████░", "░░█░░░"],
                ["░░░░░░", "░░░█░░", "░░░░░░", "░░█░░░"],
            ],
        );
    }
}
