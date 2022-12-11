use std::fmt::Debug;

use itertools::Itertools;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 8, 2)]
fn run(inp: &str) -> usize {
    let grid = Grid::from_str(inp);
    (1..(grid.height() - 1))
        .cartesian_product(1..(grid.width() - 1))
        .map(|(row, col)| grid.scenic_score(row, col))
        .max()
        .unwrap()
}

struct Grid {
    trees: Vec<Vec<i8>>,
}

impl Grid {
    fn from_str(inp: &str) -> Self {
        let trees = inp
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        Self { trees }
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let edge = row == 0 || row == self.height() - 1 || col == 0 || col == self.width() - 1;
        if edge {
            return 0;
        }
        let down = (row..(self.height())).map(|r| (r, col));
        let up = (0..=row).rev().map(|r| (r, col));
        let right = (col..(self.width())).map(|c| (row, c));
        let left = (0..=col).rev().map(|c| (row, c));
        self.cast_ray(down) * self.cast_ray(up) * self.cast_ray(left) * self.cast_ray(right)
    }

    fn cast_ray(&self, indices: impl Iterator<Item = (usize, usize)>) -> usize {
        let mut trees = indices.map(|(row, col)| self.trees[row][col]);
        let start = trees.next().unwrap();
        let mut trees_seen = 0;
        for tree in trees {
            trees_seen += 1;
            if tree >= start {
                break;
            }
        }
        trees_seen
    }

    fn width(&self) -> usize {
        self.trees[0].len()
    }

    fn height(&self) -> usize {
        self.trees.len()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..(self.height()) {
            for col in 0..(self.width()) {
                let score = self.scenic_score(row, col);
                let c = match score {
                    0..=9 => score.to_string(),
                    _ => "x".to_string(),
                };
                write!(f, "{c}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "30373
25512
65332
33549
35390";
        assert_eq!(run(inp), 8);
    }
}
