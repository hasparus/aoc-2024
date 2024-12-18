use aoc_2024_lib::input_reader::read_input;
use parse_input::parse_input;

mod ex1;
mod ex2;
mod parse_input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = read_input("./inputs.md")?;

    println!("\nDay 15");

    for input in input_file.sections {
        let ex1_result = ex1::solve(&parse_input(&input.content)?);
        let ex2_result = ex2::solve(&parse_input(&input.content)?);
        println!("ex1 {}:\t{}", input.name, ex1_result);
        println!("ex2 {}:\t{}", input.name, ex2_result);
    }

    Ok(())
}
