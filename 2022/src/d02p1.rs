#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 2, 1)]
fn run(inp: &str) -> anyhow::Result<usize> {
    let sum = inp
        .lines()
        .map(|line| {
            let mut c = line.chars();
            let a = Rps::from_char(c.next().unwrap());
            assert_eq!(c.next().unwrap(), ' ');
            let b = Rps::from_char(c.next().unwrap());
            score_round(a, b)
        })
        .sum();
    Ok(sum)
}

enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn from_char(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!(),
        }
    }
}

fn score_round(a: Rps, b: Rps) -> usize {
    match (a, b) {
        (Rps::Rock, Rps::Rock) => 3 + 1,
        (Rps::Rock, Rps::Paper) => 6 + 2,
        (Rps::Rock, Rps::Scissors) => 0 + 3,
        (Rps::Paper, Rps::Rock) => 0 + 1,
        (Rps::Paper, Rps::Paper) => 3 + 2,
        (Rps::Paper, Rps::Scissors) => 6 + 3,
        (Rps::Scissors, Rps::Rock) => 6 + 1,
        (Rps::Scissors, Rps::Paper) => 0 + 2,
        (Rps::Scissors, Rps::Scissors) => 3 + 3,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "A Y\nB X\nC Z";
        assert_eq!(run(inp).unwrap(), 15);
    }
}
