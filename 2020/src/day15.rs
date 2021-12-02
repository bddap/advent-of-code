fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc_runner_derive::aoc(day15, part1)]
pub fn solve1(input: &str) -> usize {
    let input = parse(input);
    let iter = Iter::from_list(&input);
    input.into_iter().chain(iter).skip(2019).next().unwrap()
}

// there's probably a faster clever way to solve this one
// this method is bruteish (732.538539ms)
#[aoc_runner_derive::aoc(day15, part2)]
pub fn solve2(input: &str) -> usize {
    let input = parse(input);
    let iter = Iter::from_list(&input);
    input
        .into_iter()
        .chain(iter)
        .skip(30000000 - 1)
        .next()
        .unwrap()
}

struct SmallUintMap {
    map: Vec<Option<usize>>,
}

impl SmallUintMap {
    fn new() -> Self {
        SmallUintMap { map: Vec::new() }
    }

    fn insert(&mut self, k: usize, v: usize) {
        if self.map.len() <= k {
            let toadd = (k * 2) - self.map.len() + 1;
            self.map.extend((0..toadd).map(|_| None));
        }
        self.map[k] = Some(v);
    }

    fn get(&self, k: usize) -> Option<usize> {
        self.map.get(k).cloned().flatten()
    }
}

struct Iter {
    prev: usize,
    history: SmallUintMap,
    idx: usize,
}

impl Iter {
    fn from_list(v: &[usize]) -> Self {
        assert!(!v.is_empty());

        let mut history = SmallUintMap::new();

        for (i, n) in v[0..v.len() - 1].iter().enumerate() {
            history.insert(*n, i);
        }

        Iter {
            prev: *v.last().unwrap(),
            history,
            idx: v.len() - 1,
        }
    }
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        let ret = self
            .history
            .get(self.prev)
            .map(|n| self.idx - n - 1)
            .unwrap_or(0);
        self.history.insert(self.prev, self.idx - 1);
        self.prev = ret;
        Some(ret)
    }
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

    #[test]
    #[ignore] // takes too long
    fn tsolve2() {
        assert_eq!(solve2("0,3,6"), 175594);
        assert_eq!(solve2("1,3,2"), 2578);
        assert_eq!(solve2("2,1,3"), 3544142);
        assert_eq!(solve2("1,2,3"), 261214);
        assert_eq!(solve2("2,3,1"), 6895259);
        assert_eq!(solve2("3,2,1"), 18);
        assert_eq!(solve2("3,1,2"), 362);
    }
}
