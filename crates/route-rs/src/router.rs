use crate::{ Lexer, LexerError };

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Lexer(LexerError),
    InsufficientSegments,
}

impl From<LexerError> for Error {
    fn from(src: LexerError) -> Self {
        Error::Lexer(src)
    }
}

#[derive(Clone, Debug)]
pub struct Router<'a> {
    lexer: Lexer<'a, &'a str>,
}

impl<'a> Router<'a> {
    pub fn new(path: &'a str) -> Self {
        Router {
            lexer: Lexer::new(path),
        }
    }

    pub fn take<const N: usize>(&mut self) -> Result<[&'a str; N], Error> {
        let mut result: [&str; N] = [""; N];
        for i in 0..N {
            let (value, _span) = self.lexer.next().ok_or(Error::InsufficientSegments)??;
            result[i] = value;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn return_error_when_requesting_too_many_segments() {
        let mut router = Router::new("/foo");
        let err = router.take::<2>().unwrap_err();
        assert_eq!(Error::InsufficientSegments, err);
    }

    #[test]
    fn move_to_next_segment_with_each_take() {
        let mut router = Router::new("/foo/bar");
        let first = router.take::<1>().unwrap()[0];
        let second = router.take::<1>().unwrap()[0];
        assert_eq!("foo", first);
        assert_eq!("bar", second);
    }
}
