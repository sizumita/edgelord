//! # edgecord
//!
//! `edgecord` is a discord http interaction bot framework for cloudflare workers.
//!
//!
//!
//!

extern crate core;

mod builder;
mod command;
mod error;
mod handler;
pub mod http;
pub mod model;

pub use async_trait::async_trait;
pub use edgecord_macros::*;

pub use builder::*;
pub use command::*;
pub use error::*;
pub use handler::*;

pub type InteractionResponse = worker::Result<worker::Response>;
pub type Result<T> = std::result::Result<T, Error>;
