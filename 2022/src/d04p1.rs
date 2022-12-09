use std::ops::RangeInclusive;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 4, 1)]
fn run(inp: &str) -> usize {
    inp.lines()
        .filter(|line| {
            let mut iter = line.split(',');
            let a = parserange(iter.next().unwrap());
            let b = parserange(iter.next().unwrap());
            contains(&a, &b) || contains(&b, &a)
        })
        .count()
}

fn contains(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn parserange(r: &str) -> RangeInclusive<usize> {
    let mut iter = r.split('-');
    let start: usize = iter.next().unwrap().parse().unwrap();
    let end: usize = iter.next().unwrap().parse().unwrap();
    assert!(iter.next().is_none());
    start..=end
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            run("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8"),
            2,
        );
    }
}
