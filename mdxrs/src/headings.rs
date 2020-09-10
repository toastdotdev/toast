use nom::{character::*, sequence::terminated, Err::Error, IResult, *};

use crate::mdx_error::MDXError;

#[derive(Debug, PartialEq, Eq)]
pub struct ATXHeading<'a> {
    pub level: usize,
    pub value: &'a [u8],
}
#[derive(Debug, PartialEq, Eq)]
pub struct SEHeading {}

#[derive(Debug, PartialEq, Eq)]
pub enum Heading<'a> {
    ATXHeading(ATXHeading<'a>),
    SEHeading(SEHeading),
}

// Parsers
pub fn end_of_line(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        nom::character::complete::line_ending(input)
    }
}
pub fn rest_of_line(input: &str) -> IResult<&str, &str> {
    terminated(nom::character::complete::alphanumeric0, end_of_line)(input)
}
named!(hashtags, is_a!("#"));
named!(spaces, take_while!(is_space));
named!(eof, eof!());

pub fn atx_heading(input: &[u8]) -> IResult<&[u8], ATXHeading, MDXError<&[u8]>> {
    // TODO: up to 3 spaces can occur here
    let (input, hashes) = hashtags(input).map_err(Err::convert)?;
    if hashes.len() > 6 {
        return Err(Error(MDXError::TooManyHashes));
    }
    // TODO: empty headings are a thing, so any parsing below this is optional
    let (input, _) = spaces(input).map_err(Err::convert)?;
    // TODO: any whitespace on the end would get trimmed out
    let (input, val) = rest_of_line(std::str::from_utf8(input).unwrap()).map_err(Err::convert)?;
    Ok((
        input.as_bytes(),
        ATXHeading {
            level: hashes.len(),
            value: val.as_bytes(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_atx_heading() {
        assert_eq!(
            atx_heading(b"# boop"),
            Ok((
                "".as_bytes(),
                ATXHeading {
                    level: 1,
                    value: b"boop"
                }
            ))
        );
    }
}
