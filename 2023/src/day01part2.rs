#[bddap_aoc::register(2023, 1, 2)]
fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let iter = line.chars().filter(|c| c.is_ascii_digit());
            let first = iter.clone().next().unwrap();
            let last = iter.last().unwrap();
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

const SPELL_DIGITS: &[(&str, usize)] = &[
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve.run(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "281"
        );
    }
}
