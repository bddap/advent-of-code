use std::{collections::HashSet, fmt::Debug, iter::once, ops::Add};

use itertools::Itertools;

const SAND_SRC: Point = Point { x: 500, y: 0 };

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 14, 2)]
fn run(inp: &str) -> usize {
    Raster::parse(inp).fill(SAND_SRC)
}

#[derive(Hash, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

struct Raster {
    grid: HashSet<Point>,
    floor: isize,
}

impl Debug for Raster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let xmin = self.grid.iter().map(|p| p.x).min().unwrap();
        let xmax = self.grid.iter().map(|p| p.x).max().unwrap();
        for y in 0..=(self.floor) {
            for x in xmin..(xmax + 1) {
                write!(f, "{}", if self.get(Point { x, y }) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

impl Raster {
    fn parse(inp: &str) -> Self {
        let grid: HashSet<Point> = lines(inp).flat_map(draw_line).collect();
        let floor = grid.iter().map(|p| p.y).max().unwrap() + 2;
        Raster { grid, floor }
    }

    fn get(&self, point: Point) -> bool {
        point.y >= self.floor || self.grid.contains(&point)
    }

    /// returns whether the spot is newly added
    fn add(&mut self, point: Point) -> bool {
        if point.y >= self.floor {
            return false;
        }
        self.grid.insert(point)
    }

    fn fill(&mut self, pointer: Point) -> usize {
        if !self.add(pointer) {
            return 0;
        }
        self.fill(pointer + DOWN)
            + self.fill(pointer + DOWN + LEFT)
            + self.fill(pointer + DOWN + RIGHT)
            + 1
    }
}

/// returns an iterator over the discrete locations the line touches
fn draw_line([mut a, mut b]: [Point; 2]) -> impl Iterator<Item = Point> {
    assert!(a.x == b.x || a.y == b.y, "aint nobody got time for that");
    if (a.x, a.y) > (b.x, b.y) {
        std::mem::swap(&mut a, &mut b);
    }
    let start = once(a.clone());
    let rest = (0..).map_while(move |_| {
        if a == b {
            return None;
        }
        a.x = (a.x + 1).min(b.x);
        a.y = (a.y + 1).min(b.y);
        Some(a)
    });
    start.chain(rest)
}

fn lines(inp: &'_ str) -> impl Iterator<Item = [Point; 2]> + '_ {
    inp.lines()
        .flat_map(|line| trace(line).zip(trace(line).skip(1)))
        .map(|(a, b)| [a, b])
}

fn trace(line: &'_ str) -> impl Iterator<Item = Point> + '_ {
    line.split(" -> ").map(|point| {
        let (x, y) = point
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Point { x, y }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(run(inp), 93);
    }
}
