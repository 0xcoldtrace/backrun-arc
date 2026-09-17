//! A3: paper quote EURC/cirBTC aero_cl vs uni_v3.
//! pending_enabled=false. Không PRIVATE_KEY, không sendRaw, không Morpho.flashLoan.

use std::path::Path;
use std::process::ExitCode;

use arc_arb::discover::run_discover;
use arc_arb::logs::MORPHO_BLUE;
use arc_arb::paper::run_paper;
use arc_arb::quote::quoter_status_line;
use arc_arb::watch::{aero_factory_status, pinned_venues_line, print_watch_candidates, watch_once};
use arc_arb::{Config, PairBook, RpcClient};

fn arg_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn arg_u64(args: &[String], name: &str) -> Option<u64> {
    let eq = format!("{name}=");
    for (i, a) in args.iter().enumerate() {
        if a == name {
            return args.get(i + 1).and_then(|s| s.parse().ok());
        }
        if let Some(rest) = a.strip_prefix(&eq) {
            return rest.parse().ok();
        }
    }
    None
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let watch_once_flag = arg_flag(&args, "--watch-once");
    let discover_flag = arg_flag(&args, "--discover");
    let paper_seconds = arg_u64(&args, "--paper-seconds");

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

    if discover_flag {
        println!("{}", pinned_venues_line());
        println!("morpho_pin={MORPHO_BLUE:#x} flashLoan_called=0 (A3 không gọi)");
        match aero_factory_status(&client) {
            Ok(s) => println!("aero_factory={s}"),
            Err(e) => println!("aero_factory=MISSING (getCode err: {e})"),
        }
        match run_discover(&client, cfg.min_depth_usd, Path::new(".")) {
            Ok(r) => {
                println!(
                    "discover.ok pass={} fail={} uni_v2_usdc={} ach_v2_usdc={} aero_pools={} v3_logs={} v4_logs={} send=0",
                    r.pass, r.fail, r.v2_pairs, r.ach_pairs, r.aero_pools, r.v3_pools_seen, r.v4_pools_seen
                );
                println!("A3 discover — không ký, không sendRaw, không live. Executor No-Go.");
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("FAIL discover: {e}");
                return ExitCode::from(1);
            }
        }
    }

    let book = PairBook::load(Path::new(&cfg.pairs_arb_path));
    println!(
        "pairbook.ok path={} entries={} header_only={} missing_file={} skipped_bad_lines={}",
        book.path.display(),
        book.entries.len(),
        book.header_only,
        book.missing_file,
        book.skipped_bad_lines
    );

    if let Some(secs) = paper_seconds {
        println!("{}", pinned_venues_line());
        println!("morpho_pin={MORPHO_BLUE:#x} flashLoan_called=0 (A3 không gọi)");
        match aero_factory_status(&client) {
            Ok(s) => println!("aero_factory={s}"),
            Err(e) => println!("aero_factory=MISSING (getCode err: {e})"),
        }
        println!("{}", quoter_status_line(&client));
        let log = Path::new("baocao/evidence/paper_vps.jsonl");
        match run_paper(&client, &book, secs, cfg.min_profit_usdc, log) {
            Ok(r) => {
                println!(
                    "paper.ok ticks={} ticks_spread_gt0={} send={} PRIVATE_KEY=not_read Executor=No-Go",
                    r.ticks, r.ticks_spread_gt0, r.send_count
                );
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("FAIL paper: {e}");
                return ExitCode::from(1);
            }
        }
    }

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
        println!("A3 khung — không ký, không sendRaw, không live. (thêm --watch-once / --discover / --paper-seconds N)");
        return ExitCode::SUCCESS;
    }

    println!("{}", pinned_venues_line());
    println!("morpho_pin={MORPHO_BLUE:#x} flashLoan_called=0 (A3 không gọi)");

    match aero_factory_status(&client) {
        Ok(s) => println!("aero_factory={s}"),
        Err(e) => println!("aero_factory=MISSING (getCode err: {e})"),
    }
    println!("{}", quoter_status_line(&client));

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
        "venue uni_v2={} ach_v2={} uni_v3={} uni_v4={} aero_cl={} unknown={}",
        report.venue_uni_v2,
        report.venue_ach_v2,
        report.venue_uni_v3,
        report.venue_uni_v4,
        report.venue_aero_cl,
        report.venue_unknown
    );
    println!("send={}", report.send_count);
    println!("{}", report.dual_venue_line());
    print_watch_candidates(&client, &book);
    println!("A3 — không ký, không sendRaw, không live. Executor No-Go.");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    #[test]
    fn paper_seconds_parse_eq_and_space() {
        let args = vec![
            "x".into(),
            "--paper-seconds".into(),
            "30".into(),
        ];
        let eq = vec!["x".into(), "--paper-seconds=30".into()];
        fn arg_u64(args: &[String], name: &str) -> Option<u64> {
            let p = format!("{name}=");
            for (i, a) in args.iter().enumerate() {
                if a == name {
                    return args.get(i + 1).and_then(|s| s.parse().ok());
                }
                if let Some(rest) = a.strip_prefix(&p) {
                    return rest.parse().ok();
                }
            }
            None
        }
        assert_eq!(arg_u64(&args, "--paper-seconds"), Some(30));
        assert_eq!(arg_u64(&eq, "--paper-seconds"), Some(30));
    }
}
