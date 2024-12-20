use anyhow::Result;
use aoc_2024_lib::input_reader::{self, Input};

mod cell;
mod cheat;
mod ex1;
mod ex2;
mod parse_board;
mod test_utils;

fn main() -> Result<()> {
    let inputs = input_reader::read_input("./inputs.md")?;

    for Input { name, content } in inputs.sections {
        let result1 = ex1::solve(&content, 100);
        let result2 = ex2::solve(&content, 100);
        println!("ex1\t{name}\t{result1}\t{result2}");
    }

    Ok(())
}
