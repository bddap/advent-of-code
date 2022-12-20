use std::{cmp::Ordering, iter::once};

use itertools::Itertools;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 13, 2)]
fn run(inp: &str) -> usize {
    let mut packets = inp
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect_vec();

    let extra_a = Tree::Split([Tree::Split([Tree::Leaf(2)].into())].into());
    let extra_b = Tree::Split([Tree::Split([Tree::Leaf(6)].into())].into());
    packets.push(extra_a.clone());
    packets.push(extra_b.clone());
    packets.sort();

    let idx_a = packets.binary_search(&extra_a).unwrap() + 1;
    let idx_b = packets.binary_search(&extra_b).unwrap() + 1;

    idx_a * idx_b
}

fn parse_line(line: &str) -> Tree {
    Tree::from_json(serde_json::from_str(line).unwrap())
}

#[derive(Clone)]
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
        assert_eq!(run(INP), 140);
    }
}
