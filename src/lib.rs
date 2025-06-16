//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/serde-ini/")]

//! Windows INI format serialization for serde

pub mod de;
pub mod error;
pub mod parse;
pub mod ser;
pub mod write;

pub use de::{Deserializer, from_bufread, from_read, from_str};
pub use parse::{Item, Parser};
pub use ser::{Serializer, to_string, to_vec, to_writer};
pub use write::{LineEnding, Writer};
