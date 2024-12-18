mod ex1;
use std::error::Error;

use aoc_2024_lib::input_reader::read_input;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = read_input("./inputs.md")?;

    for input in inputs.sections {
        let result = ex1::solve(&input.content);
        println!("ex1 {}: {}", input.name, result);
    }

    Ok(())
}
