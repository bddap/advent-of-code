use std::iter::once;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 10, 1)]
fn run(inp: &str) -> isize {
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|k| integrated_value_at_cycle(inp, k) * k as isize)
        .sum()
}

fn integrated_value_at_cycle(inp: &str, cycle: usize) -> isize {
    deltas_per_cycle(inp).take(cycle - 1).sum::<isize>() + 1
}

fn deltas_per_cycle(inp: &'_ str) -> impl Iterator<Item = isize> + '_ {
    inp.lines().flat_map(|line| {
        let mut ls = line.split(' ');
        let command = ls.next().unwrap();
        let scalar = ls.next().map(|n| n.parse().unwrap());
        assert!((command == "addx" && scalar.is_some()) || (command == "noop" && scalar.is_none()));
        once(0isize).into_iter().chain(scalar)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let inp = "noop
addx 3
addx -5";
        assert_eq!(integrated_value_at_cycle(inp, 1), 1);
        assert_eq!(integrated_value_at_cycle(inp, 2), 1);
        assert_eq!(integrated_value_at_cycle(inp, 3), 1);
    }

    #[test]
    fn test2() {
        let inp = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        assert_eq!(integrated_value_at_cycle(inp, 20), 21);
        assert_eq!(integrated_value_at_cycle(inp, 60), 19);
        assert_eq!(integrated_value_at_cycle(inp, 100), 18);
        assert_eq!(integrated_value_at_cycle(inp, 140), 21);
        assert_eq!(integrated_value_at_cycle(inp, 180), 16);
        assert_eq!(integrated_value_at_cycle(inp, 220), 18);
        assert_eq!(run(inp), 13140);
    }
}
