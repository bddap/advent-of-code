// I use std::PathBuf for this challenge so this might yeild incorrect results on windows.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use itertools::Itertools;

const TOTAL: usize = 70000000;
const NEED: usize = 30000000;

#[macro_rules_attribute::apply(challenge)]
#[aoc(2022, 7, 2)]
fn run(inp: &str) -> usize {
    let mut trav = Traverse {
        pwd: "/".into(),
        file_sizes: Default::default(),
        dirs: Default::default(),
    };
    for line in inp.lines() {
        let trans = Transition::from_line(line);
        trav.run(&trans);
    }
    let consumed = trav.sizeof_dir(&"/".into());
    let free = TOTAL - consumed;
    let must_remove = NEED.checked_sub(free).unwrap();

    let mut dir_sizes = trav
        .dirs
        .iter()
        .map(|dir| (trav.sizeof_dir(dir), dir))
        .collect_vec();
    dir_sizes.sort();
    let idx = dir_sizes
        .binary_search_by(|(size, _)| size.cmp(&must_remove))
        .unwrap_or_else(|e| e);
    dir_sizes[idx].0
}

struct Traverse {
    pwd: PathBuf,
    file_sizes: HashMap<PathBuf, usize>,
    dirs: HashSet<PathBuf>,
}

impl Traverse {
    fn run(&mut self, transition: &Transition) {
        match transition {
            Transition::CdIn(dir) => {
                self.pwd = self.pwd.join(dir);
            }
            Transition::CdOut => {
                assert!(self.pwd.pop());
            }
            Transition::CdRoot => {
                self.pwd = "/".into();
            }
            Transition::Ls => {}
            Transition::Dir(d) => {
                self.dirs.insert(self.pwd.join(d));
            }
            Transition::File { size, name } => {
                self.file_sizes.insert(self.pwd.join(name), *size);
            }
        }
    }

    fn sizeof_dir(&self, dir: &PathBuf) -> usize {
        self.file_sizes
            .iter()
            .filter_map(|(name, size)| name.starts_with(dir).then_some(size))
            .sum()
    }
}

enum Transition<'a> {
    CdIn(&'a str),
    CdOut,
    CdRoot,
    Ls,
    Dir(&'a str),
    File { size: usize, name: &'a str },
}

impl<'a> Transition<'a> {
    fn from_line(line: &'a str) -> Self {
        let mut split = line.split(' ');
        match (split.next(), split.next(), split.next(), split.next()) {
            (Some("$"), Some("cd"), Some("/"), None) => Self::CdRoot,
            (Some("$"), Some("cd"), Some(".."), None) => Self::CdOut,
            (Some("$"), Some("cd"), Some(dirname), None) => Self::CdIn(dirname),
            (Some("$"), Some("ls"), None, None) => Self::Ls,
            (Some("dir"), Some(dirname), None, None) => Self::Dir(dirname),
            (Some(size), Some(filename), None, None) => Self::File {
                size: size.parse().unwrap(),
                name: filename,
            },
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let inp = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(run(inp), 24933642);
    }
}
