use std::{collections::HashSet, iter::repeat};

use glam::IVec2;
use itertools::Itertools;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 9, 2)]
fn run(inp: &str) -> usize {
    let mut snek = repeat(IVec2::new(0, 0)).take(10).collect_vec();
    let mut visited = HashSet::<IVec2>::new();
    for command in commands(inp) {
        *snek.first_mut().unwrap() += command;
        for i in 0..(snek.len() - 1) {
            let front = snek[i];
            chase(front, &mut snek[i + 1]);
        }
        visited.insert(*snek.last().unwrap());
    }
    visited.len()
}

fn chase(head: IVec2, tail: &mut IVec2) {
    let diff = head - *tail;
    if diff.x.abs() > 1 || diff.y.abs() > 1 {
        *tail += diff.signum();
    }
}

fn commands<'a>(inp: &'a str) -> impl Iterator<Item = IVec2> + 'a {
    inp.lines().flat_map(|line| {
        let mut command = line.split(' ');
        let c = command.next().unwrap();
        let n = command.next().unwrap().parse::<usize>().unwrap();
        let mov = match c {
            "R" => [0, 1],
            "U" => [1, 0],
            "D" => [-1, 0],
            "L" => [0, -1],
            _ => panic!(),
        };
        repeat(mov.into()).take(n)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(run(inp), 1);
    }

    #[test]
    fn test2() {
        let inp = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(run(inp), 36);
    }
}
