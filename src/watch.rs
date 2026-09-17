//! 1 head (HTTP latest = newHeads-equivalent) → eth_getLogs 1 block.
//! Không subscribe pending. 0 send.

use crate::logs::{
    ACHSWAP_V2_FACTORY, AERO_CL_FACTORY, DecodedSwap, SEL_FACTORY, SEL_TOKEN0, SEL_TOKEN1, SwapFamily,
    UNI_V2_FACTORY, UNI_V3_FACTORY, UNI_V4_POOL_MANAGER, Venue, address_from_word_hex, classify_venue,
    decode_swap, parse_raw_log, topic_hex, topic_v2, topic_v3, topic_v4,
};
use crate::pairbook::PairBook;
use crate::quote::quotes_for_token;
use crate::rpc::{RpcClient, RpcError};
use alloy::primitives::Address;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct WatchOnceReport {
    pub block_hex: String,
    pub logs_fetched: usize,
    pub swaps_decoded: usize,
    pub family_v2: usize,
    pub family_v3: usize,
    pub family_v4: usize,
    pub venue_uni_v2: usize,
    pub venue_ach_v2: usize,
    pub venue_uni_v3: usize,
    pub venue_uni_v4: usize,
    pub venue_aero_cl: usize,
    pub venue_unknown: usize,
    pub send_count: u64,
    pub dual_venue_tokens: Option<usize>,
    pub dual_venue_blocks: u64,
}

impl WatchOnceReport {
    pub fn dual_venue_line(&self) -> String {
        match self.dual_venue_tokens {
            Some(n) if n > 0 => format!(
                "dual_venue tokens_ge_2_pools={} ({} blocks logs)",
                n, self.dual_venue_blocks
            ),
            _ => "dual_venue: CHƯA ĐO".to_string(),
        }
    }
}

pub fn watch_once(rpc: &RpcClient, dual_venue_blocks: u64) -> Result<WatchOnceReport, RpcError> {
    let block_hex = rpc.block_number_hex()?;
    let t2 = topic_hex(topic_v2());
    let t3 = topic_hex(topic_v3());
    let t4 = topic_hex(topic_v4());

    // V2/V3 Swap emit trên pair/pool, không phải factory → getLogs theo topic0, 1 block.
    // V4 Swap emit trên PoolManager.
    let logs_v2 = rpc.get_logs(&block_hex, &block_hex, None, &[&t2])?;
    let logs_v3 = rpc.get_logs(&block_hex, &block_hex, None, &[&t3])?;
    let logs_v4 = rpc.get_logs(
        &block_hex,
        &block_hex,
        Some(&format!("{UNI_V4_POOL_MANAGER:#x}")),
        &[&t4],
    )?;

    let mut all = Vec::new();
    all.extend(logs_v2);
    all.extend(logs_v3);
    all.extend(logs_v4);
    let logs_fetched = all.len();

    let mut decoded: Vec<DecodedSwap> = Vec::new();
    for v in &all {
        let Some(raw) = parse_raw_log(v) else {
            continue;
        };
        if let Some(d) = decode_swap(raw.address, &raw.topics, &raw.data) {
            decoded.push(d);
        }
    }

    let mut factory_cache: HashMap<Address, Option<Address>> = HashMap::new();
    let mut report = WatchOnceReport {
        block_hex: block_hex.clone(),
        logs_fetched,
        swaps_decoded: decoded.len(),
        send_count: 0,
        dual_venue_blocks,
        ..Default::default()
    };

    for d in &decoded {
        match d.family() {
            SwapFamily::V2 => report.family_v2 += 1,
            SwapFamily::V3 => report.family_v3 += 1,
            SwapFamily::V4 => report.family_v4 += 1,
        }
        let factory = match d.family() {
            SwapFamily::V4 => None,
            SwapFamily::V2 | SwapFamily::V3 => {
                let addr = d.address();
                if let Some(hit) = factory_cache.get(&addr) {
                    *hit
                } else {
                    let got = rpc
                        .eth_call(&format!("{addr:#x}"), SEL_FACTORY)
                        .ok()
                        .and_then(|h| address_from_word_hex(&h));
                    factory_cache.insert(addr, got);
                    got
                }
            }
        };
        match classify_venue(d.family(), d.address(), factory) {
            Venue::UniV2 => report.venue_uni_v2 += 1,
            Venue::AchSwapV2 => report.venue_ach_v2 += 1,
            Venue::UniV3 => report.venue_uni_v3 += 1,
            Venue::UniV4 => report.venue_uni_v4 += 1,
            Venue::AeroCl => report.venue_aero_cl += 1,
            Venue::Unknown => report.venue_unknown += 1,
        }
    }

    report.dual_venue_tokens = scan_dual_venue(rpc, &block_hex, dual_venue_blocks, &t2, &t3, &t4);
    Ok(report)
}

/// 20 block Swap logs → token0/token1 trên pair V2/V3. ≥2 pool thì Some(n); 0 hoặc lỗi → None (CHƯA ĐO).
fn scan_dual_venue(
    rpc: &RpcClient,
    latest_hex: &str,
    blocks: u64,
    t2: &str,
    t3: &str,
    t4: &str,
) -> Option<usize> {
    if blocks == 0 {
        return None;
    }
    let latest = u64::from_str_radix(latest_hex.trim_start_matches("0x"), 16).ok()?;
    let from = latest.saturating_sub(blocks.saturating_sub(1));
    let from_hex = format!("0x{from:x}");
    let mut logs = Vec::new();
    logs.extend(rpc.get_logs(&from_hex, latest_hex, None, &[t2]).ok()?);
    logs.extend(rpc.get_logs(&from_hex, latest_hex, None, &[t3]).ok()?);
    logs.extend(
        rpc.get_logs(
            &from_hex,
            latest_hex,
            Some(&format!("{UNI_V4_POOL_MANAGER:#x}")),
            &[t4],
        )
        .ok()?,
    );

    let mut v2v3_pools: HashSet<Address> = HashSet::new();
    for v in &logs {
        let Some(raw) = parse_raw_log(v) else {
            continue;
        };
        let Some(d) = decode_swap(raw.address, &raw.topics, &raw.data) else {
            continue;
        };
        match d {
            DecodedSwap::V2 { address, .. } | DecodedSwap::V3 { address, .. } => {
                v2v3_pools.insert(address);
            }
            DecodedSwap::V4 { .. } => {}
        }
    }

    let mut token_pools: HashMap<Address, HashSet<Address>> = HashMap::new();
    for pool in v2v3_pools {
        let t0 = rpc
            .eth_call(&format!("{pool:#x}"), SEL_TOKEN0)
            .ok()
            .and_then(|h| address_from_word_hex(&h));
        let t1 = rpc
            .eth_call(&format!("{pool:#x}"), SEL_TOKEN1)
            .ok()
            .and_then(|h| address_from_word_hex(&h));
        for tok in [t0, t1].into_iter().flatten() {
            token_pools.entry(tok).or_default().insert(pool);
        }
    }
    let n = token_pools.values().filter(|s| s.len() >= 2).count();
    if n == 0 {
        None
    } else {
        Some(n)
    }
}

/// Aero Lite CLFactory Arc: PIN khi getCode != 0x. Base factory = negative control.
pub fn aero_factory_status(rpc: &RpcClient) -> Result<String, RpcError> {
    const AERO_BASE: &str = "0x420DD381b31aEf6683db6B902084cB0FFECe40Da";
    let base = rpc.get_code_hex(AERO_BASE)?;
    let base_empty = base == "0x" || base == "0x0" || base.len() <= 2;
    let code = rpc.get_code_hex(&format!("{AERO_CL_FACTORY:#x}"))?;
    let empty = code == "0x" || code == "0x0" || code.len() <= 2;
    if empty {
        return Ok(format!(
            "MISSING (Base {AERO_BASE} empty={base_empty}; truncated 0xb89d…d03 not on chain)"
        ));
    }
    let nhex = rpc.eth_call(&format!("{AERO_CL_FACTORY:#x}"), "0xefde4e64")?;
    let n = u64::from_str_radix(nhex.trim_start_matches("0x"), 16).unwrap_or(0);
    let slot0 = rpc.eth_call(
        &format!("{AERO_CL_FACTORY:#x}"),
        "0x41d1de970000000000000000000000000000000000000000000000000000000000000000",
    )?;
    Ok(format!(
        "PINNED {AERO_CL_FACTORY:#x} allPoolsLength={n} allPools(0)={} base_neg_empty={base_empty}",
        crate::logs::address_from_word_hex(&slot0)
            .map(|a| format!("{a:#x}"))
            .unwrap_or_else(|| "err".into())
    ))
}

pub fn pinned_venues_line() -> String {
    format!(
        "venues uni_v2={UNI_V2_FACTORY:#x} uni_v3={UNI_V3_FACTORY:#x} uni_v4_pm={UNI_V4_POOL_MANAGER:#x} ach_v2={ACHSWAP_V2_FACTORY:#x} aero_cl={AERO_CL_FACTORY:#x}"
    )
}

/// In candidate nếu ≥2 venue quote được. Thiếu quote → no_quote. Cấm bịa profit.
pub fn print_watch_candidates(rpc: &RpcClient, book: &PairBook) {
    if book.entries.is_empty() {
        println!("candidates=0 pairbook_empty_or_header_only — no_quote (không bịa profit)");
        return;
    }
    let mut n_ok = 0u32;
    let mut n_no = 0u32;
    for e in &book.entries {
        if !e.ok {
            continue;
        }
        let qs = quotes_for_token(rpc, e.token, &e.venues);
        if qs.len() >= 2 {
            n_ok += 1;
            let v = qs
                .iter()
                .map(|q| q.venue.as_str())
                .collect::<Vec<_>>()
                .join("+");
            println!(
                "candidate token={:#x} symbol={} venues={} quote=ok quote_venues={v} (không in profit)",
                e.token, e.symbol, e.venues
            );
        } else {
            n_no += 1;
            println!(
                "no_quote token={:#x} symbol={} venues={} quotes={}",
                e.token,
                e.symbol,
                e.venues,
                qs.len()
            );
        }
    }
    println!("candidates_quoted={n_ok} no_quote={n_no} profit=not_computed");
}
