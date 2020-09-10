use nom::{
    bytes::complete::{self, tag, take_while_m_n},
    character::*,
    combinator::map_res,
    error::{ErrorKind, ParseError},
    sequence::terminated,
    Err::{Error, Failure, Incomplete},
    IResult, *,
};

use crate::headings::{atx_heading, Heading};
use crate::mdx_error::MDXError;

#[derive(Debug, PartialEq, Eq)]
pub enum MdxAst<'a> {
    Heading(Heading<'a>),
}

fn mdx_atx_heading(input: &[u8]) -> IResult<&[u8], MdxAst, MDXError<&[u8]>> {
    let (input, heading) = atx_heading(input).map_err(Err::convert)?;
    Ok((input, MdxAst::Heading(Heading::ATXHeading(heading))))
}

named!(pub mdx_ast<&[u8], (Vec<MdxAst>, &[u8]), MDXError<&[u8]>>, many_till!(mdx_atx_heading, eof!()));
