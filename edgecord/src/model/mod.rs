pub mod rest;

use std::fmt::{Display, Formatter};
pub use twilight_model::*;

/// Emoji for encoding to URL Encoded.
pub struct UrlEncodedEmoji(guild::Emoji);

impl Display for UrlEncodedEmoji {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0.name, self.0.id)?;
        todo!("add percent encoding")
    }
}
