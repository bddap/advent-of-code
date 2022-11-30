use advent_of_code_2022::{Challenge, CHALLENGES};
use clap::Parser;

/// Download and run advent of code challenges.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    year: Vec<usize>,
    #[arg(short, long)]
    day: Vec<usize>,
    #[arg(short, long)]
    part: Vec<usize>,
}

fn main() {
    let args = Args::parse();

    let mut todos: Vec<&Challenge> = CHALLENGES
        .iter()
        .filter(|c| args.year.contains(&c.year) || args.year.is_empty())
        .filter(|c| args.day.contains(&c.day) || args.day.is_empty())
        .filter(|c| args.part.contains(&c.part) || args.part.is_empty())
        .collect();

    if todos.is_empty() {
        eprintln!("No matching challenges.");
    }

    todos.sort_by_key(|c| (c.year, c.day, c.part));

    for challenge in todos {
        print!("{} {}.{} ", challenge.year, challenge.day, challenge.part);
        match challenge.run() {
            Ok(r) => println!("{r}"),
            Err(e) => {
                println!("error {:?}", format!("{e}"));
            }
        }
    }
}
