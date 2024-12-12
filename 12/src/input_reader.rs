use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, multispace0, not_line_ending},
    multi::many0,
    sequence::{delimited, preceded, terminated},
    IResult,
};
use std::fs;

#[derive(Debug)]
pub struct Input {
    pub name: String,
    pub content: String,
}

#[derive(Debug)]
pub struct InputsFile {
    pub sections: Vec<Input>,
}

fn parse_title(input: &str) -> IResult<&str, String> {
    let (input, title) = preceded(tag("# "), not_line_ending)(input)?;
    Ok((input, title.trim().to_string()))
}

fn parse_code_block(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, content) = delimited(tag("```"), take_until("```"), tag("```"))(input)?;
    Ok((input, content.trim().to_string()))
}

fn parse_section(input: &str) -> IResult<&str, Input> {
    let (input, _) = multispace0(input)?;
    let (input, title) = terminated(parse_title, line_ending)(input)?;
    let (input, content) = parse_code_block(input)?;

    Ok((
        input,
        Input {
            name: title,
            content,
        },
    ))
}

fn parse_markdown_subset(input: &str) -> IResult<&str, InputsFile> {
    let (input, _) = multispace0(input)?;
    let (input, sections) = many0(parse_section)(input)?;
    Ok((input, InputsFile { sections }))
}

pub fn read_input(file_path: &str) -> Result<InputsFile, std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    match parse_markdown_subset(&contents) {
        Ok((_, input_file)) => Ok(input_file),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse markdown subset: {}", e),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = read_input("inputs.md").expect("Should parse input file");
        assert_eq!(input.sections.len(), 3);
        assert_eq!(
            input
                .sections
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<_>>(),
            ["Trivial", "Simple", "Input"]
        );
    }
}
