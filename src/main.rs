//! A1: load config + pairbook + --watch-once (1 block Swap logs).
//! pending_enabled=false. Không PRIVATE_KEY, không sendRaw.

use std::path::Path;
use std::process::ExitCode;

use arc_arb::logs::MORPHO_BLUE;
use arc_arb::watch::{aero_factory_status, pinned_venues_line, watch_once};
use arc_arb::{Config, PairBook, RpcClient};

fn main() -> ExitCode {
    let watch_once_flag = std::env::args().any(|a| a == "--watch-once");

    let cfg = match Config::from_path(Path::new("config.toml")) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("FAIL {e}");
            return ExitCode::from(1);
        }
    };

    println!(
        "config.ok strategy={} chain_id={} dry_run={} allow_live={} bot_armed={} live_mode={} pending_enabled={} flash_provider={} min_profit_usdc={} arb_max_borrow_usdc={} pairs_min_swap_usdc={} min_depth_usd={} min_base_fee_gwei={}",
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
        cfg.pairs_min_swap_usdc,
        cfg.min_depth_usd,
        cfg.min_base_fee_gwei
    );
    println!("pending_enabled=false — Arc pending RPC không dùng (filter/subscribe/getBlock pending).");
    println!("rpc_crate=alloy (sol-types decode); HTTP JSON-RPC = reqwest blocking + UA");

    let book = PairBook::load(Path::new(&cfg.pairs_arb_path));
    println!(
        "pairbook.ok path={} entries={} header_only={} missing_file={} skipped_bad_lines={}",
        book.path.display(),
        book.entries.len(),
        book.header_only,
        book.missing_file,
        book.skipped_bad_lines
    );

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

    if !watch_once_flag {
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
        println!("A1 khung — không ký, không sendRaw, không live. (thêm --watch-once để decode 1 block)");
        return ExitCode::SUCCESS;
    }

    println!("{}", pinned_venues_line());
    println!(
        "morpho_pin={MORPHO_BLUE:#x} flashLoan_called=0 (A1 không gọi)"
    );

    match aero_factory_status(&client) {
        Ok(s) => println!("aero_factory={s}"),
        Err(e) => println!("aero_factory=MISSING (getCode err: {e})"),
    }

    let report = match watch_once(&client, 20) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("FAIL watch-once: {e}");
            return ExitCode::from(1);
        }
    };

    println!(
        "watch-once.ok source=http_latest_head block={} logs={} swaps_decoded={} family_v2={} family_v3={} family_v4={}",
        report.block_hex,
        report.logs_fetched,
        report.swaps_decoded,
        report.family_v2,
        report.family_v3,
        report.family_v4
    );
    println!(
        "venue uni_v2={} ach_v2={} uni_v3={} uni_v4={} unknown={}",
        report.venue_uni_v2,
        report.venue_ach_v2,
        report.venue_uni_v3,
        report.venue_uni_v4,
        report.venue_unknown
    );
    println!("send={}", report.send_count);
    println!("{}", report.dual_venue_line());
    println!("min_swap/depth loaded from config — chưa sim đa venue, không filter USDC ảo.");
    println!("A1 — không ký, không sendRaw, không live. Executor No-Go.");
    ExitCode::SUCCESS
}
