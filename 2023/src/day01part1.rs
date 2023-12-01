#[bddap_aoc::register(2023, 1, 1)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            solve.run(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            "142"
        );
    }
}
