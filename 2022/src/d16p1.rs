// use std::collections::HashMap;

// use aoc_parse::{parser, prelude::*};
// use petgraph::Graph;

// #[macro_rules_attribute::apply(challenge)]
// #[aoc(2022, 16, 1)]
// fn run(inp: &str) -> usize {
//     let g = parse(inp);
//     0
// }

// fn parse(inp: &str) -> Graph<usize, ()> {
//     let ident = parser!(upper upper);
//     let ln = parser![
//         "Valve "
//         a:ident
//         " has flow rate="
//         b:usize
//         {
//             "; tunnels lead to valves ",
//             "; tunnel leads to valve "
//         }
//         c:repeat_sep(ident, ", ")
//         => (a, b, c)
//     ];
//     let p = parser!(lines(ln));
//     let ip: Vec<((char, char), usize, Vec<(char, char)>)> = p.parse(inp).unwrap();
//     let mut ret: Graph<usize, ()> = Default::default();
//     let name_to_id: HashMap<(char, char), _> = ip
//         .iter()
//         .map(|(iden, weight, _)| (*iden, ret.add_node(*weight)))
//         .collect();
//     for (iden, _weight, connections) in ip {
//         let iden = name_to_id[&iden];
//         for connection in connections {
//             ret.add_edge(iden, name_to_id[&connection], ());
//         }
//     }
//     ret
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     const INP: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
// Valve BB has flow rate=13; tunnels lead to valves CC, AA
// Valve CC has flow rate=2; tunnels lead to valves DD, BB
// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
// Valve EE has flow rate=3; tunnels lead to valves FF, DD
// Valve FF has flow rate=0; tunnels lead to valves EE, GG
// Valve GG has flow rate=0; tunnels lead to valves FF, HH
// Valve HH has flow rate=22; tunnel leads to valve GG
// Valve II has flow rate=0; tunnels lead to valves AA, JJ
// Valve JJ has flow rate=21; tunnel leads to valve II";

//     #[test]
//     fn test() {
//         assert_eq!(run(INP), 1651);
//     }
// }
