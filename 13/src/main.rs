mod ex1;
mod ex2;
mod parse_input;

use aoc_2024_lib::input_reader;

fn main() -> Result<(), std::io::Error> {
    let inputs = input_reader::read_input("inputs.md")?;

    for input in inputs.sections {
        let result = ex1::solve(&input.content);
        println!("{}", result);
    }

    Ok(())
}
