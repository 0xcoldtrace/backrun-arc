//! arc_arb — A1: config + JSON-RPC đọc + Swap log decoder (V2/V3/V4).
//! Không ký, không sendRaw, không subscribe pending.

pub mod config;
pub mod logs;
pub mod pairbook;
pub mod rpc;
pub mod watch;

pub use config::Config;
pub use pairbook::PairBook;
pub use rpc::{RpcClient, RpcError};
