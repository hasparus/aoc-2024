use aoc_2024_lib::input_reader::read_input;
use parse_input::parse_input;

mod ex_1;
mod parse_input;
mod point2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = read_input("./inputs.md")?;

    println!("\nDay 15");
    for input in input_file.sections {
        let ex1_result = ex_1::solve(&parse_input(&input.content)?);
        println!("ex1 {}:\t{}", input.name, ex1_result);
    }

    Ok(())
}
