#[derive(Debug)]
struct BusShedule {
    _you_arrive: u64,
    departures: Vec<Option<u64>>,
}

fn parse(input: &str) -> BusShedule {
    let mut lines = input.lines();
    let _you_arrive = lines.next().unwrap().parse().unwrap();
    let departures = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| {
            if n == "x" {
                None
            } else {
                Some(n.parse::<u64>().unwrap())
            }
        })
        .collect();
    BusShedule {
        _you_arrive,
        departures,
    }
}

#[aoc_runner_derive::aoc(day13, part2)]
pub fn solve2(input: &str) -> u64 {
    let constraints: Vec<(u64, u64)> = parse(input)
        .departures
        .iter()
        .enumerate()
        .filter_map(|(i, md)| md.map(|d| (i as u64, d)))
        .collect();

    let sat = |t: u64, offset: u64, departure: u64| ((t + offset) % departure) == 0;

    let mut t = 0;
    let mut step = 1;
    for (offset, departure) in &constraints {
        while !sat(t, *offset, *departure) {
            t += step;
        }
        step *= departure;
    }

    for (offset, departure) in &constraints {
        assert!(sat(t, *offset, *departure));
    }

    t
}
