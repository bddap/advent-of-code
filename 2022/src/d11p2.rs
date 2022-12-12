use std::collections::VecDeque;

use itertools::Itertools;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 11, 2)]
fn run(inp: &str) -> usize {
    let mut troop = inp.split("\n\n").map(Munkae::parse).collect_vec();
    let troop_len = troop.len();

    // this it the trick to part 2
    let common_multiple: usize = troop.iter().map(|monk| monk.test.div).product();

    for _ in 0..10000 {
        for i in 0..(troop.len()) {
            while let Some((where_to, item)) = troop[i].pop() {
                let item = item % common_multiple;
                troop[where_to].holding.push_back(item);
            }
        }
    }

    let (_, nth, greater) =
        troop.select_nth_unstable_by_key(troop_len - 2, |monk| monk.items_inspected);
    assert_eq!(greater.len(), 1);

    nth.items_inspected * greater[0].items_inspected
}

struct Munkae {
    holding: VecDeque<usize>,
    op: Op,
    test: Test,
    items_inspected: usize,
}

impl Munkae {
    fn parse(inp: &str) -> Munkae {
        let mut inp = inp.lines();

        inp.next().unwrap();

        let starting_items = inp.next().unwrap();
        let starting_items = starting_items.strip_prefix("  Starting items: ").unwrap();
        let holding = starting_items
            .split(", ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let op = Op::parse(inp.next().unwrap());

        let test = Test::from_lines(
            inp.next().unwrap(),
            inp.next().unwrap(),
            inp.next().unwrap(),
        );

        assert!(inp.next().is_none());

        Self {
            holding,
            op,
            test,
            items_inspected: 0,
        }
    }

    fn pop(&mut self) -> Option<(usize, usize)> {
        let item = self.holding.pop_front()?;
        self.items_inspected += 1;
        let item = self.op.run(item);
        let where_to_throw = self.test.where_to(item);
        Some((where_to_throw, item))
    }
}

enum Op {
    Add(Val, Val),
    Mul(Val, Val),
}

impl Op {
    fn parse(op: &str) -> Self {
        let op = op.strip_prefix("  Operation: new = ").unwrap();
        let mut op = op.split(" ");
        let a = Val::parse(op.next().unwrap());
        let f = op.next().unwrap();
        let b = Val::parse(op.next().unwrap());
        match f {
            "+" => Op::Add(a, b),
            "*" => Op::Mul(a, b),
            _ => panic!(),
        }
    }

    fn run(&self, inp: usize) -> usize {
        match self {
            Op::Add(a, b) => a.run(inp) + b.run(inp),
            Op::Mul(a, b) => a.run(inp) * b.run(inp),
        }
    }
}

enum Val {
    Const(usize),
    Input,
}

impl Val {
    fn parse(inp: &str) -> Self {
        match inp {
            "old" => Self::Input,
            n => Self::Const(n.parse().unwrap()),
        }
    }

    fn run(&self, inp: usize) -> usize {
        match self {
            Val::Const(c) => *c,
            Val::Input => inp,
        }
    }
}

struct Test {
    div: usize,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn from_lines(a: &str, b: &str, c: &str) -> Self {
        let div = a.strip_prefix("  Test: divisible by ").unwrap();
        let tru = b.strip_prefix("    If true: throw to monkey ").unwrap();
        let fal = c.strip_prefix("    If false: throw to monkey ").unwrap();
        let div = div.parse().unwrap();
        let tru = tru.parse().unwrap();
        let fal = fal.parse().unwrap();
        Test {
            div,
            if_true: tru,
            if_false: fal,
        }
    }

    fn where_to(&self, item: usize) -> usize {
        if item % self.div == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(run(inp), 2713310158);
    }
}
