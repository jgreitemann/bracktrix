use bracket_lib::prelude::*;

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
            points_from_str(map)
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
            points_from_str(map)
        );
    }
}
