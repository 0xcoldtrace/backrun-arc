//! eth_call dust buy rồi sell trên 1 pool (stateOverride code overlay).
//! Không ký, không sendRaw, không Morpho.flashLoan.

use crate::logs::{address_from_word_hex, decode_hex, parse_address};
use crate::rpc::{RpcClient, RpcError};
use alloy::primitives::{Address, U256};

const PAIR_PROBE: &str = include_str!("bytecode/PairTaxProbe.hex");
const WALLET_PROBE: &str = include_str!("bytecode/ProbeWallet.hex");
const PROBE_ADDR: &str = "0x1111111111111111111111111111111111111111";

/// Cửa tax: > 100 bps (1%) = FAIL.
pub const MAX_TAX_BPS: u32 = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaxReport {
    pub buy_bps: u32,
    pub sell_bps: u32,
    pub dust: U256,
    pub got: U256,
    pub returned: U256,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VetFail {
    Revert(String),
    Honeypot(&'static str),
    Tax { buy_bps: u32, sell_bps: u32 },
    Rpc(String),
}

impl std::fmt::Display for VetFail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VetFail::Revert(s) => write!(f, "revert:{s}"),
            VetFail::Honeypot(s) => write!(f, "honeypot:{s}"),
            VetFail::Tax { buy_bps, sell_bps } => {
                write!(f, "tax_gt_1pct buy={buy_bps}bps sell={sell_bps}bps")
            }
            VetFail::Rpc(s) => write!(f, "rpc:{s}"),
        }
    }
}

/// bps = (expected - got) * 10000 / expected. got >= expected → 0. expected=0 → None.
pub fn tax_bps(expected: U256, got: U256) -> Option<u32> {
    if expected.is_zero() {
        return None;
    }
    if got >= expected {
        return Some(0);
    }
    let delta = expected - got;
    let bps = delta.saturating_mul(U256::from(10_000u64)) / expected;
    Some(u32::try_from(bps).unwrap_or(u32::MAX))
}

pub fn dust_from_reserve(token_reserve: U256) -> U256 {
    let d = token_reserve / U256::from(100_000u64);
    if d.is_zero() {
        U256::from(1u64)
    } else {
        d
    }
}

fn strip_hex(s: &str) -> &str {
    s.trim().trim_start_matches("0x").trim_start_matches("0X")
}

fn pack_calldata(token: Address, dust: U256, probe: Address) -> String {
    let mut out = String::from("0x");
    out.push_str(&format!("{token:x}"));
    out.push_str(&format!("{dust:064x}"));
    out.push_str(&format!("{probe:x}"));
    out
}

fn word_u256(hex: &str, word: usize) -> Option<U256> {
    let s = strip_hex(hex);
    let start = word * 64;
    let end = start + 64;
    if s.len() < end {
        return None;
    }
    U256::from_str_radix(&s[start..end], 16).ok()
}

/// Overlay probe bytecode lên pair + ví giả. transfer dust token pair→probe rồi probe→pair.
pub fn vet_pool_tax(
    rpc: &RpcClient,
    pair: Address,
    token: Address,
    token_reserve: U256,
) -> Result<TaxReport, VetFail> {
    if token_reserve.is_zero() {
        return Err(VetFail::Honeypot("zero_token_reserve"));
    }
    let dust = dust_from_reserve(token_reserve);
    let probe = parse_address(PROBE_ADDR).ok_or_else(|| VetFail::Rpc("probe addr".into()))?;
    let data = pack_calldata(token, dust, probe);
    let pair_code = PAIR_PROBE.trim();
    let wallet_code = WALLET_PROBE.trim();
    let override_map = serde_json::json!({
        format!("{pair:#x}"): { "code": pair_code },
        PROBE_ADDR: { "code": wallet_code },
    });
    let hex = rpc
        .eth_call_ex(
            &format!("{pair:#x}"),
            &data,
            Some("0x0000000000000000000000000000000000000001"),
            Some(override_map),
        )
        .map_err(|e| match e {
            RpcError::Rpc(s) => VetFail::Revert(s),
            RpcError::Http(s) => VetFail::Rpc(s),
        })?;
    let got = word_u256(&hex, 0).ok_or_else(|| VetFail::Honeypot("decode_got"))?;
    let returned = word_u256(&hex, 1).ok_or_else(|| VetFail::Honeypot("decode_returned"))?;
    if got.is_zero() {
        return Err(VetFail::Honeypot("buy_got_zero"));
    }
    let buy_bps = tax_bps(dust, got).unwrap_or(0);
    let sell_bps = tax_bps(got, returned).unwrap_or(0);
    if buy_bps > MAX_TAX_BPS || sell_bps > MAX_TAX_BPS {
        return Err(VetFail::Tax { buy_bps, sell_bps });
    }
    Ok(TaxReport {
        buy_bps,
        sell_bps,
        dust,
        got,
        returned,
    })
}

pub fn u256_from_word_hex(s: &str) -> Option<U256> {
    let s = strip_hex(s);
    if s.is_empty() {
        return Some(U256::ZERO);
    }
    U256::from_str_radix(s, 16).ok()
}

pub fn decode_string_or_bytes32(hex: &str) -> Option<String> {
    let raw = decode_hex(hex)?;
    if raw.len() == 32 {
        let s: String = raw.iter().take_while(|b| **b != 0).map(|b| *b as char).collect();
        let s = s.chars().filter(|c| c.is_ascii_graphic() || *c == ' ').collect::<String>();
        if s.is_empty() {
            return None;
        }
        return Some(s);
    }
    if raw.len() < 64 {
        return None;
    }
    let off = U256::from_be_slice(&raw[0..32]);
    let off_us = usize::try_from(off).ok()?;
    if off_us + 32 > raw.len() {
        return None;
    }
    let len = U256::from_be_slice(&raw[off_us..off_us + 32]);
    let n = usize::try_from(len).ok()?;
    let start = off_us + 32;
    if start + n > raw.len() {
        return None;
    }
    let s = String::from_utf8_lossy(&raw[start..start + n])
        .chars()
        .filter(|c| c.is_ascii_graphic() || *c == ' ')
        .collect::<String>();
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

pub fn read_symbol(rpc: &RpcClient, token: Address) -> String {
    match rpc.eth_call(&format!("{token:#x}"), "0x95d89b41") {
        Ok(h) => decode_string_or_bytes32(&h).unwrap_or_else(|| format!("{:x}", token)[..6].to_string()),
        Err(_) => format!("{:x}", token)[..6].to_string(),
    }
}

/// USDC.balanceOf(who) — 6 dec ERC-20.
pub fn usdc_balance(rpc: &RpcClient, usdc: Address, who: Address) -> Option<U256> {
    let data = format!("0x70a08231{}", crate::logs::abi_word_addr(who));
    let h = rpc.eth_call(&format!("{usdc:#x}"), &data).ok()?;
    u256_from_word_hex(&h)
}

pub fn token_balance(rpc: &RpcClient, token: Address, who: Address) -> Option<U256> {
    let data = format!("0x70a08231{}", crate::logs::abi_word_addr(who));
    let h = rpc.eth_call(&format!("{token:#x}"), &data).ok()?;
    u256_from_word_hex(&h)
}

/// depth_usd = reserve_usdc * 2 / 1e6  (USDC ERC-20 6 dec).
pub fn depth_usd_from_usdc_reserve(reserve_usdc: U256) -> f64 {
    let v: f64 = reserve_usdc.to_string().parse().unwrap_or(0.0);
    v * 2.0 / 1_000_000.0
}

pub fn code_empty(hex: &str) -> bool {
    let s = hex.trim();
    s == "0x" || s == "0x0" || s.len() <= 2
}

pub fn must_addr(s: &str) -> Option<Address> {
    parse_address(s).or_else(|| address_from_word_hex(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tax_zero_when_full() {
        assert_eq!(tax_bps(U256::from(1000u64), U256::from(1000u64)), Some(0));
        assert_eq!(tax_bps(U256::from(1000u64), U256::from(1001u64)), Some(0));
    }

    #[test]
    fn tax_one_percent() {
        // 1% = 10 of 1000
        assert_eq!(tax_bps(U256::from(1000u64), U256::from(990u64)), Some(100));
        assert_eq!(tax_bps(U256::from(1000u64), U256::from(989u64)), Some(110));
    }

    #[test]
    fn tax_expected_zero() {
        assert_eq!(tax_bps(U256::ZERO, U256::from(1u64)), None);
    }

    #[test]
    fn dust_nonzero() {
        assert_eq!(dust_from_reserve(U256::ZERO), U256::from(1u64));
        assert_eq!(
            dust_from_reserve(U256::from(100_000u64)),
            U256::from(1u64)
        );
        assert_eq!(
            dust_from_reserve(U256::from(5_000_000u64)),
            U256::from(50u64)
        );
    }

    #[test]
    fn probe_bytecode_present() {
        assert!(PAIR_PROBE.trim().starts_with("0x60806040"));
        assert!(WALLET_PROBE.trim().starts_with("0x60806040"));
        assert!(PAIR_PROBE.trim().len() > 100);
    }

    #[test]
    fn pack_72_bytes() {
        let t = parse_address("0x3600000000000000000000000000000000000000").unwrap();
        let p = parse_address(PROBE_ADDR).unwrap();
        let d = pack_calldata(t, U256::from(100u64), p);
        // 2 + 40 + 64 + 40
        assert_eq!(d.len(), 2 + 40 + 64 + 40);
    }
}
