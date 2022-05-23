//! # edgecord
//!
//! `edgecord` is a discord http interaction bot framework for cloudflare workers.
//!
//!
//!
//!

extern crate core;

pub mod application_command;
pub mod builder;
pub mod error;
pub mod handler;
pub mod http;
pub mod model;

#[doc(inline)]
pub use async_trait::async_trait;
#[doc(inline)]
pub use edgecord_macros::*;

pub use error::Error;

pub type InteractionResponse = worker::Result<worker::Response>;
pub type Result<T> = std::result::Result<T, Error>;
