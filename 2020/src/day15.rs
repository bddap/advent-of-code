use std::collections::BTreeMap;

fn parse(input: &str) -> Vec<u64> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

struct Iter {
    prev: u64,
    history: BTreeMap<u64, u64>,
    idx: u64,
}

impl Iter {
    fn from_list(v: &[u64]) -> Self {
        assert!(!v.is_empty());

        let mut history = BTreeMap::<u64, u64>::new();

        assert_eq!(v[0..v.len() - 1].len(), v.len() - 1);
        for (i, n) in v[0..v.len() - 1].iter().enumerate() {
            history.insert(*n, i as u64);
        }

        Iter {
            prev: *v.last().unwrap(),
            history,
            idx: v.len() as u64 - 1,
        }
    }
}

impl Iterator for Iter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        let ret = self
            .history
            .get(&self.prev)
            .map(|n| self.idx - n - 1)
            .unwrap_or(0);
        self.history.insert(self.prev, self.idx - 1);
        self.prev = ret;
        Some(ret)
    }
}

#[aoc_runner_derive::aoc(day15, part1)]
pub fn solve1(input: &str) -> u64 {
    let input = parse(input);
    let iter = Iter::from_list(&input);

    input.into_iter().chain(iter).skip(2019).next().unwrap()
}

#[aoc_runner_derive::aoc(day15, part2)]
pub fn solve2(input: &str) -> u32 {
    parse(input);
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter3() {
        let mut it = Iter::from_list(&[0, 3, 6]);
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(0));
    }

    #[test]
    fn tsolve1() {
        solve1("0,3,6");
        assert_eq!(solve1("1,3,2"), 1);
        assert_eq!(solve1("2,1,3"), 10);
        assert_eq!(solve1("1,2,3"), 27);
        assert_eq!(solve1("2,3,1"), 78);
        assert_eq!(solve1("3,2,1"), 438);
        assert_eq!(solve1("3,1,2"), 1836);
    }
}
