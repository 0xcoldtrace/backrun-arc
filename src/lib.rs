//! arc_arb — khung A0: load config + JSON-RPC đọc (block/baseFee).
//! Không ký, không sendRaw, không subscribe pending.

pub mod config;
pub mod rpc;

pub use config::Config;
pub use rpc::{RpcClient, RpcError};
