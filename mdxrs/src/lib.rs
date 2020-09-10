use nom::{
    bytes::complete::{self, tag, take_while_m_n},
    character::*,
    combinator::map_res,
    error::{ErrorKind, ParseError},
    sequence::terminated,
    Err::{Error, Failure, Incomplete},
    IResult, *,
};

mod headings;
mod mdx_ast;
mod mdx_error;

use mdx_ast::{mdx_ast, MdxAst};
use mdx_error::MDXError;

#[derive(Debug, PartialEq, Eq)]
pub struct MdxFile<'a> {
    contents: Vec<MdxAst<'a>>,
}

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
    use crate::headings::{ATXHeading, Heading};

    #[test]
    fn test_parse_mdx_file() {
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
    fn test_parse_mdx_file_headings() {
        assert_eq!(
            parse(
                b"# boop
## second
### third
#### fourth
##### fifth
###### sixth"
            ),
            Ok(MdxFile {
                contents: vec![
                    MdxAst::Heading(Heading::ATXHeading(ATXHeading {
                        level: 1,
                        value: b"boop"
                    })),
                    MdxAst::Heading(Heading::ATXHeading(ATXHeading {
                        level: 2,
                        value: b"second"
                    })),
                    MdxAst::Heading(Heading::ATXHeading(ATXHeading {
                        level: 3,
                        value: b"third"
                    })),
                    MdxAst::Heading(Heading::ATXHeading(ATXHeading {
                        level: 4,
                        value: b"fourth"
                    })),
                    MdxAst::Heading(Heading::ATXHeading(ATXHeading {
                        level: 5,
                        value: b"fifth"
                    })),
                    MdxAst::Heading(Heading::ATXHeading(ATXHeading {
                        level: 6,
                        value: b"sixth"
                    }))
                ]
            })
        );
    }
}
