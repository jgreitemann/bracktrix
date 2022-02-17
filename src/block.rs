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

    pub fn pixels(&self) -> [Pixel; 4] {
        let color = self.color();
        self.points().map(move |position| Pixel {
            position,
            color,
            glyph: '█',
        })
    }

    pub fn rotation_offset(&self) -> Point {
        use BlockShape::*;
        match self {
            L | J | S | Z | T => Point::zero(),
            O | I => Point::new(1, -1),
        }
    }
}

/*

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
 */
