#[derive(Clone, Debug, PartialEq)]
pub enum SegmentType {
    /// Exactly matched
    Static(String),
    /// Convert single segment value into a param with provided key
    Param(String),
    /// Convert remaining path value into a param with provided key
    Consume(String),
    /// Match but discard remaining segments beyond this one
    Wildcard,
}

impl<'a> From<&'a str> for SegmentType {
    fn from(src: &'a str) -> Self {
        match src {
            "" => SegmentType::Static("".to_owned()),
            "*" => SegmentType::Wildcard,
            _ => match &src[..1] {
                ":" => SegmentType::Param(src[1..].to_owned()),
                "*" => SegmentType::Consume(src[1..].to_owned()),
                _ => SegmentType::Static(src.to_owned()),
            },
        }
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    // Clears codecov misses for derived macros
    fn clone_segment_type() {
        assert_eq!("Wildcard", format!("{:?}", SegmentType::Wildcard));
        assert_eq!(SegmentType::Wildcard, SegmentType::Wildcard.clone());
    }

    #[test]
    fn accept_empty_string_as_static() {
        assert_eq!(SegmentType::Static("".to_owned()), "".into());
    }

    #[test]
    // Expect the caller to provide char validation for thier domain
    fn accept_invalid_path_chars() {
        assert_eq!(SegmentType::Static("$%^&#@$".to_owned()), "$%^&#@$".into());
    }

    #[test]
    fn parse_static_segment_type() {
        assert_eq!(SegmentType::Static("foo".to_owned()), "foo".into());
    }

    #[test]
    fn parse_param_segment_type() {
        assert_eq!(SegmentType::Param("foo".to_owned()), ":foo".into());
    }

    #[test]
    // Supports empty string param key
    fn parse_unnamed_param_segment_type() {
        assert_eq!(SegmentType::Param("".to_owned()), ":".into());
    }

    #[test]
    fn parse_consume_segment_type() {
        assert_eq!(SegmentType::Consume("foo".to_owned()), "*foo".into());
    }

    #[test]
    fn parse_wildcard_segment_type() {
        assert_eq!(SegmentType::Wildcard, "*".into());
    }
}
