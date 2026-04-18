//! Event ingestion pipeline — RPC polling, XDR decoding, data transformation.

pub mod listener;
pub mod decoder;

pub use listener::EventListener;
pub use decoder::XdrDecoder;
