//! arc_arb — A2: pairbook vet tax + aero verify + watch candidates.
//! Không ký, không sendRaw, không subscribe pending, không Morpho.flashLoan.

pub mod config;
pub mod discover;
pub mod logs;
pub mod pairbook;
pub mod quote;
pub mod rpc;
pub mod vet;
pub mod watch;

pub use config::Config;
pub use pairbook::PairBook;
pub use rpc::{RpcClient, RpcError};
