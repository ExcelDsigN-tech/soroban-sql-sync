//! Event ingestion pipeline — RPC polling, XDR decoding, data transformation.

pub mod decoder;
pub mod listener;

#[allow(unused_imports)]
pub use decoder::XdrDecoder;
#[allow(unused_imports)]
pub use listener::EventListener;
