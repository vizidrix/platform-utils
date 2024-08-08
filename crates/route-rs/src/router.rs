use crate::{ Lexer, LexerError };

#[derive(Clone, Debug, PartialEq)]
pub enum RouterError {
    Lexer(LexerError),
    InsufficientSegments,
}

impl From<LexerError> for RouterError {
    fn from(src: LexerError) -> Self {
        RouterError::Lexer(src)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Router<'a> {
    lexer: Lexer<'a, &'a str>,
}

impl<'a> Router<'a> {
    pub fn new(path: &'a str) -> Self {
        Router {
            lexer: Lexer::new(path),
        }
    }

    pub fn peek<const N: usize>(&mut self) -> [Option<&'a str>; N] {
        let mut result: [Option<&'a str>; N] = [None; N];
        let mut lexer = None;
        for i in 0..N {
            let mylexer = lexer.unwrap_or(self.lexer);
            let (new_lexer, peek) = mylexer.peek();
            lexer = Some(new_lexer);
            if let Some(Ok((value, _distance, _span))) = peek {
                result[i] = Some(value);
            }
        }
        result
    }

    pub fn consume<const N: usize>(&mut self) -> [Option<&'a str>; N] {
        let mut result: [Option<&'a str>; N] = [None; N];
        for i in 0..N {
            if let Some(Ok((value, _span))) = self.lexer.next() {
                result[i] = Some(value);
            }
        }
        result
    }

    pub fn try_consume<const N: usize>(&mut self) -> Result<[&'a str; N], RouterError> {
        let mut result: [&str; N] = [""; N];
        for i in 0..N {
            let (value, _span) = self.lexer.next().ok_or(RouterError::InsufficientSegments)??;
            result[i] = value;
        }
        Ok(result)
    }
}

impl<'a> Iterator for Router<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(Ok((item, _span))) => Some(item),
            _ => None,
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn convert_lexer_error_to_router_error() {
        let lexer_err = LexerError::InvalidPath(Some(3), "foo".to_owned());
        let inner_err = lexer_err.to_owned();
        let router_err: RouterError = lexer_err.into();
        assert_eq!(RouterError::Lexer(inner_err), router_err);
    }

    #[test]
    fn fill_empty_segment_and_none_for_take_from_root_path() {
        let mut router = Router::new("/");
        let segments = router.consume::<2>();
        assert_eq!([Some(""), None], segments);
    }

    #[test]
    fn not_consume_segments_on_peek() {
        let mut router = Router::new("/foo/bar");
        let peek = router.peek::<2>();
        assert_eq!([Some("foo"), Some("bar")], peek);
        let peek = router.peek::<2>();
        assert_eq!([Some("foo"), Some("bar")], peek);
        let segments = router.consume::<2>();
        assert_eq!([Some("foo"), Some("bar")], segments);
    }

    #[test]
    fn not_return_consumed_segments_on_peek() {
        let mut router = Router::new("/foo/bar");
        let segments = router.consume::<1>();
        assert_eq!([Some("foo")], segments);
        let peek = router.peek::<2>();
        assert_eq!([Some("bar"), None], peek);
    }

    #[test]
    fn fill_all_none_for_take_past_path_end() {
        let mut router = Router::new("/foo/bar");
        router.consume::<2>();
        let segments = router.consume::<2>();
        assert_eq!([None, None], segments);
    }

    #[test]
    fn fill_all_segments_for_sufficient_path() {
        let mut router = Router::new("/foo/bar");
        let segments = router.consume::<2>();
        assert_eq!([Some("foo"), Some("bar")], segments);
    }

    #[test]
    fn fill_partial_segments_for_missing_path() {
        let mut router = Router::new("/foo");
        let segments = router.consume::<2>();
        assert_eq!([Some("foo"), None], segments);
    }

    #[test]
    fn match_valid_segment_count_with_all_some() {
        let mut router = Router::new("/foo/bar");
        let segments = router.consume::<2>();
        assert_eq!(segments[0], Some("foo"));
        assert_eq!(segments[1], Some("bar"));
    }

    #[test]
    fn match_short_segment_count_with_padded_none() {
        let mut router = Router::new("/foo/bar");
        let segments = router.consume::<3>();
        assert_eq!(segments[0], Some("foo"));
        assert_eq!(segments[1], Some("bar"));
        assert_eq!(segments[2], None);
    }

    #[test]
    fn return_error_when_requesting_too_many_segments_with_take_or() {
        let mut router = Router::new("/foo");
        let err = router.try_consume::<2>().unwrap_err();
        assert_eq!(RouterError::InsufficientSegments, err);
    }

    #[test]
    fn move_to_next_segment_with_each_take_or() {
        let mut router = Router::new("/foo/bar");
        let first = router.try_consume::<1>().unwrap()[0];
        let second = router.try_consume::<1>().unwrap()[0];
        assert_eq!("foo", first);
        assert_eq!("bar", second);
    }

    #[test]
    fn walk_segments_from_router_as_iterator() {
        let router = Router::new("/foo/bar");
        let segments = router.into_iter().collect::<Vec<_>>();
        assert_eq!(2, segments.len());
        assert_eq!(segments[0], "foo");
        assert_eq!(segments[1], "bar");
    }

    #[test]
    fn return_error_from_router_as_iterator() {
        let router = Router::new("");
        let segments = router.into_iter().collect::<Vec<_>>();
        assert_eq!(0, segments.len());
    }
}
