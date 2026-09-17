//! Quote đọc (eth_call). Không bịa profit. 0 send.

use crate::logs::{UNI_V2_ROUTER, UNI_V3_FACTORY, UNI_V3_QUOTER, USDC};
use crate::rpc::{RpcClient, RpcError};
use alloy::primitives::{Address, U256};
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
}

const V3_FEES: [u32; 4] = [100, 500, 3000, 10_000];
const DUST_USDC: u64 = 1_000_000; // 1 USDC (6 dec)

#[derive(Debug, Clone)]
pub struct VenueQuote {
    pub venue: String,
    pub amount_out: U256,
}

pub fn quote_v2_usdc_to_token(
    rpc: &RpcClient,
    token: Address,
) -> Result<U256, RpcError> {
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

pub fn quote_v3_usdc_to_token(
    rpc: &RpcClient,
    token: Address,
    fee: u32,
) -> Result<U256, RpcError> {
    let params = QuoteExactInputSingleParams {
        tokenIn: USDC,
        tokenOut: token,
        amountIn: U256::from(DUST_USDC),
        fee: alloy::primitives::aliases::U24::try_from(fee).map_err(|e| RpcError::Rpc(e.to_string()))?,
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

pub fn uni_v3_get_pool(rpc: &RpcClient, token: Address, fee: u32) -> Option<Address> {
    let fee_u = alloy::primitives::aliases::U24::try_from(fee).ok()?;
    let data = IUniV3Factory::getPoolCall {
        tokenA: USDC,
        tokenB: token,
        fee: fee_u,
    }
    .abi_encode();
    let hex = rpc
        .eth_call(
            &format!("{UNI_V3_FACTORY:#x}"),
            &format!("0x{}", alloy::primitives::hex::encode(&data)),
        )
        .ok()?;
    crate::logs::address_from_word_hex(&hex).filter(|a| !a.is_zero())
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
        for fee in V3_FEES {
            if uni_v3_get_pool(rpc, token, fee).is_none() {
                continue;
            }
            if let Ok(a) = quote_v3_usdc_to_token(rpc, token, fee) {
                if !a.is_zero() {
                    out.push(VenueQuote {
                        venue: format!("uni_v3:{fee}"),
                        amount_out: a,
                    });
                    break;
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::DUST_USDC;
    #[test]
    fn dust_is_one_usdc_6dec() {
        assert_eq!(DUST_USDC, 1_000_000);
    }
}
