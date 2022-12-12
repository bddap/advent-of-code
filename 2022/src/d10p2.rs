use std::{fmt::Display, iter::once};

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 10, 2)]
fn run(inp: &str) -> String {
    let signal = signal(deltas_per_cycle(inp));
    let mut screen = Screen::fresh();
    screen.write(signal);
    screen.to_string()
}

const SCREENH: usize = 6;
const SCREENW: usize = 40;

struct Screen {
    pixels: [[bool; SCREENW]; SCREENH],
}

impl Screen {
    fn fresh() -> Self {
        let pixels = [[false; SCREENW]; SCREENH];
        Self { pixels }
    }

    fn write(&mut self, mut signal: impl Iterator<Item = isize>) {
        for row in self.pixels.iter_mut() {
            for (col, pixel) in row.iter_mut().enumerate() {
                let sprite_pos = signal.next().unwrap();
                if (col as isize - sprite_pos).abs() <= 1 {
                    *pixel = true;
                }
            }
        }
        signal.next();
        assert!(signal.next().is_none());
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for row in &self.pixels {
            for pixel in row {
                write!(f, "{}", if *pixel { '#' } else { ' ' })?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn signal(deltas: impl Iterator<Item = isize>) -> impl Iterator<Item = isize> {
    let mut sum = 1;
    once(1).chain(deltas.map(move |d| {
        sum += d;
        sum
    }))
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

        let want = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        .replace('.', " ");
        eprintln!("want\n{}", want);
        eprintln!("got\n{}", run(inp));
        assert_eq!(want, run(inp));
    }
}
