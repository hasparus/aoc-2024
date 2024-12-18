mod ex1;
mod ex2;

use std::error::Error;

use aoc_2024_lib::input_reader::read_input;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = read_input("./inputs.md")?;

    for input in inputs.sections {
        let result = ex1::solve(&input.content);
        let result2 = ex2::sum_all_points_on_shortest_paths(&input.content);
        println!("ex1 {}: {}", input.name, result);
        println!("ex2 {}: {}", input.name, result2);
    }
    Ok(())
}
