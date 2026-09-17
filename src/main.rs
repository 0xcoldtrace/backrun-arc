//! A0 boot: load config + eth_blockNumber + baseFee.
//! pending_enabled=false (in ra, không subscribe pending).
//! Không PRIVATE_KEY, không sendRaw.

use std::path::Path;
use std::process::ExitCode;

use arc_arb::{Config, RpcClient};

fn main() -> ExitCode {
    let cfg = match Config::from_path(Path::new("config.toml")) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FAIL {e}");
            return ExitCode::from(1);
        }
    };

    println!(
        "config.ok strategy={} chain_id={} dry_run={} allow_live={} bot_armed={} live_mode={} pending_enabled={} flash_provider={} min_profit_usdc={} arb_max_borrow_usdc={} min_base_fee_gwei={}",
        cfg.strategy,
        cfg.chain_id,
        cfg.dry_run,
        cfg.allow_live,
        cfg.bot_armed,
        cfg.live_mode,
        cfg.pending_enabled,
        cfg.flash_provider,
        cfg.min_profit_usdc,
        cfg.arb_max_borrow_usdc,
        cfg.min_base_fee_gwei
    );
    println!("pending_enabled=false — Arc pending RPC không dùng (filter/subscribe/getBlock pending).");

    let rpc_url = std::env::var("ARC_HTTP")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "https://rpc.mainnet.arc.io".to_string());

    let client = match RpcClient::new(rpc_url) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FAIL rpc client: {e}");
            return ExitCode::from(1);
        }
    };

    let block = match client.block_number_hex() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("FAIL eth_blockNumber: {e}");
            return ExitCode::from(1);
        }
    };
    let (latest_num, base_fee) = match client.latest_base_fee_hex() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("FAIL eth_getBlockByNumber latest: {e}");
            return ExitCode::from(1);
        }
    };

    println!(
        "rpc.ok eth_blockNumber={} latest.number={} baseFeePerGas={}",
        block, latest_num, base_fee
    );
    println!("A0 khung — không ký, không sendRaw, không live.");
    ExitCode::SUCCESS
}
