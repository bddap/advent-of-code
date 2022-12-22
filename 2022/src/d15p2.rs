use std::ops::{Range, Sub};

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 15, 2)]
fn run(inp: &str) -> isize {
    solve(inp, 4000000)
}

fn solve(inp: &str, search_max: isize) -> isize {
    for target_row in 0.. {
        let inp = parse(inp);

        let mut negatives = Vec::<Range<isize>>::new();
        for (sensor, beacon) in &inp {
            let mradius = sensor.sub(*beacon).manhattan();
            let distance_to_row = (sensor.y - target_row).abs();

            if distance_to_row > mradius {
                continue;
            }
            assert!(distance_to_row <= mradius);

            let coverage = mradius + 1 - distance_to_row;

            negatives.push((sensor.x - coverage)..(sensor.x + coverage + 1));
        }
        negatives.sort_by_key(|r| r.start);
        let negatives = negatives.into_iter().fold(Vec::new(), push_range);
        if let Some(ret) = hole(search_max, &negatives) {
            return ret;
        }
    }
    panic!()
}

fn hole(search_max: isize, ruled_out: &[Range<isize>]) -> Option<isize> {
    assert!(!search_max.is_negative());
    todo!()
}

/// ensures ranges do not overlap
fn push_range(mut rgs: Vec<Range<isize>>, b: Range<isize>) -> Vec<Range<isize>> {
    let Some(last) = rgs.last_mut() else {
        rgs.push(b);
        return rgs;
    };

    assert!(last.start <= b.start);
    if last.end < b.start {
        rgs.push(b);
        return rgs;
    }

    last.end = last.end.max(b.end);
    rgs
}

fn parse(inp: &str) -> Vec<(Point, Point)> {
    use aoc_parse::{
        parser,
        prelude::{isize, lines, Parser},
    };
    let point = parser!("x=" x:isize ", y=" y:isize => Point {x, y});
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
        assert_eq!(solve(INP, 20), 56000011);
    }

    #[test]
    fn rac() {
        assert_eq!(push_range(Vec::new(), 2..4), vec![2..4]);
        assert_eq!(push_range(vec![2..4], 2..4), vec![2..4]);
        assert_eq!(push_range(vec![1..4], 2..4), vec![1..4]);
        assert_eq!(push_range(vec![1..2], 2..4), vec![1..4]);
        assert_eq!(push_range(vec![1..2], 3..4), vec![1..2, 3..4]);
        assert_eq!(push_range(vec![1..2], 1..2), vec![1..2]);
        assert_eq!(push_range(vec![1..2], 1..3), vec![1..3]);
        assert_eq!(push_range(vec![1..3], 1..2), vec![1..3]);
    }
}
