fn parse(input: &str) -> Vec<u64> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc_runner_derive::aoc(day1, part1)]
pub fn solve1(input: &str) -> usize {
    let input = parse(input);
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count()
}

#[aoc_runner_derive::aoc(day1, part2)]
pub fn solve2(input: &str) -> usize {
    let input = parse(input);
    let input: Vec<u64> = input.windows(3).map(|w| w.iter().clone().sum()).collect();
    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(a, b)| b > a)
        .count()
}
