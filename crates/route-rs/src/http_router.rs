#[derive(Clone, Debug)]
pub enum MatchError {
    NotFound,
}

#[derive(Clone, Debug)]
pub struct HttpRouter {}

impl HttpRouter {
    pub fn new() -> Self {
        HttpRouter {}
    }

    pub fn eval(&self, _value: impl Into<String>) -> Result<(), MatchError> {
        Err(MatchError::NotFound)
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn return_error_for_eval_base_path_on_empty_router() {
        let router = HttpRouter::new();
        match router.eval("/").unwrap_err() {
            MatchError::NotFound => {}
        }
    }

    #[test]
    fn f() {
        let _router = HttpRouter::new();
        // router.insert("get", "/", )
    }
}
