use anyhow::Result;
use aoc_2024_lib::input_reader::{self, Input};

mod ex1;
mod ex2;
mod parser;
mod trie;

fn main() -> Result<()> {
    let inputs = input_reader::read_input("./inputs.md")?;

    for Input { name, content } in inputs.sections {
        let result1 = ex1::solve(&content)?;
        let result2 = ex2::solve(&content)?;
        println!("ex1\t{name}\t{result1}\t{result2}");
    }

    Ok(())
}
