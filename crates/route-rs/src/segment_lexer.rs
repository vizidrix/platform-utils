///
/// SegmentLexer extracts string sections between forward slashes and
/// converts them into segment definitions per the rules
/// 
/// [Lexer Example](https://users.rust-lang.org/t/how-to-write-a-fast-parser-in-idiomatic-rust/49927/2)
/// [Token Scanning Examples](https://petermalmgren.com/token-scanning-with-rust/)
/// 

use crate::InsertError;
use crate::SegmentType;

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    start: usize,
    end: usize,
}

pub struct SegmentLexer<'a> {
    src: &'a str,
    cursor: usize,
}

impl<'a> SegmentLexer<'a> {
    pub fn new(src: &'a str) -> Self {
        SegmentLexer {
            src,
            cursor: 0,
        }
    }

    pub fn rest(&self) -> &'a str {
        &self.src[self.cursor..]
    }
}

// ""
// "/"
// "/foo"
// "/foo/bar"

impl<'a> Iterator for SegmentLexer<'a> {
    type Item = Result<(SegmentType<'a>, Span), InsertError>;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Skip whitespace?
        // if self.src.len() == 0 {
        //     return Some(Err(InsertError::EmptyPath));
        // }
        let rest = self.rest();
        let len = rest.len();
        if len == 0 {
            // return Some(Err(InsertError::TrailingSlash(self.cursor)));
            // return Some(Err(InsertError::EmptyPath));
            return None;
        }
        // Each segment has to start with a leading slash
        if !rest.starts_with('/') {
            return Some(Err(InsertError::InvalidPath(Some(self.cursor), rest.to_string())));
        }
        // Either root slash or a trailing empty slash
        if len == 1 {
            // let span = Span { start: self.cursor, end: self.cursor + 1 };
            // self.cursor += 1;
            // let segment = "".into();
            // // return Some(Ok(SegmentType::Static { path: "" }));
            // return Some(Ok((segment, span)));
            return None;
        }
        // Scan to next slash
        let mut char_indices = rest.char_indices();
        let mut dist = len;
        // panic!("{:?} {:?}", rest, len);
        while let Some((pos, ch)) = char_indices.next() {
            // panic!("{:?} {:?}", pos, ch);
            // Found beginning of next segment
            if pos > 0 && ch == '/' {
                dist = pos;
                break;
                // let span = Span { start: self.cursor, end: self.cursor + pos };
                // self.cursor += pos;
                // let segment = rest[0..pos].into();
                // Some(Ok((segment, span)))
            }
        }
        // let span = Span { start: self.cursor, end: self.cursor + len };
        // self.cursor += len;
        // let segment = 
        let span = Span { start: self.cursor + 1, end: self.cursor + dist };
        self.cursor += dist;
        let segment = rest[1..dist].into();
        Some(Ok((segment, span)))
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn return_single_none_for_empty_string() {
        let mut lexer = SegmentLexer::new("");
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn return_invalid_path_error_for_missing_leading_slash() {
        let expected = Err(InsertError::InvalidPath(Some(0), "foo".to_owned()));
        let mut lexer = SegmentLexer::new("foo");
        assert_eq!(Some(expected), lexer.next());
    }

    #[test]
    fn parse_bare_root_as_none() {
        // let expected = (SegmentType::Static { path: "" }, Span { start: 0, end: 1 } );
        let mut lexer = SegmentLexer::new("/");
        // assert_eq!(Some(Ok(expected)), lexer.next());
        assert_eq!(None, lexer.next());
    }

    #[test]
    fn parse_static_segment() {
        let expected = vec![
            (SegmentType::Static { path: "foo" }, Span { start: 1, end: 4 } ),
        ];
        let lexer = SegmentLexer::new("/foo");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_param_segment() {
        let expected = vec![
            (SegmentType::Param { key: "foo" }, Span { start: 1, end: 5 } ),
        ];
        let lexer = SegmentLexer::new("/:foo");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_consume_segment() {
        let expected = vec![
            (SegmentType::Consume { key: "foo" }, Span { start: 1, end: 5 } ),
        ];
        let lexer = SegmentLexer::new("/*foo");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_wildcard_segment() {
        let expected = vec![
            (SegmentType::Wildcard, Span { start: 1, end: 2 } ),
        ];
        let lexer = SegmentLexer::new("/*");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }

    #[test]
    fn parse_two_static_segments() {
        let expected = vec![
            (SegmentType::Static { path: "foo" }, Span { start: 1, end: 4 } ),
            (SegmentType::Static { path: "bar" }, Span { start: 5, end: 8 } ),
        ];
        let lexer = SegmentLexer::new("/foo/bar");
        let values = lexer.collect::<Result<Vec<_>, _>>().unwrap();
        assert_eq!(expected, values);
    }
}