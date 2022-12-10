use itertools::Itertools;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 6, 1)]
fn run(inp: &str) -> usize {
    let window_size = 4;
    let vc = inp.chars().collect_vec();
    vc.windows(window_size)
        .enumerate()
        .find(|(_i, window)| all_uniq(window))
        .unwrap()
        .0
        + window_size
}

fn all_uniq<T: Ord>(ts: &[T]) -> bool {
    let Some((head, rest)) = ts.split_first() else {
        return true;
    };
    if rest.contains(head) {
        return false;
    }
    all_uniq(rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
