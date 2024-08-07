///
/// Lexer extracts string sections between forward slashes and
/// converts them into segment definitions per the rules
///
/// [Lexer Example](https://users.rust-lang.org/t/how-to-write-a-fast-parser-in-idiomatic-rust/49927/2)
/// [Token Scanning Examples](https://petermalmgren.com/token-scanning-with-rust/)
///
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub enum LexerError {
    InvalidPath(Option<usize>, String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn distance(&self) -> usize {
        if self.start == self.end {
            1
        } else {
            self.end - self.start
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lexer<'a, T = String>
where
    T: From<&'a str>,
{
    src: &'a str,
    cursor: usize,
    t: PhantomData<T>,
}

impl<'a, T> Lexer<'a, T>
where
    T: From<&'a str>,
{
    pub fn new(src: &'a str) -> Self {
        Lexer { src, cursor: 0, t: PhantomData }
    }

    fn mv(&self, distance: usize) -> Self {
        let src = self.src;
        Lexer { src, cursor: self.cursor + distance, t: PhantomData  }
    }

    pub fn rest(&self) -> &'a str {
        &self.src[self.cursor..]
    }

    pub fn remaining(&self) -> usize {
        self.src.len() - self.cursor
    }

    pub fn peek(&self) -> (Self, Option<Result<(T, usize, Span), LexerError>>) {
        let rest = self.rest();
        let len = rest.len();
        if len == 0 {
            return (Lexer::mv(self, 0), None);
        }
        // Each segment has to start with a leading slash
        if !rest.starts_with('/') {
            return (Lexer::mv(self, 0), Some(Err(LexerError::InvalidPath(
                Some(self.cursor),
                rest.to_string(),
            ))));
        }
        // Either root slash or a trailing empty slash
        if len == 1 {
            let span = Span {
                start: self.cursor,
                end: self.cursor,
            };
            return (Lexer::mv(self, 1), Some(Ok(("".into(), 1, span))))
        }
        // Scan to next slash
        let mut char_indices = rest.char_indices();
        let mut distance = len;
        while let Some(char_index) = char_indices.next() {
            let (position, char) = char_index;
            // Found beginning of next segment
            if position > 0 && char == '/' {
                distance = position;
                break;
            }
        }
        let span = Span {
            start: self.cursor + 1,
            end: self.cursor + distance,
        };
        let segment = rest[1..distance].into();
        (Lexer::mv(self, distance), Some(Ok((segment, distance, span))))
    }

    pub fn pop(&mut self) -> Option<Result<(T, usize, Span), LexerError>> {
        // Look at the next token
        let (lexer, peek) = self.peek();
        // Move the lexer reference to the the next position
        *self = lexer;
        // Return the peek result
        peek
    }
}

impl<'a, T> Iterator for Lexer<'a, T>
where
    T: From<&'a str>,
{
    type Item = Result<(T, Span), LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.pop() {
            Some(Ok((item, _distance, span))) => {
                Some(Ok((item, span)))
            },
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn accurately_clone_spans() {
        let expected = Span { start: 10, end: 20 };
        let value = expected.clone();
        assert_eq!(expected, value);
    }

    #[test]
    fn return_single_none_for_empty_string() {
        let mut lexer = Lexer::<'_, String>::new("");
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn return_invalid_path_error_for_missing_leading_slash() {
        let expected = Err(LexerError::InvalidPath(Some(0), "foo".to_owned()));
        let mut lexer = Lexer::<'_, String>::new("foo");
        assert_eq!(Some(expected), lexer.next());
    }

    #[test]
    fn parse_bare_root_as_empty_str() {
        let expected = vec![(
            "".to_owned(),
            Span { start: 0, end: 0 },
        )];
        let lexer = Lexer::new("/");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_static_segment() {
        let expected = vec![(
            "foo".to_owned(),
            Span { start: 1, end: 4 },
        )];
        let lexer = Lexer::new("/foo");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_param_segment() {
        let expected = vec![(
            ":foo".to_owned(),
            Span { start: 1, end: 5 }
        )];
        let lexer = Lexer::new("/:foo");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_consume_segment() {
        let expected = vec![(
            "*foo".to_owned(),
            Span { start: 1, end: 5 },
        )];
        let lexer = Lexer::new("/*foo");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_wildcard_segment() {
        let expected = vec![(
            "*",
            Span { start: 1, end: 2 }
        )];
        let lexer = Lexer::new("/*");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_two_static_segments() {
        let expected = vec![
            (
                "foo".to_owned(),
                Span { start: 1, end: 4 },
            ),
            (
                ":bar".to_owned(),
                Span { start: 5, end: 9 },
            ),
        ];
        let lexer = Lexer::new("/foo/:bar");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }
}
