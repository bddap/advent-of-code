fn parse(input: &str) -> Vec<()> {
    input.lines().map(drop).collect()
}

#[aoc_runner_derive::aoc(day22, part1)]
pub fn solve1(input: &str) -> u32 {
    parse(input);
    todo!();
}

#[aoc_runner_derive::aoc(day22, part2)]
pub fn solve2(input: &str) -> u32 {
    parse(input);
    todo!();
}
