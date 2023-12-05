#[bddap_aoc::register(2023, 1, 2)]
fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let line = parse_line(line);
            let first = line.first().unwrap();
            let last = line.last().unwrap();
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn parse_line(line: &str) -> Vec<usize> {
    line.char_indices()
        .filter_map(|(i, _)| starts_with_digit(&line[i..]))
        .collect()
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

fn starts_with_digit(inp: &str) -> Option<usize> {
    for (spell, digit) in SPELL_DIGITS {
        if inp.starts_with(spell) {
            return Some(*digit);
        }
    }
    None
}

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
