mod hook;
pub mod http;

#[cfg(feature = "discord")]
pub use edgelord_discord as discord;

pub use hook::set_panic_hook;
