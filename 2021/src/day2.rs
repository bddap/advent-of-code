enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

struct Pos {
    depth: u64,
    dist: u64,
}

impl Pos {
    fn run(&mut self, command: Command) {
        match command {
            Command::Forward(n) => self.dist += n,
            Command::Down(n) => self.depth += n,
            Command::Up(n) => self.depth -= n,
        }
    }
}

fn parse(input: &str) -> Vec<Command> {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::u64, multi::separated_list0,
        sequence::preceded, IResult, Parser,
    };
    let r: IResult<&str, Vec<Command>> = separated_list0(
        tag("\n"),
        alt((
            preceded(tag("forward "), u64).map(Command::Forward),
            preceded(tag("down "), u64).map(Command::Down),
            preceded(tag("up "), u64).map(Command::Up),
        )),
    )(input);
    r.unwrap().1
}

#[aoc_runner_derive::aoc(day2, part1)]
pub fn solve1(input: &str) -> u64 {
    let mut pos = Pos { depth: 0, dist: 0 };
    for command in parse(input) {
        pos.run(command);
    }
    pos.depth * pos.dist
}

struct Pos2 {
    depth: i64,
    dist: i64,
    aim: i64,
}

impl Pos2 {
    fn run(&mut self, command: Command) {
        match command {
            Command::Forward(n) => {
                self.dist += n as i64;
                self.depth += self.aim * n as i64;
            }
            Command::Down(n) => self.aim += n as i64,
            Command::Up(n) => self.aim -= n as i64,
        }
    }
}

#[aoc_runner_derive::aoc(day2, part2)]
pub fn solve2(input: &str) -> i64 {
    let mut pos = Pos2 {
        depth: 0,
        dist: 0,
        aim: 0,
    };
    for command in parse(input) {
        pos.run(command);
    }
    pos.depth * pos.dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn s1() {
        assert_eq!(
            solve1(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ),
            150
        );
    }

    #[test]
    fn s2() {
        assert_eq!(
            solve2(
                "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            ),
            900
        );
    }
}
