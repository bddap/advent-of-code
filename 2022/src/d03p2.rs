#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 3, 2)]
fn run(inp: &str) -> anyhow::Result<usize> {
    let ret = inp
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|line_triple| {
            for c in line_triple[0].chars() {
                if line_triple[1].contains(c) && line_triple[2].contains(c) {
                    return c;
                }
            }
            panic!()
        })
        .map(score)
        .sum();
    Ok(ret)
}

fn score(c: char) -> usize {
    match c {
        c if ('a'..='z').contains(&c) => c as usize - 'a' as usize + 1,
        c if ('A'..='Z').contains(&c) => c as usize - 'A' as usize + 1 + 26,
        _ => panic!(),
    }
}
