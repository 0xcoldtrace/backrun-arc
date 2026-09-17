//! arc_arb — A3: paper quote 2 chân aero_cl vs uni_v3.
//! Không ký, không sendRaw, không subscribe pending, không Morpho.flashLoan.

pub mod config;
pub mod discover;
pub mod logs;
pub mod pairbook;
pub mod paper;
pub mod quote;
pub mod rpc;
pub mod vet;
pub mod watch;

pub use config::Config;
pub use pairbook::PairBook;
pub use rpc::{RpcClient, RpcError};
