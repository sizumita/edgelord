mod hook;
pub mod http;

#[cfg(feature = "discord")]
pub use edgelord_discord as discord;

pub use serde_json as json;

pub use hook::{set_panic_hook};
