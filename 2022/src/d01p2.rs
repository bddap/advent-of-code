#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 1, 2)]
fn run(inp: &str) -> usize {
    let mut ls: Vec<usize> = inp
        .split("\n\n")
        .map(|section| {
            section
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();
    ls.sort();
    ls.reverse();
    let top3 = ls.get(0..3).expect("not enough input");
    top3.iter().sum::<usize>()
}
