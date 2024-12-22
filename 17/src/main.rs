mod ex1;
mod types;

use aoc_2024_lib::input_reader::read_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = read_input("./inputs.md")?;

    for input in inputs.sections {
        let result1 = ex1::solve(&input.content);
        println!("{}\t{}", input.name, result1);
    }

    Ok(())
}
