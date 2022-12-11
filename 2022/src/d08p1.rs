use std::fmt::Debug;

use itertools::Itertools;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 8, 1)]
fn run(inp: &str) -> usize {
    let mut grid = Grid::from_str(inp);
    grid.mark();
    grid.visible.into_iter().flatten().filter(|b| *b).count()
}

struct Grid {
    trees: Vec<Vec<i8>>,
    visible: Vec<Vec<bool>>,
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
        let visible = trees
            .iter()
            .map(|row| row.iter().map(|_| false).collect_vec())
            .collect_vec();
        Self { trees, visible }
    }

    fn mark(&mut self) {
        let rows = self.trees.len();
        let cols = self.trees[0].len();

        for row in 0..rows {
            let line = (0..cols).map(|col| (row, col));
            self.mark_row(line.clone());
            self.mark_row(line.rev());
        }

        for col in 0..cols {
            let line = (0..rows).map(|row| (row, col));
            self.mark_row(line.clone());
            self.mark_row(line.rev());
        }
    }

    fn mark_row(&mut self, indices: impl Iterator<Item = (usize, usize)>) {
        let mut max = -1i8;
        for (row, col) in indices {
            let height = self.trees[row][col];
            self.visible[row][col] |= height > max;
            max = height.max(max);
        }
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.visible {
            for t in row {
                write!(f, "{}", if *t { '.' } else { 'x' })?;
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
        assert_eq!(run(inp), 21);
    }
}
