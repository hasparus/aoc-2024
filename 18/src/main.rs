mod ex1;
mod ex2;

use aoc_2024_lib::input_reader::read_input;
use std::error::Error;

static SIZE: u8 = 70;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = read_input("./inputs.md")?;

    let input = inputs.get_input("Input");
    let result1 = ex1::solve(&input.content, SIZE, 1024);
    println!("ex1\t{}", result1);

    let result2 = ex2::solve(&input.content, SIZE);
    println!("ex2\t{:?}", result2);

    Ok(())
}
