use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Block {
    L,
    J,
    O,
    S,
    Z,
    T,
    I,
}

impl Block {
    pub fn random(rng: &mut RandomNumberGenerator) -> Self {
        match rng.range(0, 7) {
            0 => Block::L,
            1 => Block::J,
            2 => Block::O,
            3 => Block::S,
            4 => Block::Z,
            5 => Block::T,
            6 => Block::I,
            _ => panic!(),
        }
    }

    fn colors(&self) -> ColorPair {
        ColorPair::new(
            match self {
                Block::L => ORANGE3,
                Block::J => BLUE3,
                Block::O => YELLOW3,
                Block::S => GREEN3,
                Block::Z => RED3,
                Block::T => PURPLE3,
                Block::I => TURQUOISE3,
            },
            BLACK,
        )
    }

    fn points(&self) -> [Point; 4] {
        match self {
            Block::L => [
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, -1),
            ],
            Block::J => [
                Point::new(-1, -1),
                Point::new(-1, 0),
                Point::new(0, 0),
                Point::new(1, 0),
            ],
            Block::O => [
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, -1),
                Point::new(1, -1),
            ],
            Block::S => [
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(1, -1),
                Point::new(-1, 0),
            ],
            Block::Z => [
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(-1, -1),
                Point::new(1, 0),
            ],
            Block::T => [
                Point::new(0, -1),
                Point::new(0, 0),
                Point::new(-1, 0),
                Point::new(1, 0),
            ],
            Block::I => [
                Point::new(-1, -1),
                Point::new(0, -1),
                Point::new(1, -1),
                Point::new(2, -1),
            ],
        }
    }

    fn rotation_offset(&self) -> Point {
        use Block::*;
        match self {
            L | J | S | Z | T => Point::zero(),
            O | I => Point::new(1, -1),
        }
    }

    pub fn components<Tag>(&self, spawn: &Point) -> [(Game, Tag, Position, Pivot, PixelRender); 4]
    where
        Tag: Copy + Default,
    {
        let tag = Tag::default();
        let offset = self.rotation_offset();
        let render = PixelRender {
            colors: self.colors(),
            glyph: to_cp437('█'),
        };
        self.points().map(|pt| {
            (
                Game,
                tag,
                Position(pt + *spawn),
                Pivot(pt * 2 - offset),
                render,
            )
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;

    struct Rotating([(Game, Preview, Position, Pivot, PixelRender); 4]);

    impl Iterator for Rotating {
        type Item = [Point; 4];

        fn next(&mut self) -> Option<[Point; 4]> {
            let copy = self.0.map(|(_, _, Position(pos), _, _)| pos);
            self.0
                .iter_mut()
                .for_each(|(_, _, pos, pivot, _)| Rotation::Deg90.apply_to(pos, pivot));
            Some(copy)
        }
    }

    fn test_rotation_states(block: Block, expected: [[&str; 4]; 4]) {
        let expected_strs: [String; 4] =
            [0, 1, 2, 3].map(|i| expected.map(|line| line[i]).join("\n"));

        let components = block.components::<Preview>(&Point::new(2, 2));

        for (actual, expected) in Rotating(components).zip(expected_strs) {
            assert_eq!(str_from_points(actual.into_iter(), 6, 4), expected);
        }
    }

    #[test]
    fn rotation_states() {
        test_rotation_states(
            Block::L,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░░█░░", "░░█░░░", "░░░░░░", "░██░░░"],
                ["░███░░", "░░█░░░", "░███░░", "░░█░░░"],
                ["░░░░░░", "░░██░░", "░█░░░░", "░░█░░░"],
            ],
        );
        test_rotation_states(
            Block::J,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░█░░░░", "░░██░░", "░░░░░░", "░░█░░░"],
                ["░███░░", "░░█░░░", "░███░░", "░░█░░░"],
                ["░░░░░░", "░░█░░░", "░░░█░░", "░██░░░"],
            ],
        );
        test_rotation_states(
            Block::O,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░██░░", "░░██░░", "░░██░░", "░░██░░"],
                ["░░██░░", "░░██░░", "░░██░░", "░░██░░"],
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
            ],
        );
        test_rotation_states(
            Block::S,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░██░░", "░░█░░░", "░░░░░░", "░█░░░░"],
                ["░██░░░", "░░██░░", "░░██░░", "░██░░░"],
                ["░░░░░░", "░░░█░░", "░██░░░", "░░█░░░"],
            ],
        );
        test_rotation_states(
            Block::Z,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░██░░░", "░░░█░░", "░░░░░░", "░░█░░░"],
                ["░░██░░", "░░██░░", "░██░░░", "░██░░░"],
                ["░░░░░░", "░░█░░░", "░░██░░", "░█░░░░"],
            ],
        );
        test_rotation_states(
            Block::T,
            [
                ["░░░░░░", "░░░░░░", "░░░░░░", "░░░░░░"],
                ["░░█░░░", "░░█░░░", "░░░░░░", "░░█░░░"],
                ["░███░░", "░░██░░", "░███░░", "░██░░░"],
                ["░░░░░░", "░░█░░░", "░░█░░░", "░░█░░░"],
            ],
        );
        test_rotation_states(
            Block::I,
            [
                ["░░░░░░", "░░░█░░", "░░░░░░", "░░█░░░"],
                ["░████░", "░░░█░░", "░░░░░░", "░░█░░░"],
                ["░░░░░░", "░░░█░░", "░████░", "░░█░░░"],
                ["░░░░░░", "░░░█░░", "░░░░░░", "░░█░░░"],
            ],
        );
    }
}
