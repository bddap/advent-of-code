#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 1, 1)]
fn run(inp: &str) -> anyhow::Result<usize> {
    inp.split("\n\n")
        .map(|section| {
            section
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .ok_or(anyhow::anyhow!("no input numbers"))
}
