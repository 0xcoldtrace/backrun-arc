//! Quét V2 allPairs + V3 getPool/PoolCreated + Aero allPools + V4 Initialize (nếu đọc được).
//! Ghi pairs_arb.txt / pairs_arb.rejected.txt. Không copy BSC. Không Morpho.flashLoan.

use crate::logs::{
    ACHSWAP_V2_FACTORY, AERO_CL_FACTORY, UNI_V2_FACTORY, UNI_V3_FACTORY, UNI_V4_POOL_MANAGER, USDC,
    address_from_word_hex,
};
use crate::rpc::{RpcClient, RpcError};
use crate::vet::{
    depth_usd_from_usdc_reserve, read_symbol, token_balance, usdc_balance, vet_pool_tax, VetFail,
    MAX_TAX_BPS,
};
use alloy::primitives::{Address, U256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const BATCH: usize = 20;
const V3_FEES: [u32; 4] = [100, 500, 3000, 10_000];
const LOG_SPAN: u64 = 4000;
const LOG_CHUNKS: u64 = 25; // ~100k block
const V3_POOL_CREATED: &str = "0x783cca1c0412dd0d695e784568c96da2e9c22ff989357a2e8b1d9b2b4e6b7118";
const V4_INITIALIZE: &str = "0xdd466e674ea557f56295e2d0218a125ea4b4f0f6f3307b95f85e6110838d6438";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LegVenue {
    UniV2,
    AchV2,
    UniV3,
    AeroCl,
    UniV4,
}

impl LegVenue {
    pub fn as_str(self) -> &'static str {
        match self {
            LegVenue::UniV2 => "uni_v2",
            LegVenue::AchV2 => "achswap_v2",
            LegVenue::UniV3 => "uni_v3",
            LegVenue::AeroCl => "aero_cl",
            LegVenue::UniV4 => "uni_v4",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Leg {
    pub venue: LegVenue,
    pub pool: Address,
    pub usdc_reserve: U256,
    pub token_reserve: U256,
    pub depth_usd: f64,
}

#[derive(Debug, Clone)]
pub struct DiscoverReport {
    pub pass: usize,
    pub fail: usize,
    pub v2_pairs: usize,
    pub ach_pairs: usize,
    pub aero_pools: usize,
    pub v3_pools_seen: usize,
    pub v4_pools_seen: usize,
}

fn pad_u256(v: u64) -> String {
    format!("{v:064x}")
}

fn call_addr(rpc: &RpcClient, to: Address, sel: &str) -> Option<Address> {
    let h = rpc.eth_call(&format!("{to:#x}"), sel).ok()?;
    address_from_word_hex(&h).filter(|a| !a.is_zero())
}

fn batch_ok(rpc: &RpcClient, calls: &[(String, String)]) -> Vec<Option<String>> {
    std::thread::sleep(std::time::Duration::from_millis(80));
    match rpc.batch_eth_call(calls) {
        Ok(v) => v.into_iter().map(|r| r.ok()).collect(),
        Err(_) => {
            std::thread::sleep(std::time::Duration::from_millis(400));
            match rpc.batch_eth_call(calls) {
                Ok(v) => v.into_iter().map(|r| r.ok()).collect(),
                Err(_) => calls.iter().map(|_| None).collect(),
            }
        }
    }
}

fn word_u256(hex: &str, word: usize) -> Option<U256> {
    let s = hex.trim().trim_start_matches("0x");
    let start = word * 64;
    let end = start + 64;
    if s.len() < end {
        return None;
    }
    U256::from_str_radix(&s[start..end], 16).ok()
}

fn v2_usdc_token_legs(rpc: &RpcClient, factory: Address, venue: LegVenue) -> Result<Vec<(Address, Leg)>, RpcError> {
    let nhex = rpc.eth_call(&format!("{factory:#x}"), "0x574f2ba3")?;
    let n = u64::from_str_radix(nhex.trim_start_matches("0x"), 16).unwrap_or(0);
    println!("discover {} allPairsLength={n}", venue.as_str());
    let mut pairs = Vec::new();
    for chunk in (0..n).collect::<Vec<_>>().chunks(BATCH) {
        let calls: Vec<(String, String)> = chunk
            .iter()
            .map(|i| {
                (
                    format!("{factory:#x}"),
                    format!("0x1e3dd18b{}", pad_u256(*i)),
                )
            })
            .collect();
        for h in batch_ok(rpc, &calls) {
            if let Some(a) = h.and_then(|x| address_from_word_hex(&x)) {
                if !a.is_zero() {
                    pairs.push(a);
                }
            }
        }
    }
    let mut out = Vec::new();
    for chunk in pairs.chunks(BATCH) {
        let mut calls = Vec::new();
        for p in chunk {
            let ps = format!("{p:#x}");
            calls.push((ps.clone(), "0x0dfe1681".into()));
            calls.push((ps.clone(), "0xd21220a7".into()));
            calls.push((ps, "0x0902f1ac".into()));
        }
        let got = batch_ok(rpc, &calls);
        for (i, p) in chunk.iter().enumerate() {
            let t0 = got.get(i * 3).and_then(|x| x.as_ref()).and_then(|h| address_from_word_hex(h));
            let t1 = got.get(i * 3 + 1).and_then(|x| x.as_ref()).and_then(|h| address_from_word_hex(h));
            let res = got.get(i * 3 + 2).and_then(|x| x.as_ref());
            let (Some(t0), Some(t1), Some(reshex)) = (t0, t1, res) else {
                continue;
            };
            let r0 = word_u256(reshex, 0).unwrap_or(U256::ZERO);
            let r1 = word_u256(reshex, 1).unwrap_or(U256::ZERO);
            let (tok, usdc_r, tok_r) = if t0 == USDC {
                (t1, r0, r1)
            } else if t1 == USDC {
                (t0, r1, r0)
            } else {
                continue;
            };
            if tok == USDC {
                continue;
            }
            out.push((
                tok,
                Leg {
                    venue,
                    pool: *p,
                    usdc_reserve: usdc_r,
                    token_reserve: tok_r,
                    depth_usd: depth_usd_from_usdc_reserve(usdc_r),
                },
            ));
        }
    }
    Ok(out)
}

fn aero_usdc_legs(rpc: &RpcClient) -> Result<(u64, Vec<(Address, Leg)>), RpcError> {
    let nhex = rpc.eth_call(&format!("{AERO_CL_FACTORY:#x}"), "0xefde4e64")?;
    let n = u64::from_str_radix(nhex.trim_start_matches("0x"), 16).unwrap_or(0);
    println!("discover aero_cl allPoolsLength={n}");
    let mut pools = Vec::new();
    for i in 0..n {
        let h = rpc.eth_call(
            &format!("{AERO_CL_FACTORY:#x}"),
            &format!("0x41d1de97{}", pad_u256(i)),
        )?;
        if let Some(p) = address_from_word_hex(&h).filter(|a| !a.is_zero()) {
            pools.push(p);
        }
    }
    let mut out = Vec::new();
    for p in pools {
        let t0 = call_addr(rpc, p, "0x0dfe1681");
        let t1 = call_addr(rpc, p, "0xd21220a7");
        let (Some(t0), Some(t1)) = (t0, t1) else {
            continue;
        };
        let tok = if t0 == USDC {
            t1
        } else if t1 == USDC {
            t0
        } else {
            continue;
        };
        let usdc_r = usdc_balance(rpc, USDC, p).unwrap_or(U256::ZERO);
        let tok_r = token_balance(rpc, tok, p).unwrap_or(U256::ZERO);
        out.push((
            tok,
            Leg {
                venue: LegVenue::AeroCl,
                pool: p,
                usdc_reserve: usdc_r,
                token_reserve: tok_r,
                depth_usd: depth_usd_from_usdc_reserve(usdc_r),
            },
        ));
    }
    Ok((n, out))
}

fn pad_addr(a: Address) -> String {
    crate::logs::abi_word_addr(a)
}

fn encode_get_pool_v3(token: Address, fee: u32) -> String {
    format!("0x1698ee82{}{}{:064x}", pad_addr(USDC), pad_addr(token), fee)
}

fn v3_usdc_pool(rpc: &RpcClient, token: Address) -> Vec<Leg> {
    let fac = format!("{UNI_V3_FACTORY:#x}");
    let calls: Vec<(String, String)> = V3_FEES
        .iter()
        .map(|fee| (fac.clone(), encode_get_pool_v3(token, *fee)))
        .collect();
    let got = batch_ok(rpc, &calls);
    let mut legs = Vec::new();
    let mut pools: Vec<Address> = Vec::new();
    for h in got {
        if let Some(p) = h.and_then(|x| address_from_word_hex(&x)).filter(|a| !a.is_zero()) {
            if !pools.contains(&p) {
                pools.push(p);
            }
        }
    }
    for p in pools {
        let usdc_r = usdc_balance(rpc, USDC, p).unwrap_or(U256::ZERO);
        let tok_r = token_balance(rpc, token, p).unwrap_or(U256::ZERO);
        legs.push(Leg {
            venue: LegVenue::UniV3,
            pool: p,
            usdc_reserve: usdc_r,
            token_reserve: tok_r,
            depth_usd: depth_usd_from_usdc_reserve(usdc_r),
        });
    }
    legs
}

fn scan_v3_created(rpc: &RpcClient) -> (usize, BTreeSet<Address>) {
    let Ok(bn_hex) = rpc.block_number_hex() else {
        return (0, BTreeSet::new());
    };
    let Ok(bn) = u64::from_str_radix(bn_hex.trim_start_matches("0x"), 16) else {
        return (0, BTreeSet::new());
    };
    let mut tokens = BTreeSet::new();
    let mut nlog = 0usize;
    for k in 0..LOG_CHUNKS {
        let to = bn.saturating_sub(k * LOG_SPAN);
        let from = to.saturating_sub(LOG_SPAN.saturating_sub(1));
        let logs = rpc.get_logs(
            &format!("0x{from:x}"),
            &format!("0x{to:x}"),
            Some(&format!("{UNI_V3_FACTORY:#x}")),
            &[V3_POOL_CREATED],
        );
        let Ok(logs) = logs else {
            continue;
        };
        nlog += logs.len();
        for v in logs {
            let topics = v.get("topics").and_then(|t| t.as_array()).cloned().unwrap_or_default();
            if topics.len() < 3 {
                continue;
            }
            let t0 = topics[1].as_str().and_then(address_from_word_hex);
            let t1 = topics[2].as_str().and_then(address_from_word_hex);
            match (t0, t1) {
                (Some(a), Some(b)) if a == USDC => {
                    tokens.insert(b);
                }
                (Some(a), Some(b)) if b == USDC => {
                    tokens.insert(a);
                }
                _ => {}
            }
        }
    }
    (nlog, tokens)
}

fn scan_v4_init(rpc: &RpcClient) -> usize {
    let Ok(bn_hex) = rpc.block_number_hex() else {
        return 0;
    };
    let Ok(bn) = u64::from_str_radix(bn_hex.trim_start_matches("0x"), 16) else {
        return 0;
    };
    let mut n = 0usize;
    for k in 0..LOG_CHUNKS {
        let to = bn.saturating_sub(k * LOG_SPAN);
        let from = to.saturating_sub(LOG_SPAN.saturating_sub(1));
        if let Ok(logs) = rpc.get_logs(
            &format!("0x{from:x}"),
            &format!("0x{to:x}"),
            Some(&format!("{UNI_V4_POOL_MANAGER:#x}")),
            &[V4_INITIALIZE],
        ) {
            n += logs.len();
        }
    }
    n
}

fn best_leg(legs: &[Leg]) -> Option<&Leg> {
    legs.iter().max_by(|a, b| a.depth_usd.partial_cmp(&b.depth_usd).unwrap_or(std::cmp::Ordering::Equal))
}

pub fn run_discover(rpc: &RpcClient, min_depth_usd: f64, out_dir: &Path) -> Result<DiscoverReport, RpcError> {
    println!("discover.start quote=USDC {USDC:#x} min_depth_usd={min_depth_usd} formula=reserve_usdc*2/1e6");
    println!("discover.venues Kyber/1inch/LI.FI aggregator — không đưa vào pairbook. Curve/fomo không pin.");

    let uni_v2 = v2_usdc_token_legs(rpc, UNI_V2_FACTORY, LegVenue::UniV2)?;
    let ach_v2 = v2_usdc_token_legs(rpc, ACHSWAP_V2_FACTORY, LegVenue::AchV2)?;
    let (aero_n, aero) = aero_usdc_legs(rpc)?;
    let (v3_logs, v3_extra) = scan_v3_created(rpc);
    let v4_n = scan_v4_init(rpc);
    println!(
        "discover.scan uni_v2_usdc_pairs={} ach_v2_usdc_pairs={} aero_pools={} v3_PoolCreated_logs={} v3_usdc_tokens={} v4_Initialize_logs={} (V4 depth unread → không tính chân)",
        uni_v2.len(),
        ach_v2.len(),
        aero_n,
        v3_logs,
        v3_extra.len(),
        v4_n
    );

    let mut by_token: BTreeMap<Address, Vec<Leg>> = BTreeMap::new();
    for (t, leg) in uni_v2.iter().chain(ach_v2.iter()).chain(aero.iter()) {
        by_token.entry(*t).or_default().push(leg.clone());
    }
    // V3-only (cùng factory) không đủ 2 venue. Chỉ getPool token đã có V2/Aero.
    let tokens: Vec<Address> = by_token.keys().copied().collect();
    println!("discover.tokens_with_v2_or_aero={}", tokens.len());
    for (i, t) in tokens.iter().enumerate() {
        let extra = v3_usdc_pool(rpc, *t);
        if !extra.is_empty() {
            by_token.entry(*t).or_default().extend(extra);
        }
        if i > 0 && i % 50 == 0 {
            println!("discover.v3_getPool {}/{}", i, tokens.len());
        }
    }

    let mut pass_lines = Vec::new();
    let mut rej_lines = Vec::new();
    let mut pass = 0usize;
    let mut fail = 0usize;

    for (token, legs) in &by_token {
        if *token == USDC {
            rej_lines.push(format!("{token:#x}\tUSDC\treject=quote_is_usdc"));
            fail += 1;
            continue;
        }
        // deepest per venue
        let mut per: BTreeMap<LegVenue, Leg> = BTreeMap::new();
        for l in legs {
            per.entry(l.venue)
                .and_modify(|e| {
                    if l.depth_usd > e.depth_usd {
                        *e = l.clone();
                    }
                })
                .or_insert_with(|| l.clone());
        }
        let deep: Vec<Leg> = per
            .into_values()
            .filter(|l| l.depth_usd + f64::EPSILON >= min_depth_usd)
            .collect();
        if deep.len() < 2 {
            let vs: Vec<String> = legs
                .iter()
                .map(|l| format!("{}:{:.2}", l.venue.as_str(), l.depth_usd))
                .collect();
            rej_lines.push(format!(
                "{token:#x}\t?\treject=lt_2_venues_deep min={min_depth_usd} have={}",
                vs.join("|")
            ));
            fail += 1;
            continue;
        }
        let Some(vet_leg) = best_leg(&deep) else {
            rej_lines.push(format!("{token:#x}\t?\treject=no_leg"));
            fail += 1;
            continue;
        };
        let symbol = read_symbol(rpc, *token);
        let tax = match vet_pool_tax(rpc, vet_leg.pool, *token, vet_leg.token_reserve) {
            Ok(t) => t,
            Err(VetFail::Revert(s)) => {
                rej_lines.push(format!(
                    "{token:#x}\t{symbol}\treject=transfer_or_sell_revert {s}"
                ));
                fail += 1;
                continue;
            }
            Err(e) => {
                rej_lines.push(format!("{token:#x}\t{symbol}\treject={e}"));
                fail += 1;
                continue;
            }
        };
        let venues = {
            let mut v: Vec<&str> = deep.iter().map(|l| l.venue.as_str()).collect();
            v.sort();
            v.dedup();
            v.join("+")
        };
        let depth = deep
            .iter()
            .map(|l| l.depth_usd)
            .fold(f64::MAX, f64::min);
        pass_lines.push(format!(
            "{token:#x},{symbol},{venues},{depth:.2},{},{},ok",
            tax.buy_bps, tax.sell_bps
        ));
        pass += 1;
    }

    pass_lines.sort();
    rej_lines.sort();

    let header = format!(
        "# pairs_arb.txt — Arc mainnet (5042) backrun/arb candidates\n\
         # token,symbol,venues,depth_usd,tax_buy_bps,tax_sell_bps,ok\n\
         # Quote = USDC ERC-20 {USDC:#x} (6 dec). depth_usd = min(leg); leg = reserve_usdc * 2 / 1e6\n\
         # min_depth_usd={min_depth_usd}. tax cửa {MAX_TAX_BPS} bps. Không copy BSC. Kyber/1inch/LI.FI không phải venue.\n\
         # discovered_at_unix={}\n",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    );
    let mut pass_file = header.clone();
    for l in &pass_lines {
        pass_file.push_str(l);
        pass_file.push('\n');
    }
    fs::write(out_dir.join("pairs_arb.txt"), pass_file).map_err(|e| RpcError::Http(e.to_string()))?;

    let mut rej_file = format!(
        "# pairs_arb.rejected.txt — token FAIL vet/depth/venue\n# token,symbol,reason\n"
    );
    // use tab format already
    for l in &rej_lines {
        rej_file.push_str(l);
        rej_file.push('\n');
    }
    fs::write(out_dir.join("pairs_arb.rejected.txt"), rej_file)
        .map_err(|e| RpcError::Http(e.to_string()))?;

    println!("discover.done pass={pass} fail={fail} wrote pairs_arb.txt pairs_arb.rejected.txt");
    Ok(DiscoverReport {
        pass,
        fail,
        v2_pairs: uni_v2.len(),
        ach_pairs: ach_v2.len(),
        aero_pools: aero_n as usize,
        v3_pools_seen: v3_logs,
        v4_pools_seen: v4_n,
    })
}

#[cfg(test)]
mod tests {
    use super::LegVenue;
    #[test]
    fn venue_names() {
        assert_eq!(LegVenue::UniV2.as_str(), "uni_v2");
        assert_eq!(LegVenue::AeroCl.as_str(), "aero_cl");
    }
}
