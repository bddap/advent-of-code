use std::{collections::HashSet, iter::repeat};

use glam::IVec2;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 9, 1)]
fn run(inp: &str) -> usize {
    let mut head = IVec2::new(0, 0);
    let mut tail = IVec2::new(0, 0);
    let mut visited = HashSet::<IVec2>::new();
    for command in commands(inp) {
        head += command;
        chase(head, &mut tail);
        visited.insert(tail);
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
        assert_eq!(run(inp), 13);
    }
}
