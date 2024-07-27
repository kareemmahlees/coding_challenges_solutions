//! # Roar
//! A Redis server clone with full support for the RESP protocol.
//! Supports the following commands:
//!
//! - PING
//! - ECHO
//! - SET
//! - GET
//! - INCR
//! - DECR
//! - EXISTS
//! - DEL
//! - LPUSH
//! - RPUSH

mod command;
mod data_type;
mod deserializer;
mod server;

pub use command::Command;
pub use data_type::DataType;
pub(crate) use deserializer::DeSerializer;
pub use server::Server;
