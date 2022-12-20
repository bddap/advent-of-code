use std::{cmp::Ordering, iter::once};

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 13, 1)]
fn run(inp: &str) -> usize {
    inp.split("\n\n")
        .map(parse_pair)
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .filter(|(_i, (a, b))| a <= b)
        .map(|(i, _p)| i)
        .sum()
}

fn parse_pair(inp: &str) -> (Tree, Tree) {
    let mut lines = inp.lines();
    let a = Tree::from_json(serde_json::from_str(lines.next().unwrap()).unwrap());
    let b = Tree::from_json(serde_json::from_str(lines.next().unwrap()).unwrap());
    assert!(lines.next().is_none());
    (a, b)
}

enum Tree {
    Split(Vec<Tree>),
    Leaf(usize),
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Split(arg0) => f.debug_list().entries(arg0).finish(),
            Self::Leaf(arg0) => write!(f, "{}", arg0),
        }
    }
}

impl Tree {
    fn from_json(js: serde_json::Value) -> Self {
        match js {
            serde_json::Value::Number(n) => Tree::Leaf(n.as_u64().unwrap().try_into().unwrap()),
            serde_json::Value::Array(a) => {
                Tree::Split(a.into_iter().map(Tree::from_json).collect())
            }
            _ => panic!(),
        }
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Tree {}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Tree) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Tree::Split(a), Tree::Split(b)) => a.cmp(b),
            (Tree::Split(a), b @ Tree::Leaf(_)) => cmp_tree_iter(a.iter(), once(b)),
            (a @ Tree::Leaf(_), Tree::Split(b)) => cmp_tree_iter(once(a), b.iter()),
            (Tree::Leaf(a), Tree::Leaf(b)) => a.cmp(b),
        }
    }
}

fn cmp_tree_iter<'a>(
    mut a: impl Iterator<Item = &'a Tree>,
    mut b: impl Iterator<Item = &'a Tree>,
) -> Ordering {
    loop {
        let aa = a.next();
        let bb = b.next();
        match aa.cmp(&bb) {
            Ordering::Equal => {}
            o => return o,
        }
        if aa.is_none() {
            return Ordering::Equal;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INP: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test() {
        assert_eq!(run(INP), 13);
    }

    #[test]
    fn test2() {
        let expected = vec![true, true, false, true, false, true, false, false];
        let got: Vec<bool> = INP
            .split("\n\n")
            .map(parse_pair)
            .map(|(a, b)| a <= b)
            .collect();
        assert_eq!(expected, got);
    }
}
