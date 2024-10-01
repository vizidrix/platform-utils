mod lexer;
mod router;

pub use lexer::{Lexer, LexerError};
pub use router::{Router, RouterError};
pub use std::future::Future;

// #[cfg(feature="worker")]
// use worker::{ Env, Request, Result, Response };

// #[cfg(feature="worker")]
// pub trait Service {
//     fn handler(self, router: Router, req: Request, env: Env, ctx: worker::Context) -> impl Future<Output = Result<Response>>;
// }

// #[cfg(feature="worker")]
// pub trait WorkerService {
//     type Action;

//     fn from_request(self, router: Router, req: Request, env: Env, ctx: worker::Context) -> impl Future<Output = Result<impl Into<Self::Action>>>;
// }

// pub trait ActionHandler {
//     type Action;
//     type ActionResult;

// }

// #[cfg(feature="worker")]
// pub trait WorkerResponder {
//     type Response = impl Into<Response>;

//     fn handle(self, action: TAction) -> impl Future<Output = Result<Self::Response>>;
// }