use nom::error::{ErrorKind, ParseError};

#[derive(Debug, PartialEq)]
pub enum MDXError<I> {
    TooManyHashes,
    Nom(I, ErrorKind),
    Incomplete(I),
    Failure(I),
}

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
