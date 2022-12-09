#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 3, 1)]
fn run(inp: &str) -> usize {
    inp.lines()
        .map(|line| {
            let h = line.len() / 2;
            let a = &line[..h];
            let b = &line[h..];
            for c in a.chars() {
                if b.contains(c) {
                    return c;
                }
            }
            panic!()
        })
        .map(score)
        .sum()
}

fn score(c: char) -> usize {
    match c {
        c if ('a'..='z').contains(&c) => c as usize - 'a' as usize + 1,
        c if ('A'..='Z').contains(&c) => c as usize - 'A' as usize + 1 + 26,
        _ => panic!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(score('a'), 1);
        assert_eq!(score('A'), 27);
        assert_eq!(score('B'), 28);
        assert_eq!(score('s'), 19);
    }
}
