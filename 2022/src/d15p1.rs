use std::ops::{Range, Sub};

use aoc_parse::{
    parser,
    prelude::{isize as Pisize, lines, Parser},
};

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 15, 1)]
fn run(inp: &str) -> usize {
    let target_row = 10;
    let inp = parse(inp);

    let mut negatives = Vec::<Range<isize>>::new();
    for (sensor, beacon) in inp {
        let mradius = sensor.sub(beacon).manhattan();
        let dtorow = (sensor.y - target_row).abs();
        let coverage = mradius.checked_sub(dtorow).unwrap_or(0);
        if coverage == 0 {
            continue;
        }
        negatives.push((sensor.x - coverage)..(sensor.x + coverage + 1));
    }
    // todo need to not mark beacons as negative
    negatives.sort_by_key(|r| r.start);

    negatives.into_iter().reduce(range_combine).unwrap().len()
}

fn range_combine(a: Range<isize>, b: Range<isize>) -> Range<isize> {
    assert!(a.start <= b.start);
    let overlap = overlap(&a, &b);
    let end = a.end.max(b.end);
    let len = a.len() as isize + b.len() as isize - overlap;
    (end - len)..end
}

fn overlap(a: &Range<isize>, b: &Range<isize>) -> isize {
    assert!(a.start <= b.start);
    if a.contains(&b.end) {
        return b.len() as isize;
    }
    if a.contains(&b.start) {
        return a.end - b.start;
    }
    0
}

fn parse(inp: &str) -> Vec<(Point, Point)> {
    let point = parser!("x=" x:Pisize ", y=" y:Pisize => Point {x, y});
    let line = parser![
        "Sensor at "
            point
            ": closest beacon is at "
            point
    ];
    let lines = parser!(lines(line));
    lines.parse(inp).unwrap()
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Point {
    fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INP: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test() {
        assert_eq!(run(INP), 26);
    }

    #[test]
    fn rac() {
        assert_eq!(range_combine(0..2, 2..4), 0..4);
        assert_eq!(overlap(&(0..3), &(2..4)), 1);
        assert_eq!(range_combine(0..3, 2..4), 0..4);
        assert_eq!(range_combine(0..3, 0..4), 0..4);
        assert_eq!(range_combine(0..5, 0..4), 0..5);
        assert_eq!(range_combine(0..1, 2..4), 1..4);
    }
}
