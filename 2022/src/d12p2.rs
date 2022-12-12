use itertools::Itertools;
use petgraph::{
    algo::dijkstra::dijkstra, matrix_graph::DiMatrix, matrix_graph::NodeIndex,
    visit::IntoNodeReferences,
};

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 12, 2)]
fn run(inp: &str) -> usize {
    let t = Terrain::parse(inp);
    t.solve()
}

struct Terrain {
    map: DiMatrix<char, ()>,
    dest: NodeIndex,
}

impl Terrain {
    fn parse(inp: &str) -> Terrain {
        let mut map = DiMatrix::<char, ()>::new();
        let m: Vec<Vec<NodeIndex>> = inp
            .lines()
            .map(|line| line.chars().map(|c| map.add_node(c)).collect_vec())
            .collect_vec();

        let adjacent_nodes = adjacent_indices(m.len(), m[0].len())
            .map(|[[ai, aj], [bi, bj]]| [m[ai][aj], m[bi][bj]]);

        for [aidx, bidx] in adjacent_nodes {
            let a = from_char(*map.node_weight(aidx));
            let b = from_char(*map.node_weight(bidx));
            if a + 1 >= b {
                // swapped bidx and aidx for part 2
                map.add_edge(bidx, aidx, ());
            }
        }

        let dest = map.node_references().find(|(_i, &c)| c == 'E').unwrap().0;
        Self { map, dest }
    }

    fn solve(mut self) -> usize {
        // add a psuedo node that to represent a success condition
        let success = self.map.add_node('0');

        let ays = self
            .map
            .node_references()
            .filter_map(|(i, &a)| (a == 'a').then_some(i))
            .collect_vec();
        for a in ays {
            self.map.add_edge(a, success, ());
        }

        dijkstra(&self.map, self.dest, Some(success), |_| 1usize)
            .get(&success)
            .unwrap()
            .clone()
            - 1
    }
}

fn adjacent_indices(hei: usize, wid: usize) -> impl Iterator<Item = [[usize; 2]; 2]> {
    let hors = (0..hei)
        .cartesian_product(0..(wid - 1))
        .map(|(h, w)| [[h, w], [h, w + 1]]);
    let verts = (0..(hei - 1))
        .cartesian_product(0..wid)
        .map(|(h, w)| [[h, w], [h + 1, w]]);
    let all = hors.chain(verts);
    all.clone().map(|[a, b]| [b, a]).chain(all)
}

fn from_char(mut c: char) -> u8 {
    if c == 'S' {
        c = 'a';
    }
    if c == 'E' {
        c = 'z';
    }
    assert!(c >= 'a');
    assert!(c <= 'z');
    (c as u32 - 'a' as u32) as u8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        assert_eq!(run(inp), 29);
    }
}
