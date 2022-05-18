//! # edgelord_discord
//!
//! `edgelord_discord` is a discord http interaction bot framework for cloudflare workers.
//!
//!
//!
//!

extern crate core;

mod command;
pub mod http;
mod handler;
mod builder;
mod error;

use std::collections::HashMap;

pub use edgelord_discord_macros::*;
pub use twilight_model as model;
pub use async_trait::async_trait;

pub use command::*;
pub use handler::*;
pub use builder::*;
pub use error::*;

pub type InteractionResponse = worker::Result<worker::Response>;
