#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 2, 1)]
fn run(inp: &str) -> anyhow::Result<usize> {
    let sum = inp
        .lines()
        .map(|line| {
            let mut c = line.chars();
            let a = Rps::from_char(c.next().unwrap());
            assert_eq!(c.next().unwrap(), ' ');
            let b = Outcome::from_char(c.next().unwrap());
            score_round(a, b)
        })
        .sum();
    Ok(sum)
}

#[derive(Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Rps {
    fn score(self) -> usize {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!(),
        }
    }
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!(),
        }
    }
}

fn score_round(a: Rps, o: Outcome) -> usize {
    use Outcome::*;
    use Rps::*;

    let b = match (a, o) {
        (Rock, Lose) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,
        (Paper, Lose) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Lose) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, Win) => Rock,
    };

    b.score() + o.score()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "A Y\nB X\nC Z";
        assert_eq!(run(inp).unwrap(), 12);
    }
}
