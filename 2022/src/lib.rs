use std::{collections::HashMap, sync::Mutex};

use emergence::AoC as Aoc;
use linkme::distributed_slice;

#[distributed_slice]
pub static CHALLENGES: [Challenge] = [..];

pub struct Challenge {
    pub year: usize,
    pub day: usize,
    pub part: usize,
    pub run: fn(&str) -> String,
}

impl Challenge {
    pub fn run(&self) -> anyhow::Result<String> {
        let input = aoc_years()
            .lock()
            .unwrap()
            .entry(self.year)
            .or_insert_with(|| Aoc::new(self.year).map_err(Into::into))
            .as_ref()
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .read_or_fetch(self.day)?;
        Ok((self.run)(&input))
    }
}

#[lazy_fn::lazy_fn]
fn aoc_years() -> Mutex<HashMap<usize, anyhow::Result<Aoc>>> {
    Default::default()
}

macro_rules! challenge {
    (
        #[aoc($year:literal, $day:literal, $part:literal)]
        fn $funcname:ident($argname:ident: &str) -> $rettype:ty $imp:block
    ) => {
        mod $funcname {
            #[linkme::distributed_slice(crate::CHALLENGES)]
            static __: crate::Challenge = {
                let part = $part;
                assert!(part == 1 || part == 2);

                fn $funcname(a: &str) -> String {
                    super::$funcname(a).to_string()
                }

                crate::Challenge {
                    year: $year,
                    day: $day,
                    part,
                    run: $funcname,
                }
            };
        }

        fn $funcname($argname: &str) -> $rettype $imp
    };
}

mod d01p1;
mod d01p2;
mod d02p1;
mod d02p2;
mod d03p1;
mod d03p2;
mod d04p1;
mod d04p2;
mod d05p1;
mod d05p2;
mod d06p1;
mod d06p2;
mod d07p1;
mod d07p2;
mod d08p1;
mod d08p2;
mod d09p1;
mod d09p2;
mod d10p1;
mod d10p2;
mod d11p1;
mod d11p2;
mod d12p1;
mod d12p2;
mod d13p1;
mod d13p2;
mod d14p1;
mod d14p2;
mod d15p1;
mod d15p2;
mod d16p1;
mod d16p2;
mod d17p1;
mod d17p2;
mod d18p1;
mod d18p2;
mod d19p1;
mod d19p2;
mod d20p1;
mod d20p2;
mod d21p1;
mod d21p2;
mod d22p1;
mod d22p2;
mod d23p1;
mod d23p2;
mod d24p1;
mod d24p2;
mod d25p1;
mod d25p2;
