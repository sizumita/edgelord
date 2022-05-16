//! # edgelord_discord
//!
//! `edgelord_discord` is a discord http interaction bot framework for cloudflare workers.
//!
//!
//!
//!

mod command;
pub mod http;
mod handler;
mod builder;

use std::collections::HashMap;

pub use edgelord_discord_macros::*;
pub use twilight_model as model;
pub use async_trait::async_trait;

pub use command::*;
pub use handler::*;
pub use builder::*;

pub type InteractionResponse = worker::Result<worker::Response>;
