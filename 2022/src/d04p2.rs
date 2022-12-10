use std::ops::RangeInclusive;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 4, 2)]
fn run(inp: &str) -> usize {
    inp.lines()
        .filter(|line| {
            let mut iter = line.split(',');
            let a = parserange(iter.next().unwrap());
            let b = parserange(iter.next().unwrap());
            overlaps(&a, &b)
        })
        .count()
}

fn overlaps(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
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
            4,
        );
    }
}
