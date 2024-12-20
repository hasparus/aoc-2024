use anyhow::{anyhow, Result};

pub struct Towel<'a>(pub &'a str);
pub struct Design<'a>(pub &'a str);

pub fn parse_input(input: &str) -> Result<(Vec<Towel>, Vec<Design>)> {
    let (towels, designs) = input.split_once("\n\n").ok_or(anyhow!(
        "Failed to parse input. Expected two newlines. {}",
        input
    ))?;

    let towels = towels.split(',').map(|t| Towel(t.trim())).collect();
    let designs = designs.lines().map(|d| Design(d.trim())).collect();

    Ok((towels, designs))
}
