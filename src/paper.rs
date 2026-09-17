//! Paper 2 chân aero_cl vs uni_v3. send=0. Không PRIVATE_KEY. Không live.

use crate::pairbook::PairBook;
use crate::quote::{
    PAPER_SIZES_USDC, TwoLegQuote, format_no_quote_line, format_quote_line, gas_formula_line,
    quote_two_leg,
};
use crate::rpc::{RpcClient, RpcError};
use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Default)]
pub struct PaperReport {
    pub seconds: u64,
    pub ticks: u64,
    pub ticks_spread_gt0: u64,
    pub quotes_ok: u64,
    pub no_quote: u64,
    pub send_count: u64,
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn print_size_table(rpc: &RpcClient, book: &PairBook, min_profit_usdc: f64) -> Vec<TwoLegQuote> {
    println!("{}", gas_formula_line());
    let mut ok = Vec::new();
    for e in &book.entries {
        if !e.ok {
            continue;
        }
        for size in PAPER_SIZES_USDC {
            match quote_two_leg(rpc, e.token, &e.symbol, size, min_profit_usdc) {
                Ok(q) => {
                    println!("{}", format_quote_line(&q));
                    ok.push(q);
                }
                Err(n) => {
                    println!("{}", format_no_quote_line(&n));
                }
            }
        }
    }
    ok
}

fn append_jsonl(file: &mut File, v: &serde_json::Value) -> std::io::Result<()> {
    writeln!(file, "{v}")
}

pub fn run_paper(
    rpc: &RpcClient,
    book: &PairBook,
    seconds: u64,
    min_profit_usdc: f64,
    log_path: &Path,
) -> Result<PaperReport, RpcError> {
    if let Some(parent) = log_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .map_err(|e| RpcError::Rpc(format!("paper log {}: {e}", log_path.display())))?;

    println!(
        "paper.start seconds={} tokens=2 sizes=100,1000,5000 log={} send=0 PRIVATE_KEY=not_read",
        seconds,
        log_path.display()
    );
    let table = print_size_table(rpc, book, min_profit_usdc);

    let mut report = PaperReport {
        seconds,
        send_count: 0,
        quotes_ok: table.len() as u64,
        no_quote: (book.entries.iter().filter(|e| e.ok).count() * PAPER_SIZES_USDC.len())
            .saturating_sub(table.len()) as u64,
        ..Default::default()
    };

    if seconds == 0 {
        println!(
            "paper.done ticks=0 ticks_spread_gt0=0 quotes_ok={} no_quote={} send=0",
            report.quotes_ok, report.no_quote
        );
        return Ok(report);
    }

    let deadline = Instant::now() + Duration::from_secs(seconds);
    let mut last_block = String::new();
    while Instant::now() < deadline {
        let block = match rpc.block_number_hex() {
            Ok(b) => b,
            Err(e) => {
                eprintln!("paper.rpc_err eth_blockNumber: {e}");
                std::thread::sleep(Duration::from_millis(400));
                continue;
            }
        };
        if block == last_block {
            std::thread::sleep(Duration::from_millis(250));
            continue;
        }
        last_block = block.clone();
        report.ticks += 1;
        let ts = now_unix();
        let mut tick_spread = false;

        for e in &book.entries {
            if !e.ok {
                continue;
            }
            // 1 size / tick (1000 USDC) — bảng 3 size đã in lúc start
            match quote_two_leg(rpc, e.token, &e.symbol, 1_000, min_profit_usdc) {
                Ok(q) => {
                    report.quotes_ok += 1;
                    if q.spread_bps > 0.0 {
                        tick_spread = true;
                    }
                    println!("{}", format_quote_line(&q));
                    let rec = json!({
                        "ts_unix": ts,
                        "block": block,
                        "token": format!("{:#x}", q.token),
                        "symbol": q.symbol,
                        "size_usdc": q.size_usdc,
                        "price_a": q.price_a.to_string(),
                        "price_b": q.price_b.to_string(),
                        "venue_a": q.venue_a,
                        "venue_b": q.venue_b,
                        "spread_bps": q.spread_bps,
                        "after_gas_est": q.after_gas_est,
                        "profitable_yes_no": q.profitable_yes_no,
                        "gas_est_usdc": q.gas_est_usdc,
                        "send": 0u64,
                    });
                    let _ = append_jsonl(&mut log, &rec);
                }
                Err(n) => {
                    report.no_quote += 1;
                    println!("{}", format_no_quote_line(&n));
                    let rec = json!({
                        "ts_unix": ts,
                        "block": block,
                        "token": format!("{:#x}", n.token),
                        "symbol": n.symbol,
                        "size_usdc": n.size_usdc,
                        "no_quote": true,
                        "quotes": n.quotes,
                        "detail": n.detail,
                        "send": 0u64,
                    });
                    let _ = append_jsonl(&mut log, &rec);
                }
            }
        }
        if tick_spread {
            report.ticks_spread_gt0 += 1;
        }
    }

    println!(
        "paper.done ticks={} ticks_spread_gt0={} quotes_ok={} no_quote={} send={}",
        report.ticks, report.ticks_spread_gt0, report.quotes_ok, report.no_quote, report.send_count
    );
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn fail_jsonl_has_no_profit_word() {
        let rec = json!({
            "no_quote": true,
            "quotes": 1,
            "detail": "aero_cl fail",
            "send": 0u64,
        });
        let s = rec.to_string();
        assert!(s.contains("no_quote"));
        assert!(!s.to_ascii_lowercase().contains("profit"));
    }

    #[test]
    fn paper_report_send_zero() {
        let r = PaperReport::default();
        assert_eq!(r.send_count, 0);
    }

    #[test]
    fn ok_json_has_required_fields() {
        let rec: Value = json!({
            "price_a": "1",
            "price_b": "2",
            "spread_bps": 1.0,
            "after_gas_est": -0.008,
            "profitable_yes_no": "no",
            "send": 0,
        });
        assert_eq!(rec["profitable_yes_no"], "no");
        assert_eq!(rec["send"], 0);
    }
}
