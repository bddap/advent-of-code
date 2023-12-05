use std::str::FromStr;

#[bddap_aoc::register(2023, 2, 1)]
fn solve(input: &str) -> String {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let in_bag = Collection {
        counts: vec![
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ],
    };
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<_>>();
    games
        .into_iter()
        .filter(|game| game.possible(&in_bag))
        .map(|game| game.game_id)
        .sum::<usize>()
        .to_string()
}

struct Game {
    game_id: usize,
    rounds: Vec<Collection>,
}

impl Game {
    fn possible(&self, in_bag: &Collection) -> bool {
        self.rounds.iter().all(|round| round.is_subset_of(in_bag))
    }
}

impl FromStr for Game {
    type Err = &'static str;

    // parse a single line
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let game_id = parts
            .next()
            .unwrap()
            .strip_prefix("Game ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let rounds = parts
            .next()
            .unwrap()
            .split("; ")
            .map(|round| round.parse::<Collection>().unwrap())
            .collect();
        assert!(parts.next().is_none());
        Ok(Self { game_id, rounds })
    }
}

struct Collection {
    counts: Vec<(String, usize)>,
}

impl Collection {
    fn is_subset_of(&self, other: &Self) -> bool {
        for (color, number) in &self.counts {
            if let Some((_other_color, other_number)) =
                other.counts.iter().find(|(c, _)| c == color)
            {
                if other_number < number {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

impl FromStr for Collection {
    type Err = &'static str;

    // parse a single line
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let counts = s
            .split(", ")
            .map(|count| {
                let mut parts = count.split(' ');
                let number = parts.next().unwrap().parse::<usize>().unwrap();
                let color = parts.next().unwrap().to_string();
                (color, number)
            })
            .collect();
        Ok(Self { counts })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve.run(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "8"
        );
    }
}
