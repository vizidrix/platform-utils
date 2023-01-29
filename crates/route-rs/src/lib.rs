mod lexer;
mod router;

pub use lexer::{Lexer, LexerError};
pub use router::{Router, RouterError};

#[cfg(feature="worker")]
use worker::{ Env, Request, Result, Response };

#[cfg(feature="worker")]
pub trait Service {
    fn handler(&mut self, router: &mut Router, req: Request, env: Env, ctx: worker::Context) -> Result<Response>;
}