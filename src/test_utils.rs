use std::collections::HashSet;

use bracket_lib::prelude::*;

pub fn points_from_str(map_str: &str) -> HashSet<Point> {
    map_str
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some(Point::new(x, y)),
                _ => None,
            })
        })
        .collect()
}

mod test {
    use super::*;

    #[test]
    fn points_from_str() {
        let map_str = r#"#......#
...#..##
..###...
.##.##..
.##..##.
..####..
...##..#"#;

        let points: HashSet<_> = [
            Point::new(0, 0),
            Point::new(7, 0),
            Point::new(3, 1),
            Point::new(6, 1),
            Point::new(7, 1),
            Point::new(2, 2),
            Point::new(3, 2),
            Point::new(4, 2),
            Point::new(1, 3),
            Point::new(2, 3),
            Point::new(4, 3),
            Point::new(5, 3),
            Point::new(1, 4),
            Point::new(2, 4),
            Point::new(5, 4),
            Point::new(6, 4),
            Point::new(2, 5),
            Point::new(3, 5),
            Point::new(4, 5),
            Point::new(5, 5),
            Point::new(3, 6),
            Point::new(4, 6),
            Point::new(7, 6),
        ]
        .into_iter()
        .collect();

        assert_eq!(super::points_from_str(map_str), points);
    }
}
