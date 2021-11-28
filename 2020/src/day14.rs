use std::collections::HashMap;

use nom::Parser;

fn parse(input: &str) -> Vec<Command> {
    use nom::{character::complete::char, multi::separated_list0};
    separated_list0(char('\n'), Command::nom)(input).unwrap().1
}

#[aoc_runner_derive::aoc(day14, part1)]
// Excellent learning experience this one, bcause I decided to leanrn nom for this one.
pub fn solve1(input: &str) -> u64 {
    let commands = parse(input);
    let mut state = State::new();
    for command in commands {
        state.command(command);
    }
    state.sum()
}

#[aoc_runner_derive::aoc(day14, part2)]
pub fn solve2(input: &str) -> u64 {
    parse(input);
    todo!();
}

#[derive(Clone, Eq, PartialEq)]
struct Mask {
    active: u64,
    btmask: u64,
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mask")
            .field("active", &format_args!("{:064b}", self.active))
            .field("btmask", &format_args!("{:064b}", self.btmask))
            .finish()
    }
}

impl Mask {
    fn from_str(inmask: &str) -> Self {
        assert!(inmask.len() <= 64);

        let mut active: u64 = 0;
        let mut mask: u64 = 0;
        for m in inmask.chars() {
            match m {
                '1' => {
                    active |= 1;
                    mask |= 1;
                }
                '0' => {
                    active |= 1;
                }
                'X' => {}
                _ => panic!(),
            }
            active = active << 1;
            mask = mask << 1;
        }

        active = active >> 1;
        mask = mask >> 1;

        Mask {
            active,
            btmask: mask,
        }
    }

    fn mask(&self, val: u64) -> u64 {
        let mut ret = val;
        ret |= self.active & self.btmask;
        ret &= !(self.active & !self.btmask);
        ret
    }

    fn nom(inp: &str) -> nom::IResult<&str, Self> {
        use nom::{
            bytes::complete::{tag, take_while},
            character::complete::{char, space0},
            error::{Error, ErrorKind},
            sequence::tuple,
        };

        let (inp, _) = tuple((tag("mask"), space0, char('='), space0))(inp)?;
        let (inp, msk) = take_while(|c| (c == 'X') | (c == '0') | (c == '1'))(inp)?;

        if msk.len() > 64 {
            // return Err(nom::error::Error::new(msk, nom::error::ErrorKind::TooLarge));
            return Err(nom::Err::Error(Error::new(msk, ErrorKind::TooLarge)));
        }

        Ok((inp, Mask::from_str(msk)))
    }
}

struct State {
    mask: Mask,
    mem: HashMap<u64, u64>,
}

impl State {
    fn new() -> Self {
        Self {
            mask: Mask::from_str(""),
            mem: Default::default(),
        }
    }

    fn command(&mut self, cmd: Command) {
        match cmd {
            Command::Mask(mask) => {
                self.mask = mask;
            }
            Command::Set(Set { addr, val }) => {
                self.mem.insert(addr, self.mask.mask(val));
            }
        }
    }

    fn sum(&self) -> u64 {
        self.mem.values().sum()
    }
}

#[derive(Debug, Clone)]
enum Command {
    Set(Set),
    Mask(Mask),
}

impl Command {
    fn nom(inp: &str) -> nom::IResult<&str, Self> {
        Set::nom
            .map(Command::Set)
            .or(Mask::nom.map(Command::Mask))
            .parse(inp)
    }
}

#[derive(Debug, Clone)]
struct Set {
    addr: u64,
    val: u64,
}

impl Set {
    fn nom(inp: &str) -> nom::IResult<&str, Self> {
        use nom::{
            bytes::complete::tag,
            character::complete::{char, space0, u64},
            sequence::{delimited, preceded, tuple},
        };

        let (inp, (addr, val)) = tuple((
            delimited(tag("mem["), delimited(space0, u64, space0), char(']')),
            preceded(delimited(space0, char('='), space0), u64),
        ))(inp)?;

        Ok((inp, Set { addr, val }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask() {
        let m = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        dbg!(&m);
        eprintln!("{:064b}", 11);
        eprintln!("{:064b}\n{:064b}", m.mask(11), 73);
        assert_eq!(m.mask(11), 73);
        assert_eq!(m.mask(101), 101);
        assert_eq!(m.mask(0), 64);
    }

    #[test]
    fn nom() {
        assert_eq!(
            Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            Mask::nom("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
                .unwrap()
                .1
        );
        let inp = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 0
";
        parse(inp);
    }
}
