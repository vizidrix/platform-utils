mod lexer;
mod router;

pub use lexer::{Lexer, LexerError};
pub use router::{Router, RouterError};
pub use std::future::Future;

#[cfg(feature="worker")]
use worker::{ Env, Request, Result, Response };

#[cfg(feature="worker")]
pub trait Service {
    fn handler(self, router: Router, req: Request, env: Env, ctx: worker::Context) -> impl Future<Output = Result<Response>> + Send;
}