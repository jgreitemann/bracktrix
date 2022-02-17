use crate::prelude::*;
use bracket_lib::prelude::*;
use std::collections::HashSet;

pub type Color = (u8, u8, u8);

pub trait Transform<'a> {
    type Element: 'a;
    fn apply_to(&self, elem: Self::Element) -> Self::Element;
    fn inv(&self) -> Self;
}

pub struct Translation(pub Point);

impl<'a> Transform<'a> for Translation {
    type Element = &'a mut Position;

    fn apply_to(&self, pos: &'a mut Position) -> Self::Element {
        let &Translation(delta) = self;
        let Position(point) = pos;
        *point += delta;
        pos
    }

    fn inv(&self) -> Self {
        Translation(self.0 * (-1))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

impl Rotation {
    fn applied_to(&self, p: &Point) -> Point {
        use Rotation::*;
        match self {
            Deg0 => p.clone(),
            Deg90 => Point::new(-p.y, p.x),
            Deg180 => Point::new(-p.x, -p.y),
            Deg270 => Point::new(p.y, -p.x),
        }
    }
}

impl<'a> Transform<'a> for Rotation {
    type Element = (&'a mut Position, &'a mut Pivot);

    fn apply_to(&self, elem: (&'a mut Position, &'a mut Pivot)) -> Self::Element {
        let (Position(pos), Pivot(pivot)) = elem;
        let new_pivot = self.applied_to(pivot);
        *pos += (new_pivot - *pivot) / 2;
        *pivot = new_pivot;
        elem
    }

    fn inv(&self) -> Self {
        use Rotation::*;
        match self {
            Deg0 => Deg0,
            Deg90 => Deg270,
            Deg180 => Deg180,
            Deg270 => Deg90,
        }
    }
}

pub struct Pixel {
    pub position: Point,
    pub color: Color,
    pub glyph: char,
}

pub fn grow_rect(rect: &Rect, amount: i32) -> Rect {
    Rect {
        x1: rect.x1 - amount,
        y1: rect.y1 - amount,
        x2: rect.x2 + amount,
        y2: rect.y2 + amount,
    }
}

pub fn rect_inner_border_points(rect: &Rect) -> impl Iterator<Item = Point> {
    let Rect { x1, x2, y1, y2 } = *rect;
    (x1..x2 - 1)
        .skip(1)
        .map(move |x| Point::new(x, y1))
        .chain((y1..y2 - 1).map(move |y| Point::new(x2 - 1, y)))
        .chain((x1..=x2 - 1).map(move |x| Point::new(x, y2 - 1)))
        .chain((y1..y2 - 1).map(move |y| Point::new(x1, y)))
}

pub fn rect_outer_border_points(rect: &Rect) -> impl Iterator<Item = Point> {
    rect_inner_border_points(&grow_rect(rect, 1))
}

pub fn border_glyph(point: &Point, all_border_points: &HashSet<Point>) -> char {
    let left = all_border_points.contains(&Point::new(point.x - 1, point.y));
    let right = all_border_points.contains(&Point::new(point.x + 1, point.y));
    let above = all_border_points.contains(&Point::new(point.x, point.y - 1));
    let below = all_border_points.contains(&Point::new(point.x, point.y + 1));
    match (left, right, above, below) {
        (false, false, false, false) => '♦',
        (false, false, false, true) => '╥',
        (false, false, true, false) => '╨',
        (false, false, true, true) => '║',
        (false, true, false, false) => '╞',
        (false, true, false, true) => '╔',
        (false, true, true, false) => '╚',
        (false, true, true, true) => '╠',
        (true, false, false, false) => '╡',
        (true, false, false, true) => '╗',
        (true, false, true, false) => '╝',
        (true, false, true, true) => '╣',
        (true, true, false, false) => '═',
        (true, true, false, true) => '╦',
        (true, true, true, false) => '╩',
        (true, true, true, true) => '╬',
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::*;
    use std::collections::HashSet;

    #[test]
    fn rect_inner_border_points() {
        let map = r#"..........
..........
.######...
.#....#...
.#....#...
.######...
..........
        "#;

        assert_eq!(
            super::rect_inner_border_points(&Rect::with_size(1, 2, 6, 4)).collect::<HashSet<_>>(),
            points_from_str(map).collect::<HashSet<_>>()
        );
    }

    #[test]
    fn rect_outer_border_points() {
        let map = r#"..........
########..
#......#..
#......#..
#......#..
#......#..
########..
        "#;

        assert_eq!(
            super::rect_outer_border_points(&Rect::with_size(1, 2, 6, 4)).collect::<HashSet<_>>(),
            points_from_str(map).collect::<HashSet<_>>()
        );
    }

    fn test_border_glyph(input_map: &str, expected_map: &str) {
        let input_points: HashSet<_> = points_from_str(input_map).collect();

        assert_eq!(
            input_points
                .iter()
                .map(|&p| (p, border_glyph(&p, &input_points)))
                .collect::<HashSet<_>>(),
            pixels_from_str(expected_map).collect::<HashSet<_>>()
        );
    }

    #[test]
    fn border_glyphs_for_single_rect() {
        test_border_glyph(
            r#"..........
..........
.######...
.#....#...
.#....#...
.######...
.........."#,
            r#"..........
..........
.╔════╗...
.║....║...
.║....║...
.╚════╝...
.........."#,
        )
    }

    #[test]
    fn border_glyphs_for_solid_rect() {
        test_border_glyph(
            r#"..........
..........
.######...
.######...
.######...
.######...
.........."#,
            r#"..........
..........
.╔╦╦╦╦╗...
.╠╬╬╬╬╣...
.╠╬╬╬╬╣...
.╚╩╩╩╩╝...
.........."#,
        )
    }

    #[test]
    fn border_glyphs_for_intersecting_rects() {
        test_border_glyph(
            r#"............
.#######....
.#.....#....
.#..#######.
.#..#..#..#.
.#..#..#..#.
.#######..#.
....#.....#.
....#######."#,
            r#"............
.╔═════╗....
.║.....║....
.║..╔══╬══╗.
.║..║..║..║.
.║..║..║..║.
.╚══╬══╝..║.
....║.....║.
....╚═════╝."#,
        )
    }

    #[test]
    fn border_glyphs_for_protruding_shapes() {
        test_border_glyph(
            r#"............
...####.....
...#..#.....
.##########.
.#........#.
.#...######.
.#...#..#...
.#...####...
.#####......"#,
            r#"............
...╔══╗.....
...║..║.....
.╔═╩══╩═══╗.
.║........║.
.║...╔══╦═╝.
.║...║..║...
.║...╠══╝...
.╚═══╝......"#,
        )
    }
}
