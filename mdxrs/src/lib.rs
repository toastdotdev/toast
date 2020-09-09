use nom::{
    bytes::complete::{self, tag, take_while_m_n},
    character::*,
    combinator::map_res,
    error::{ErrorKind, ParseError},
    sequence::tuple,
    Err::Error,
    IResult, *,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
//     u8::from_str_radix(input, 16)
// }

// fn is_hex_digit(c: char) -> bool {
//     c.is_digit(16)
// }

// fn hex_primary(input: &str) -> IResult<&str, u8> {
//     map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
// }

named!(hashtags, is_a!("#"));
named!(alpha, take_while!(is_alphanumeric));
named!(spaces, take_while!(is_space));

#[derive(Debug, PartialEq)]
pub enum MDXError<I> {
    TooManyHashes,
    Nom(I, ErrorKind),
}

// I want to do this so that ? works but it don't
// impl<I> From<Err<(I, ErrorKind)>> for Err<MDXError<I>> {
//     fn from(e: Err<(I, ErrorKind)>) -> Self {
//         match e {
//             Error((i, e)) => return Error(MDXError::Nom(i, e)),
//         };
//         // MDXError::Nom(i, ek)
//     }
// }

// neither does this
impl<'a> From<(&'a [u8], ErrorKind)> for MDXError<&'a [u8]> {
    fn from((i, ek): (&'a [u8], ErrorKind)) -> Self {
        MDXError::Nom(i, ek)
    }
}

impl<I> ParseError<I> for MDXError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        MDXError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

fn hash(i: &[u8]) -> IResult<&[u8], &[u8], MDXError<&[u8]>> {
    complete::is_a("#")(i)
}

fn atx_heading(input: &[u8]) -> IResult<&[u8], ATXHeading, MDXError<&[u8]>> {
    // let a = (input);
    let (input, hashes) = hashtags(input).map_err(Err::convert)?;
    if hashes.len() > 6 {
        return Err(Error(MDXError::TooManyHashes));
    }
    let (input, _) = spaces(input).map_err(Err::convert)?;
    let (input, val) = nom::bytes::complete::take_while(is_alphanumeric)(input)?;
    Ok((
        input,
        ATXHeading {
            level: hashes.len(),
            value: val,
        },
    ))
}

#[derive(Debug, PartialEq, Eq)]
struct ATXHeading<'a> {
    level: usize,
    value: &'a [u8],
}
struct SEHeading {}
enum Heading<'a> {
    ATXHeading(&'a ATXHeading<'a>),
    SEHeading(SEHeading),
}

#[test]
fn parse_mdx() {
    assert_eq!(
        atx_heading(b"# boop\n"),
        Ok((
            "\n".as_bytes(),
            ATXHeading {
                level: 1,
                value: b"boop"
            }
        ))
    );
}
