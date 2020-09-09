use nom::{
    bytes::complete::{self, tag, take_while_m_n},
    character::*,
    combinator::map_res,
    error::{ErrorKind, ParseError},
    sequence::terminated,
    Err::{Error, Failure, Incomplete},
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
// pub fn is_not_newlines(chr: u8) -> bool {
//     !(chr == b'\r' || chr == b'\n')
// }
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
// named!(line_ending, alt!(char!('\n') | char!('\r') | eof!()));
// named!(alpha, take_until!(line_ending));
named!(spaces, take_while!(is_space));

#[derive(Debug, PartialEq)]
pub enum MDXError<I> {
    TooManyHashes,
    Nom(I, ErrorKind),
    Incomplete(I),
    Failure(I),
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
impl<'a> From<(&'a str, ErrorKind)> for MDXError<&'a [u8]> {
    fn from((i, ek): (&'a str, ErrorKind)) -> Self {
        MDXError::Nom(i.as_bytes(), ek)
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
    // TODO: empty headings are a thing, so any parsing below this is optional
    let (input, _) = spaces(input).map_err(Err::convert)?;
    let (input, val) = rest_of_line(std::str::from_utf8(input).unwrap()).map_err(Err::convert)?;
    Ok((
        input.as_bytes(),
        ATXHeading {
            level: hashes.len(),
            value: val.as_bytes(),
        },
    ))
}
fn mdx_atx_heading(input: &[u8]) -> IResult<&[u8], MdxAst, MDXError<&[u8]>> {
    let (input, heading) = atx_heading(input).map_err(Err::convert)?;
    Ok((input, MdxAst::Heading(Heading::ATXHeading(heading))))
}

#[derive(Debug, PartialEq, Eq)]
pub struct ATXHeading<'a> {
    level: usize,
    value: &'a [u8],
}
#[derive(Debug, PartialEq, Eq)]
pub struct SEHeading {}
#[derive(Debug, PartialEq, Eq)]
pub enum Heading<'a> {
    ATXHeading(ATXHeading<'a>),
    SEHeading(SEHeading),
}
#[derive(Debug, PartialEq, Eq)]
pub enum MdxAst<'a> {
    Heading(Heading<'a>),
}
#[derive(Debug, PartialEq, Eq)]
pub struct MdxFile<'a> {
    contents: Vec<MdxAst<'a>>,
}
named!(eof, eof!());
named!(mdx_ast<&[u8], (Vec<MdxAst>, &[u8]), MDXError<&[u8]>>, many_till!(mdx_atx_heading, eof!()));
// fn mdx_heading(s: &[u8]) -> IResult<&[u8], (Vec<MdxAst>, MDXError)> {
//     nom::multi::many_till(mdx_atx_heading, tag("end"))(s)
// }
// named!(mdx, );
fn parse_mdx(input: &[u8]) -> IResult<&[u8], MdxFile, MDXError<&[u8]>> {
    let (input, (contents, _end)) = mdx_ast(input).map_err(Err::convert)?;
    println!("aaa {:?} {:?} {:?}", input, contents, _end);
    Ok((input, MdxFile { contents }))
}
pub fn parse<'a>(input: &'a [u8]) -> Result<MdxFile<'a>, MDXError<&[u8]>> {
    let result = parse_mdx(input);
    println!("result {:?}", result);
    match result {
        Ok((input, file)) => Ok(file),
        Err(Error(e)) => {
            match e {
                MDXError::Nom(i, e) => {
                    println!("errprint: {:?} {:?}", std::str::from_utf8(i), e);
                }
                _ => {}
            };
            Err(e)
        }
        Err(Incomplete(e)) => Err(MDXError::Incomplete(b"needs more")),
        Err(Failure(e)) => Err(e),
    }
    // Ok(MdxFile { contents: vec![] })
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mdx() {
        assert_eq!(
            parse(b"# boop"),
            Ok(MdxFile {
                contents: vec![MdxAst::Heading(Heading::ATXHeading(ATXHeading {
                    level: 1,
                    value: b"boop"
                }))]
            })
        );
    }
    #[test]
    fn parse_atx_heading() {
        assert_eq!(
            atx_heading(b"# boop\n"),
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
