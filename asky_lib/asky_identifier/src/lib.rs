// A tiny library that offers a validated `Identifier` type (`namespace:thing`)

#[cfg(feature = "serde")]
mod serde_impl;

mod error;
mod identifier;
mod identifier_fmt;
mod validation;

pub use error::IdentifierParseError;
pub use identifier::Identifier;

pub mod prelude {
    pub use super::Identifier;
    pub use super::IdentifierParseError;
}
