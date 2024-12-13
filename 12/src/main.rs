mod ex1;
mod ex2;
mod grid;
mod input_reader;

fn main() -> Result<(), std::io::Error> {
    let inputs = input_reader::read_input("inputs.md")?;

    println!("\n{:<10}\tPrice   \tDiscount", "Input");
    println!("{:<10}\t--------\t--------", "--------");
    for input in inputs.sections {
        let price = ex1::solve(&input.content);
        let price_with_bulk_discount = ex2::solve(&input.content);
        println!(
            "{:<10}\t{:<10}\t{}",
            input.name, price, price_with_bulk_discount
        );
    }

    Ok(())
}
