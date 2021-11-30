pub use part1::solve1;
pub use part2::solve2;

mod part1 {
    use std::collections::HashMap;

    use nom::Parser;

    fn parse(input: &str) -> Vec<Command> {
        use nom::{character::complete::char, multi::separated_list0};
        separated_list0(char('\n'), Command::nom)(input).unwrap().1
    }

    #[aoc_runner_derive::aoc(day14, part1)]
    // Excellent learning experience this one, bcause I decided to learn nom for this one.
    pub fn solve1(input: &str) -> u64 {
        let commands = parse(input);
        let mut state = State::new();
        for command in commands {
            state.command(command);
        }
        state.sum()
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
}

mod part2 {
    use std::collections::HashMap;

    use nom::Parser;

    fn parse(input: &str) -> Vec<Command> {
        use nom::{character::complete::char, multi::separated_list0};
        separated_list0(char('\n'), Command::nom)(input).unwrap().1
    }

    #[aoc_runner_derive::aoc(day14, part2)]
    pub fn solve2(input: &str) -> u64 {
        let writes = as_writes(parse(input));

        // for i in 0..writes.len() {
        //     for j in (i + 1)..writes.len() {
        //         assert!(!writes[i].overlaps(&writes[j]));
        //         // this assertion fails, there are overlaps
        //     }
        // }

        let max_writes: usize = writes.iter().map(Write::would_write).sum();
        assert!(
            max_writes < 10_000_000,
            "don't need to be super clever here, brute force won't consume too much memory",
        );

        let mut mem = HashMap::<u64, u64>::new();
        for write in writes {
            write.write_all(&mut mem);
        }

        mem.values().sum()
    }

    #[derive(Clone, Eq, PartialEq)]
    struct Write {
        addr: u64,
        val: u64,
        floating: u64,
    }

    impl Write {
        // fn overlaps(&self, other: &Self) -> bool {
        //     let mut similar = !(self.addr ^ other.addr);
        //     similar |= self.floating;
        //     similar |= other.floating;
        //     similar != 0
        // }

        // how many memory addresses would this write effect
        fn would_write(&self) -> usize {
            2usize.pow(self.floating.count_ones())
        }

        fn write_all(&self, mem: &mut HashMap<u64, u64>) {
            if self.floating.count_ones() == 0 {
                mem.insert(self.addr, self.val);
                return;
            }
            let first_floating = self.floating.leading_zeros() as u8;
            let floating = set_bit(self.floating, first_floating, false);
            Write {
                addr: set_bit(self.addr, first_floating, true),
                val: self.val,
                floating,
            }
            .write_all(mem);
            Write {
                addr: set_bit(self.addr, first_floating, false),
                val: self.val,
                floating,
            }
            .write_all(mem);
        }
    }

    fn set_bit(bits: u64, index: u8, val: bool) -> u64 {
        if val {
            bits | (1 << (63 - index))
        } else {
            bits & !(1 << (63 - index))
        }
    }

    #[derive(Clone, Eq, PartialEq, Debug)]
    struct Mask {
        set: u64,
        floating: u64,
    }

    impl Mask {
        fn from_str(inmask: &str) -> Self {
            assert!(inmask.len() <= 64);

            let mut set: u64 = 0;
            let mut floating: u64 = 0;
            for m in inmask.chars() {
                match m {
                    '1' => {
                        set |= 1;
                    }
                    '0' => {}
                    'X' => {
                        floating |= 1;
                    }
                    _ => panic!(),
                }
                set = set << 1;
                floating = floating << 1;
            }

            set = set >> 1;
            floating = floating >> 1;

            Mask { set, floating }
        }

        fn write(&self, cmd: Set) -> Write {
            let val = cmd.val;
            let addr = cmd.addr | self.set;
            Write {
                val,
                addr,
                floating: self.floating,
            }
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

    fn as_writes(commands: Vec<Command>) -> Vec<Write> {
        let mut ret = Vec::new();
        let mut mask = Mask::from_str("");

        for c in commands {
            match c {
                Command::Set(s) => ret.push(mask.write(s)),
                Command::Mask(m) => {
                    mask = m;
                }
            }
        }

        ret
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
        fn tset_bit() {
            assert_eq!(set_bit(0b0, 63, true), 0b1);
            assert_eq!(set_bit(0b1, 62, true), 0b11);
            let bits = 0b10000u64;
            assert_eq!(set_bit(bits, bits.leading_zeros() as u8, false), 0);
        }

        #[test]
        fn day2() {
            let inp = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
            assert_eq!(solve2(inp), 208);
        }
    }
}
