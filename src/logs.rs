//! Decode Swap logs V2 / V3 / V4 PoolManager (topic chuẩn Uniswap).
//! Không decode pending. V4 topic lấy từ Uniswap v4-core IPoolManager.sol.

use alloy::primitives::{Address, B256, I256, U160, U256};
use alloy::sol_types::SolEvent;
use serde_json::Value;

/// Uni V2 factory Arc — pin A0, getCode != 0x.
pub const UNI_V2_FACTORY: Address = alloy::primitives::address!("0x89e5DB8B5aA49aA85AC63f691524311AEB649eba");
/// Uni V3 factory Arc.
pub const UNI_V3_FACTORY: Address = alloy::primitives::address!("0xf0db7b58379503491d857dB50AC9ece64c653918");
/// Uni V4 PoolManager Arc.
pub const UNI_V4_POOL_MANAGER: Address =
    alloy::primitives::address!("0x8366a39CC670B4001A1121B8F6A443A643e40951");
/// AchSwap V2 factory Arc.
pub const ACHSWAP_V2_FACTORY: Address =
    alloy::primitives::address!("0xb0C2B0acb9c13079dDd871eDaF43Aabf6e88C530");
/// Morpho Blue Arc — PIN, A1/A2 không gọi flashLoan.
pub const MORPHO_BLUE: Address = alloy::primitives::address!("0x34CD04070dD72b14E241112F6d83812Df5Af7fCD");
/// USDC ERC-20 (6 dec) — quote. Gas native 18 dec. KHÔNG WETH.
pub const USDC: Address = alloy::primitives::address!("0x3600000000000000000000000000000000000000");
/// Uni V2 Router02 — pin A2, getCode != 0x.
pub const UNI_V2_ROUTER: Address =
    alloy::primitives::address!("0x1f7d7550B1b028f7571E69A784071F0205FD2EfA");
/// SwapRouter02 — pin A2.
pub const SWAP_ROUTER_02: Address =
    alloy::primitives::address!("0x53BF6B0684Ec7eF91e1387Da3D1a1769bC5A6F77");
/// Universal Router — pin A2.
pub const UNIVERSAL_ROUTER: Address =
    alloy::primitives::address!("0x4fca4a51ab4f23a7447b3284fbd7d73289a89fb1");
/// UniswapX DutchV3 reactor — pin A2; không phải AMM venue pairbook.
pub const UNISWAPX_REACTOR: Address =
    alloy::primitives::address!("0x0000000015134054eA82AE0bb9fda66b36402C36");
/// QuoterV2 — pin A2 (quote V3, không venue).
pub const UNI_V3_QUOTER: Address =
    alloy::primitives::address!("0x7DfD4F31be6814D2906BDE155c3e1B146EAc1468");
/// Aero Lite CLFactory Arc — pin A2 (getCode != 0x, allPoolsLength=7).
pub const AERO_CL_FACTORY: Address =
    alloy::primitives::address!("0xb89df768af2cfe637ceb352c587fe8edaf491d03");
/// Aero Slipstream SwapRouter — Swap.sender đo on-chain; factory()=CLFactory. Không phải quoter.
pub const AERO_SWAP_ROUTER: Address =
    alloy::primitives::address!("0xb4702e1375f712da2e0d5f534c30c0c1513edb2b");
/// WETH bridged ERC-20 Arc (18 dec). Không wrap native USDC. Không pairbook A3 (1 venue sâu).
pub const WETH_ARC: Address =
    alloy::primitives::address!("0x128cc466b61f542da60c70e3aa11c10e19b84edb");

pub mod uni_v2 {
    alloy::sol_types::sol! {
        #![sol(alloy_sol_types = alloy::sol_types)]
        #[derive(Debug, PartialEq, Eq)]
        event Swap(
            address indexed sender,
            uint256 amount0In,
            uint256 amount1In,
            uint256 amount0Out,
            uint256 amount1Out,
            address indexed to
        );
    }
}

pub mod uni_v3 {
    alloy::sol_types::sol! {
        #![sol(alloy_sol_types = alloy::sol_types)]
        #[derive(Debug, PartialEq, Eq)]
        event Swap(
            address indexed sender,
            address indexed recipient,
            int256 amount0,
            int256 amount1,
            uint160 sqrtPriceX96,
            uint128 liquidity,
            int24 tick
        );
    }
}

/// V4 Swap — nguồn: https://github.com/Uniswap/v4-core/blob/main/src/interfaces/IPoolManager.sol
/// `event Swap(PoolId indexed id, address indexed sender, int128 amount0, int128 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick, uint24 fee)`
pub mod uni_v4 {
    alloy::sol_types::sol! {
        #![sol(alloy_sol_types = alloy::sol_types)]
        #[derive(Debug, PartialEq, Eq)]
        event Swap(
            bytes32 indexed id,
            address indexed sender,
            int128 amount0,
            int128 amount1,
            uint160 sqrtPriceX96,
            uint128 liquidity,
            int24 tick,
            uint24 fee
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapFamily {
    V2,
    V3,
    V4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Venue {
    UniV2,
    UniV3,
    UniV4,
    AchSwapV2,
    AeroCl,
    Unknown,
}

impl Venue {
    pub fn as_str(self) -> &'static str {
        match self {
            Venue::UniV2 => "uni_v2",
            Venue::UniV3 => "uni_v3",
            Venue::UniV4 => "uni_v4",
            Venue::AchSwapV2 => "achswap_v2",
            Venue::AeroCl => "aero_cl",
            Venue::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodedSwap {
    V2 {
        address: Address,
        sender: Address,
        to: Address,
        amount0_in: U256,
        amount1_in: U256,
        amount0_out: U256,
        amount1_out: U256,
    },
    V3 {
        address: Address,
        sender: Address,
        recipient: Address,
        amount0: I256,
        amount1: I256,
        sqrt_price_x96: U160,
        liquidity: u128,
        tick: i32,
    },
    V4 {
        address: Address,
        pool_id: B256,
        sender: Address,
        amount0: i128,
        amount1: i128,
        sqrt_price_x96: U160,
        liquidity: u128,
        tick: i32,
        fee: u32,
    },
}

impl DecodedSwap {
    pub fn family(&self) -> SwapFamily {
        match self {
            DecodedSwap::V2 { .. } => SwapFamily::V2,
            DecodedSwap::V3 { .. } => SwapFamily::V3,
            DecodedSwap::V4 { .. } => SwapFamily::V4,
        }
    }

    pub fn address(&self) -> Address {
        match self {
            DecodedSwap::V2 { address, .. }
            | DecodedSwap::V3 { address, .. }
            | DecodedSwap::V4 { address, .. } => *address,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawLog {
    pub address: Address,
    pub topics: Vec<B256>,
    pub data: Vec<u8>,
}

pub fn topic_v2() -> B256 {
    uni_v2::Swap::SIGNATURE_HASH
}
pub fn topic_v3() -> B256 {
    uni_v3::Swap::SIGNATURE_HASH
}
pub fn topic_v4() -> B256 {
    uni_v4::Swap::SIGNATURE_HASH
}

pub fn topic_hex(t: B256) -> String {
    format!("{t:#x}")
}

/// Decode 1 log theo topic0 chuẩn Uni. Topic lạ → None (không panic).
pub fn decode_swap(address: Address, topics: &[B256], data: &[u8]) -> Option<DecodedSwap> {
    let t0 = *topics.first()?;
    if t0 == uni_v2::Swap::SIGNATURE_HASH {
        let ev = uni_v2::Swap::decode_raw_log(topics.iter().copied(), data).ok()?;
        return Some(DecodedSwap::V2 {
            address,
            sender: ev.sender,
            to: ev.to,
            amount0_in: ev.amount0In,
            amount1_in: ev.amount1In,
            amount0_out: ev.amount0Out,
            amount1_out: ev.amount1Out,
        });
    }
    if t0 == uni_v3::Swap::SIGNATURE_HASH {
        let ev = uni_v3::Swap::decode_raw_log(topics.iter().copied(), data).ok()?;
        return Some(DecodedSwap::V3 {
            address,
            sender: ev.sender,
            recipient: ev.recipient,
            amount0: ev.amount0,
            amount1: ev.amount1,
            sqrt_price_x96: ev.sqrtPriceX96,
            liquidity: ev.liquidity,
            tick: i32::try_from(ev.tick).ok()?,
        });
    }
    if t0 == uni_v4::Swap::SIGNATURE_HASH {
        let ev = uni_v4::Swap::decode_raw_log(topics.iter().copied(), data).ok()?;
        return Some(DecodedSwap::V4 {
            address,
            pool_id: ev.id,
            sender: ev.sender,
            amount0: ev.amount0,
            amount1: ev.amount1,
            sqrt_price_x96: ev.sqrtPriceX96,
            liquidity: ev.liquidity,
            tick: i32::try_from(ev.tick).ok()?,
            fee: u32::try_from(ev.fee).ok()?,
        });
    }
    None
}

pub fn classify_venue(family: SwapFamily, log_address: Address, factory: Option<Address>) -> Venue {
    match family {
        SwapFamily::V4 => {
            if log_address == UNI_V4_POOL_MANAGER {
                Venue::UniV4
            } else {
                Venue::Unknown
            }
        }
        SwapFamily::V2 => match factory {
            Some(f) if f == UNI_V2_FACTORY => Venue::UniV2,
            Some(f) if f == ACHSWAP_V2_FACTORY => Venue::AchSwapV2,
            _ => Venue::Unknown,
        },
        SwapFamily::V3 => match factory {
            Some(f) if f == UNI_V3_FACTORY => Venue::UniV3,
            Some(f) if f == AERO_CL_FACTORY => Venue::AeroCl,
            _ => Venue::Unknown,
        },
    }
}

pub fn parse_raw_log(v: &Value) -> Option<RawLog> {
    let addr = parse_address(v.get("address")?.as_str()?)?;
    let topics = v
        .get("topics")?
        .as_array()?
        .iter()
        .map(|t| parse_b256(t.as_str()?))
        .collect::<Option<Vec<_>>>()?;
    let data = decode_hex(v.get("data")?.as_str()?)?;
    Some(RawLog {
        address: addr,
        topics,
        data,
    })
}

pub fn parse_address(s: &str) -> Option<Address> {
    s.parse().ok()
}

/// ABI word: 12 bytes zero + 20-byte address, hex without 0x (64 chars).
pub fn abi_word_addr(a: Address) -> String {
    format!("{:0>64}", alloy::primitives::hex::encode(a.as_slice()))
}

pub fn parse_b256(s: &str) -> Option<B256> {
    s.parse().ok()
}

pub fn decode_hex(s: &str) -> Option<Vec<u8>> {
    let s = s.trim().trim_start_matches("0x");
    alloy::primitives::hex::decode(s).ok()
}

/// factory() / token0() / token1() result = 32-byte word, address in low 20.
pub fn address_from_word_hex(s: &str) -> Option<Address> {
    let b = decode_hex(s)?;
    if b.len() < 20 {
        return None;
    }
    Address::try_from(&b[b.len() - 20..]).ok()
}

pub const SEL_FACTORY: &str = "0xc45a0155";
pub const SEL_TOKEN0: &str = "0x0dfe1681";
pub const SEL_TOKEN1: &str = "0xd21220a7";

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Log, address};

    const SENDER: Address = address!("0x1111111111111111111111111111111111111111");
    const TO: Address = address!("0x2222222222222222222222222222222222222222");
    const PAIR: Address = address!("0x3333333333333333333333333333333333333333");

    fn topics_of<E: SolEvent>(ev: &E) -> Vec<B256> {
        ev.encode_topics().into_iter().map(|t| t.0).collect()
    }

    #[test]
    fn topic0_matches_canonical_uni() {
        assert_eq!(
            topic_hex(topic_v2()),
            "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822"
        );
        assert_eq!(
            topic_hex(topic_v3()),
            "0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67"
        );
        assert_eq!(
            topic_hex(topic_v4()),
            "0x40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f"
        );
        assert_eq!(
            uni_v4::Swap::SIGNATURE,
            "Swap(bytes32,address,int128,int128,uint160,uint128,int24,uint24)"
        );
    }

    #[test]
    fn decode_v2_roundtrip() {
        let ev = uni_v2::Swap {
            sender: SENDER,
            amount0In: U256::from(10u64),
            amount1In: U256::from(0u64),
            amount0Out: U256::from(0u64),
            amount1Out: U256::from(9u64),
            to: TO,
        };
        let topics = topics_of(&ev);
        let data = ev.encode_data();
        let d = decode_swap(PAIR, &topics, &data).expect("v2");
        match d {
            DecodedSwap::V2 {
                sender,
                to,
                amount0_in,
                amount1_out,
                ..
            } => {
                assert_eq!(sender, SENDER);
                assert_eq!(to, TO);
                assert_eq!(amount0_in, U256::from(10u64));
                assert_eq!(amount1_out, U256::from(9u64));
            }
            _ => panic!("want v2"),
        }
    }

    #[test]
    fn decode_v3_roundtrip() {
        let ev = uni_v3::Swap {
            sender: SENDER,
            recipient: TO,
            amount0: I256::try_from(5i64).unwrap(),
            amount1: I256::try_from(-7i64).unwrap(),
            sqrtPriceX96: U160::from(1u64),
            liquidity: 2u128,
            tick: alloy::primitives::aliases::I24::try_from(3i32).expect("tick"),
        };
        let topics = topics_of(&ev);
        let data = ev.encode_data();
        let d = decode_swap(PAIR, &topics, &data).expect("v3");
        match d {
            DecodedSwap::V3 {
                sender,
                recipient,
                tick,
                ..
            } => {
                assert_eq!(sender, SENDER);
                assert_eq!(recipient, TO);
                assert_eq!(tick, 3);
            }
            _ => panic!("want v3"),
        }
    }

    #[test]
    fn decode_v4_roundtrip() {
        let pool_id = B256::from([0xab; 32]);
        let ev = uni_v4::Swap {
            id: pool_id,
            sender: SENDER,
            amount0: 11i128,
            amount1: -13i128,
            sqrtPriceX96: U160::from(4u64),
            liquidity: 5u128,
            tick: alloy::primitives::aliases::I24::try_from(-9i32).expect("tick"),
            fee: alloy::primitives::aliases::U24::try_from(3000u32).expect("fee"),
        };
        let topics = topics_of(&ev);
        let data = ev.encode_data();
        let d = decode_swap(UNI_V4_POOL_MANAGER, &topics, &data).expect("v4");
        match d {
            DecodedSwap::V4 {
                pool_id: pid,
                sender,
                tick,
                ..
            } => {
                assert_eq!(pid, pool_id);
                assert_eq!(sender, SENDER);
                assert_eq!(tick, -9);
            }
            _ => panic!("want v4"),
        }
    }

    #[test]
    fn unknown_topic_skipped() {
        let topics = [B256::from([0x11; 32])];
        assert!(decode_swap(PAIR, &topics, &[]).is_none());
    }

    #[test]
    fn classify_pinned_venues() {
        assert_eq!(
            classify_venue(SwapFamily::V2, PAIR, Some(UNI_V2_FACTORY)),
            Venue::UniV2
        );
        assert_eq!(
            classify_venue(SwapFamily::V2, PAIR, Some(ACHSWAP_V2_FACTORY)),
            Venue::AchSwapV2
        );
        assert_eq!(
            classify_venue(SwapFamily::V3, PAIR, Some(UNI_V3_FACTORY)),
            Venue::UniV3
        );
        assert_eq!(
            classify_venue(SwapFamily::V4, UNI_V4_POOL_MANAGER, None),
            Venue::UniV4
        );
        assert_eq!(
            classify_venue(SwapFamily::V2, PAIR, Some(SENDER)),
            Venue::Unknown
        );
        assert_eq!(
            classify_venue(SwapFamily::V3, PAIR, Some(AERO_CL_FACTORY)),
            Venue::AeroCl
        );
        assert_eq!(
            format!("{AERO_SWAP_ROUTER:#x}"),
            "0xb4702e1375f712da2e0d5f534c30c0c1513edb2b"
        );
        assert_eq!(
            format!("{WETH_ARC:#x}"),
            "0x128cc466b61f542da60c70e3aa11c10e19b84edb"
        );
    }

    #[test]
    fn abi_word_addr_pads_12_bytes() {
        let a = address!("0xbe080ac37ad1305dfcc9521f5e6f68cfdc41b7fa");
        let s = abi_word_addr(a);
        assert_eq!(s.len(), 64);
        assert_eq!(&s[0..24], "000000000000000000000000");
        assert_eq!(&s[24..], "be080ac37ad1305dfcc9521f5e6f68cfdc41b7fa");
        // {:0>64x} on Address is NOT a 32-byte ABI word in alloy 2 — do not use it.
        let naive = format!("{a:0>64x}");
        assert_ne!(naive, s, "naive Address hex pad is not ABI word");
    }

    #[test]
    fn parse_log_json() {
        let ev = uni_v2::Swap {
            sender: SENDER,
            amount0In: U256::from(1u64),
            amount1In: U256::from(0u64),
            amount0Out: U256::from(0u64),
            amount1Out: U256::from(1u64),
            to: TO,
        };
        let log = Log::<uni_v2::Swap> {
            address: PAIR,
            data: ev,
        };
        let encoded = uni_v2::Swap::encode_log(&log);
        let topics: Vec<String> = encoded.topics().iter().map(|t| format!("{t:#x}")).collect();
        let v = serde_json::json!({
            "address": format!("{PAIR:#x}"),
            "topics": topics,
            "data": format!("0x{}", alloy::primitives::hex::encode(&encoded.data.data)),
        });
        let raw = parse_raw_log(&v).expect("json");
        assert!(decode_swap(raw.address, &raw.topics, &raw.data).is_some());
    }
}
