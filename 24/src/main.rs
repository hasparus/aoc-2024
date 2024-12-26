use std::env;

use anyhow::Result;
use aoc_2024_lib::input_reader::{self, Input};

mod ex1;
mod ex2;
mod parse_input;

fn main() -> Result<()> {
    let inputs = input_reader::read_input("./inputs.md")?;

    let args = env::args().collect::<Vec<_>>();
    let part = args
        .get(1)
        .expect("Please provide the part number to run as first argument.");

    for Input { name, content } in inputs.sections {
        if part == "1" {
            let result = ex1::solve(&content);
            println!("ex1\t{name}\t{result}");
        } else if part == "2" {
            let result = ex2::solve(&content);
            println!("ex2\t{name}\t{result}");
        } else {
            panic!("Invalid part number: {part}");
        }
    }

    Ok(())
}
