use itertools::Itertools;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 5, 2)]
fn run(inp: &str) -> String {
    let mut inp = inp.split("\n\n");
    let mut towers = parse_towers(inp.next().unwrap());
    let instructions = parse_instructions(inp.next().unwrap());
    assert!(inp.next().is_none());

    for [mov, from, to] in instructions {
        let from = &mut towers[from];
        let len = from.len();
        let moving = from.drain((len - mov)..len).collect_vec();
        assert_eq!(moving.len(), mov);
        towers[to].extend(moving);
    }

    towers.iter().map(|t| t.last().unwrap()).collect()
}

fn parse_towers(inp: &str) -> Vec<Vec<char>> {
    let pre = inp
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut ret = rotate(pre);
    let w = ['[', ']', ' '];
    for r in ret.iter_mut() {
        r.retain(|a| !w.contains(a));
    }
    ret.retain(|l| !l.is_empty());
    ret
}

fn parse_instructions(inp: &str) -> Vec<[usize; 3]> {
    inp.lines()
        .map(|line| {
            let mut sp = line.split(' ');
            assert_eq!(sp.next().unwrap(), "move");
            let mov: usize = sp.next().unwrap().parse().unwrap();
            assert_eq!(sp.next().unwrap(), "from");
            let from: usize = sp.next().unwrap().parse().unwrap();
            assert_eq!(sp.next().unwrap(), "to");
            let to: usize = sp.next().unwrap().parse().unwrap();
            [mov, from - 1, to - 1]
        })
        .collect_vec()
}

fn rotate<T>(lines: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let len = lines.iter().map(|l| l.len()).max().unwrap();
    (0..len)
        .map(|line| {
            (0..lines.len())
                .filter_map(|row| {
                    let i = lines.len().checked_sub(row + 1)?;
                    lines.get(i)?.get(line).cloned()
                })
                .collect_vec()
        })
        .collect_vec()
}
