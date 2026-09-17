//! Quote đọc (eth_call). Không bịa profit. 0 send.
//! Uni V3: QuoterV2. Aero CL: Quoter MISSING → eth_call swap-static MiniQuoter overlay.

use crate::logs::{
    AERO_CL_FACTORY, UNI_V2_ROUTER, UNI_V3_FACTORY, UNI_V3_QUOTER, USDC, abi_word_addr,
    address_from_word_hex,
};
use crate::rpc::{RpcClient, RpcError};
use alloy::primitives::{Address, I256, U256};
use alloy::sol_types::SolCall;

alloy::sol_types::sol! {
    #![sol(alloy_sol_types = alloy::sol_types)]
    interface IRouterV2 {
        function getAmountsOut(uint256 amountIn, address[] path) external view returns (uint256[] amounts);
    }
    struct QuoteExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint256 amountIn;
        uint24 fee;
        uint160 sqrtPriceLimitX96;
    }
    interface IQuoterV2 {
        function quoteExactInputSingle(QuoteExactInputSingleParams params)
            external
            returns (uint256 amountOut, uint160 sqrtPriceX96After, uint32 initializedTicksCrossed, uint256 gasEstimate);
    }
    interface IUniV3Factory {
        function getPool(address tokenA, address tokenB, uint24 fee) external view returns (address pool);
    }
    interface IMiniQuoter {
        function quote(address pool, bool zeroForOne, int256 amountSpecified)
            external
            returns (int256 amount0, int256 amount1);
    }
}

const V3_FEES: [u32; 4] = [100, 500, 3000, 10_000];
const DUST_USDC: u64 = 1_000_000; // 1 USDC (6 dec)
const AERO_TICKS: [i32; 6] = [1, 10, 50, 100, 200, 2000];

/// Overlay address for MiniQuoter (stateOverride code). Không gửi tx.
const MINI_QUOTER_ADDR: Address =
    alloy::primitives::address!("0x2222222222222222222222222222222222222222");
const MINI_QUOTER_CODE: &str = include_str!("bytecode/MiniQuoter.hex");

/// Paper 2-leg gas cap. Công thức: gas_est_wei = 20 gwei * GAS_CAP; native USDC 18 dec.
/// gas_est_usdc = gas_est_wei / 1e18.
pub const PAPER_GAS_GWEI: u64 = 20;
pub const PAPER_GAS_CAP: u64 = 400_000;
pub const PAPER_SIZES_USDC: [u64; 3] = [100, 1_000, 5_000];

#[derive(Debug, Clone)]
pub struct VenueQuote {
    pub venue: String,
    pub amount_out: U256,
}

#[derive(Debug, Clone)]
pub struct TwoLegQuote {
    pub token: Address,
    pub symbol: String,
    pub size_usdc: u64,
    pub size_usdc6: U256,
    pub price_a: U256,
    pub price_b: U256,
    pub venue_a: String,
    pub venue_b: String,
    pub spread_bps: f64,
    pub after_gas_est: f64,
    pub profitable_yes_no: String,
    pub gas_est_usdc: f64,
    pub rt_usdc6_aero_then_v3: Option<U256>,
    pub rt_usdc6_v3_then_aero: Option<U256>,
}

#[derive(Debug, Clone)]
pub struct NoQuote {
    pub token: Address,
    pub symbol: String,
    pub size_usdc: u64,
    pub quotes: usize,
    pub detail: String,
}

/// gas_est_wei = 20e9 * 400_000 = 8e15; native USDC 18 dec → / 1e18 = 0.008 USDC.
pub fn gas_est_usdc() -> f64 {
    let wei = (PAPER_GAS_GWEI as u128)
        .saturating_mul(1_000_000_000u128)
        .saturating_mul(PAPER_GAS_CAP as u128);
    (wei as f64) / 1e18_f64
}

pub fn gas_formula_line() -> String {
    format!(
        "gas_formula gas_est_wei={}e9*{PAPER_GAS_CAP} native_usdc_18dec gas_est_usdc={:.6} (không bịa)",
        PAPER_GAS_GWEI,
        gas_est_usdc()
    )
}

pub fn spread_bps(a: U256, b: U256) -> Option<f64> {
    if a.is_zero() || b.is_zero() {
        return None;
    }
    let min = a.min(b);
    let max = a.max(b);
    let min_f: f64 = min.to_string().parse().ok()?;
    let max_f: f64 = max.to_string().parse().ok()?;
    if min_f <= 0.0 {
        return None;
    }
    Some((max_f - min_f) * 10_000.0 / min_f)
}

pub fn quote_v2_usdc_to_token(rpc: &RpcClient, token: Address) -> Result<U256, RpcError> {
    let data = IRouterV2::getAmountsOutCall {
        amountIn: U256::from(DUST_USDC),
        path: vec![USDC, token],
    }
    .abi_encode();
    let hex = rpc.eth_call(
        &format!("{UNI_V2_ROUTER:#x}"),
        &format!("0x{}", alloy::primitives::hex::encode(&data)),
    )?;
    let amounts = IRouterV2::getAmountsOutCall::abi_decode_returns(
        &crate::logs::decode_hex(&hex).unwrap_or_default(),
    )
    .map_err(|e| RpcError::Rpc(format!("getAmountsOut decode: {e}")))?;
    amounts
        .last()
        .copied()
        .ok_or_else(|| RpcError::Rpc("getAmountsOut empty".into()))
}

pub fn quote_v3_exact(
    rpc: &RpcClient,
    token_in: Address,
    token_out: Address,
    fee: u32,
    amount_in: U256,
) -> Result<U256, RpcError> {
    let params = QuoteExactInputSingleParams {
        tokenIn: token_in,
        tokenOut: token_out,
        amountIn: amount_in,
        fee: alloy::primitives::aliases::U24::try_from(fee)
            .map_err(|e| RpcError::Rpc(e.to_string()))?,
        sqrtPriceLimitX96: alloy::primitives::aliases::U160::ZERO,
    };
    let data = IQuoterV2::quoteExactInputSingleCall { params }.abi_encode();
    let hex = rpc.eth_call(
        &format!("{UNI_V3_QUOTER:#x}"),
        &format!("0x{}", alloy::primitives::hex::encode(&data)),
    )?;
    let decoded = IQuoterV2::quoteExactInputSingleCall::abi_decode_returns(
        &crate::logs::decode_hex(&hex).unwrap_or_default(),
    )
    .map_err(|e| RpcError::Rpc(format!("quoter decode: {e}")))?;
    Ok(decoded.amountOut)
}

pub fn quote_v3_usdc_to_token(
    rpc: &RpcClient,
    token: Address,
    fee: u32,
) -> Result<U256, RpcError> {
    quote_v3_exact(rpc, USDC, token, fee, U256::from(DUST_USDC))
}

pub fn uni_v3_get_pool(rpc: &RpcClient, token: Address, fee: u32) -> Option<Address> {
    uni_v3_get_pool_pair(rpc, USDC, token, fee)
}

pub fn uni_v3_get_pool_pair(
    rpc: &RpcClient,
    a: Address,
    b: Address,
    fee: u32,
) -> Option<Address> {
    let fee_u = alloy::primitives::aliases::U24::try_from(fee).ok()?;
    let data = IUniV3Factory::getPoolCall {
        tokenA: a,
        tokenB: b,
        fee: fee_u,
    }
    .abi_encode();
    let hex = rpc
        .eth_call(
            &format!("{UNI_V3_FACTORY:#x}"),
            &format!("0x{}", alloy::primitives::hex::encode(&data)),
        )
        .ok()?;
    crate::logs::address_from_word_hex(&hex).filter(|p| !p.is_zero())
}

/// First V3 fee that has a pool and a non-zero quote for amount_in USDC→token.
pub fn v3_quote_usdc_to_token(
    rpc: &RpcClient,
    token: Address,
    amount_in: U256,
) -> Option<(u32, U256)> {
    for fee in V3_FEES {
        if uni_v3_get_pool(rpc, token, fee).is_none() {
            continue;
        }
        if let Ok(a) = quote_v3_exact(rpc, USDC, token, fee, amount_in) {
            if !a.is_zero() {
                return Some((fee, a));
            }
        }
    }
    None
}

pub fn v3_quote_token_to_usdc(
    rpc: &RpcClient,
    token: Address,
    fee: u32,
    amount_in: U256,
) -> Result<U256, RpcError> {
    quote_v3_exact(rpc, token, USDC, fee, amount_in)
}

fn pad_i24(ts: i32) -> String {
    let v = ts as i32 as u32 & 0x00ff_ffff;
    format!("{v:064x}")
}

pub fn aero_get_pool(rpc: &RpcClient, token: Address, tick_spacing: i32) -> Option<Address> {
    let data = format!(
        "0x28af8d0b{}{}{}",
        abi_word_addr(USDC),
        abi_word_addr(token),
        pad_i24(tick_spacing)
    );
    let hex = rpc
        .eth_call(&format!("{AERO_CL_FACTORY:#x}"), &data)
        .ok()?;
    address_from_word_hex(&hex).filter(|p| !p.is_zero())
}

pub fn aero_usdc_pool(rpc: &RpcClient, token: Address) -> Option<Address> {
    for ts in AERO_TICKS {
        if let Some(p) = aero_get_pool(rpc, token, ts) {
            return Some(p);
        }
    }
    None
}

fn call_token0(rpc: &RpcClient, pool: Address) -> Option<Address> {
    let h = rpc.eth_call(&format!("{pool:#x}"), crate::logs::SEL_TOKEN0).ok()?;
    address_from_word_hex(&h).filter(|a| !a.is_zero())
}

fn call_token1(rpc: &RpcClient, pool: Address) -> Option<Address> {
    let h = rpc.eth_call(&format!("{pool:#x}"), crate::logs::SEL_TOKEN1).ok()?;
    address_from_word_hex(&h).filter(|a| !a.is_zero())
}

/// Aero Slipstream: eth_call pool.swap qua MiniQuoter overlay (swap-static). Không sendRaw.
pub fn quote_aero_exact(
    rpc: &RpcClient,
    pool: Address,
    token_in: Address,
    token_out: Address,
    amount_in: U256,
) -> Result<U256, RpcError> {
    let t0 = call_token0(rpc, pool).ok_or_else(|| RpcError::Rpc("aero token0".into()))?;
    let t1 = call_token1(rpc, pool).ok_or_else(|| RpcError::Rpc("aero token1".into()))?;
    let zero_for_one = if token_in == t0 && token_out == t1 {
        true
    } else if token_in == t1 && token_out == t0 {
        false
    } else {
        return Err(RpcError::Rpc("aero token not in pool".into()));
    };
    let amt_i = I256::try_from(amount_in)
        .map_err(|_| RpcError::Rpc("aero amountIn does not fit int256".into()))?;
    let data = IMiniQuoter::quoteCall {
        pool,
        zeroForOne: zero_for_one,
        amountSpecified: amt_i,
    }
    .abi_encode();
    let code = MINI_QUOTER_CODE.trim();
    let ov = serde_json::json!({
        format!("{MINI_QUOTER_ADDR:#x}"): { "code": code }
    });
    let hex = rpc.eth_call_ex(
        &format!("{MINI_QUOTER_ADDR:#x}"),
        &format!("0x{}", alloy::primitives::hex::encode(&data)),
        None,
        Some(ov),
    )?;
    let decoded = IMiniQuoter::quoteCall::abi_decode_returns(
        &crate::logs::decode_hex(&hex).unwrap_or_default(),
    )
    .map_err(|e| RpcError::Rpc(format!("aero mini-quoter decode: {e}")))?;
    let out_i = if zero_for_one {
        decoded.amount1
    } else {
        decoded.amount0
    };
    if !out_i.is_negative() {
        return Err(RpcError::Rpc("aero quote expected negative out delta".into()));
    }
    Ok(out_i.unsigned_abs())
}

/// Thử quote 2 venue. Thiếu quote → Vec len < 2 (caller in no_quote). Không bịa profit.
pub fn quotes_for_token(rpc: &RpcClient, token: Address, venues: &str) -> Vec<VenueQuote> {
    let mut out = Vec::new();
    if venues.contains("uni_v2") {
        if let Ok(a) = quote_v2_usdc_to_token(rpc, token) {
            if !a.is_zero() {
                out.push(VenueQuote {
                    venue: "uni_v2".into(),
                    amount_out: a,
                });
            }
        }
    }
    if venues.contains("uni_v3") {
        if let Some((fee, a)) = v3_quote_usdc_to_token(rpc, token, U256::from(DUST_USDC)) {
            out.push(VenueQuote {
                venue: format!("uni_v3:{fee}"),
                amount_out: a,
            });
        }
    }
    if venues.contains("aero_cl") {
        if let Some(pool) = aero_usdc_pool(rpc, token) {
            if let Ok(a) = quote_aero_exact(rpc, pool, USDC, token, U256::from(DUST_USDC)) {
                if !a.is_zero() {
                    out.push(VenueQuote {
                        venue: "aero_cl".into(),
                        amount_out: a,
                    });
                }
            }
        }
    }
    out
}

fn u256_to_f64(v: U256) -> f64 {
    v.to_string().parse().unwrap_or(0.0)
}

/// 2 chân aero_cl vs uni_v3. 1 chân eth_call fail → NoQuote (cấm chữ profit).
pub fn quote_two_leg(
    rpc: &RpcClient,
    token: Address,
    symbol: &str,
    size_usdc: u64,
    min_profit_usdc: f64,
) -> Result<TwoLegQuote, NoQuote> {
    let size6 = U256::from(size_usdc).saturating_mul(U256::from(1_000_000u64));
    let aero_pool = aero_usdc_pool(rpc, token);
    let v3 = v3_quote_usdc_to_token(rpc, token, size6);

    let mut n = 0usize;
    let mut detail = Vec::new();
    let price_a = match aero_pool {
        Some(pool) => match quote_aero_exact(rpc, pool, USDC, token, size6) {
            Ok(a) if !a.is_zero() => {
                n += 1;
                Some((pool, a))
            }
            Ok(_) => {
                detail.push("aero_cl amountOut=0".into());
                None
            }
            Err(e) => {
                detail.push(format!("aero_cl fail:{e}"));
                None
            }
        },
        None => {
            detail.push("aero_cl pool=none".into());
            None
        }
    };
    let price_b = match v3 {
        Some((fee, a)) => {
            n += 1;
            Some((fee, a))
        }
        None => {
            detail.push("uni_v3 fail_or_empty".into());
            None
        }
    };

    let (Some((aero_pool, pa)), Some((fee, pb))) = (price_a, price_b) else {
        return Err(NoQuote {
            token,
            symbol: symbol.to_string(),
            size_usdc,
            quotes: n,
            detail: detail.join(";"),
        });
    };

    let spread = spread_bps(pa, pb).unwrap_or(0.0);
    let gas = gas_est_usdc();

    let rt_aero_v3 = v3_quote_token_to_usdc(rpc, token, fee, pa)
        .ok()
        .filter(|u| !u.is_zero());
    let rt_v3_aero = quote_aero_exact(rpc, aero_pool, token, USDC, pb)
        .ok()
        .filter(|u| !u.is_zero());

    let best_rt = match (rt_aero_v3, rt_v3_aero) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };

    let size_f = size_usdc as f64;
    let after = if let Some(rt) = best_rt {
        (u256_to_f64(rt) / 1e6_f64) - size_f - gas
    } else {
        // both venues quoted one-way; round-trip unread → after_gas from one-way spread, không yes
        size_f * spread / 10_000.0 - gas
    };
    let profitable = if best_rt.is_some() && after >= min_profit_usdc {
        "yes"
    } else {
        "no"
    };

    Ok(TwoLegQuote {
        token,
        symbol: symbol.to_string(),
        size_usdc,
        size_usdc6: size6,
        price_a: pa,
        price_b: pb,
        venue_a: "aero_cl".into(),
        venue_b: format!("uni_v3:{fee}"),
        spread_bps: spread,
        after_gas_est: after,
        profitable_yes_no: profitable.into(),
        gas_est_usdc: gas,
        rt_usdc6_aero_then_v3: rt_aero_v3,
        rt_usdc6_v3_then_aero: rt_v3_aero,
    })
}

pub fn format_quote_line(q: &TwoLegQuote) -> String {
    format!(
        "quote token={:#x} symbol={} size_usdc={} price_a={} price_b={} venue_a={} venue_b={} spread_bps={:.4} after_gas_est={:.6} profitable_yes_no={} send=0",
        q.token,
        q.symbol,
        q.size_usdc,
        q.price_a,
        q.price_b,
        q.venue_a,
        q.venue_b,
        q.spread_bps,
        q.after_gas_est,
        q.profitable_yes_no
    )
}

pub fn format_no_quote_line(n: &NoQuote) -> String {
    format!(
        "no_quote token={:#x} symbol={} size_usdc={} quotes={} detail={} send=0",
        n.token, n.symbol, n.size_usdc, n.quotes, n.detail
    )
}

pub fn quoter_status_line(rpc: &RpcClient) -> String {
    let v3 = match rpc.get_code_hex(&format!("{UNI_V3_QUOTER:#x}")) {
        Ok(c) if c == "0x" || c == "0x0" || c.len() <= 2 => "uni_v3_quoter=MISSING".into(),
        Ok(c) => {
            let n = (c.len().saturating_sub(2)) / 2;
            format!("uni_v3_quoter=PINNED {UNI_V3_QUOTER:#x} bytes={n}")
        }
        Err(e) => format!("uni_v3_quoter=MISSING (getCode err: {e})"),
    };
    format!("{v3} aero_quoter=MISSING (swap-static MiniQuoter overlay; V4 quoter không dùng A3)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dust_is_one_usdc_6dec() {
        assert_eq!(DUST_USDC, 1_000_000);
    }

    #[test]
    fn gas_formula_20gwei_cap400k_native18() {
        // 20e9 * 400_000 / 1e18 = 0.008
        assert!((gas_est_usdc() - 0.008).abs() < 1e-12);
        let line = gas_formula_line();
        assert!(line.contains("20"));
        assert!(line.contains("400000"));
        assert!(line.contains("native_usdc_18dec"));
    }

    #[test]
    fn spread_bps_3pct() {
        let a = U256::from(100u64);
        let b = U256::from(103u64);
        let s = spread_bps(a, b).unwrap();
        assert!((s - 300.0).abs() < 1e-9);
    }

    #[test]
    fn no_quote_line_has_no_profit_word() {
        let n = NoQuote {
            token: USDC,
            symbol: "X".into(),
            size_usdc: 100,
            quotes: 1,
            detail: "aero_cl fail".into(),
        };
        let s = format_no_quote_line(&n);
        assert!(s.contains("no_quote"));
        assert!(!s.to_ascii_lowercase().contains("profit"));
    }

    #[test]
    fn paper_sizes_100_1000_5000() {
        assert_eq!(PAPER_SIZES_USDC, [100, 1_000, 5_000]);
    }

    #[test]
    fn mini_quoter_hex_present() {
        let c = MINI_QUOTER_CODE.trim();
        assert!(c.starts_with("0x60"));
        assert_eq!((c.len() - 2) / 2, 886);
    }
}
