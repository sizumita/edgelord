//! # edgelord_discord
//!
//! `edgelord_discord` is a discord http interaction bot framework for cloudflare workers.
//!
//!
//!
//!

extern crate core;

mod builder;
mod command;
mod error;
mod handler;

pub use async_trait::async_trait;
pub use edgelord_discord_macros::*;
pub use twilight_model as model;

pub use builder::*;
pub use command::*;
pub use error::*;
pub use handler::*;

pub type InteractionResponse = worker::Result<worker::Response>;
