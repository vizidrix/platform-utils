
#[derive(Clone, Debug, PartialEq)]
pub enum InsertError {
    EmptyPath,
    InvalidPath(Option<usize>, String),
    TrailingSlash(usize),
    TrailingWildcardPath,
}

impl From<url::ParseError> for InsertError {
    fn from(src: url::ParseError) -> InsertError {
        InsertError::InvalidPath(None, src.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchError {
    NotFound,
}
