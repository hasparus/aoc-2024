mod ex1;
mod ex2;
mod parse_input;

use aoc_2024_lib::input_reader;

fn main() -> Result<(), std::io::Error> {
    let inputs = input_reader::read_input("inputs.md")?;

    for input in inputs.sections {
        println!("\n{}", input.name);
        println!("{}", ex1::solve(&input.content));
        println!("{}", ex2::solve(&input.content));
    }

    Ok(())
}
