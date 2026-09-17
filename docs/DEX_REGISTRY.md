# DEX_REGISTRY.md — Arc mainnet (chain 5042)

Pin **chỉ** khi `eth_getCode != 0x` trên RPC `https://rpc.mainnet.arc.io` + nguồn URL + ngày.
Probe: `addresses/rpc_probe.txt` + `addresses/arc.discovered.json` (2026-09-17).
`eth_chainId` = `0x13b2` trước mọi getCode.

## Core / token

| Contract | Address | source_url | pinned_date | getCode bytes | Trạng thái |
|---|---|---|---|---|---|
| USDC ERC-20 (6 dec) | `0x3600000000000000000000000000000000000000` | https://docs.arc.io/arc/references/contract-addresses | 2026-09-17 | 1798 | PINNED — landmine decimals; native gas 18 dec |
| EURC | `0xbEf5f6d51CB62b58e6A8f77868681825C6fe21c1` | https://docs.arc.io/arc/references/contract-addresses | 2026-09-17 | 1798 | PINNED |
| Permit2 | `0x000000000022D473030F116dDEE9F6B43aC78BA3` | https://docs.arc.io/arc/references/contract-addresses | 2026-09-17 | 9152 | PINNED |

## Uniswap

| Contract | Address | source_url | pinned_date | getCode bytes | Trạng thái |
|---|---|---|---|---|---|
| V4 PoolManager | `0x8366a39CC670B4001A1121B8F6A443A643e40951` | https://docs.achswap.app/technical/contract-addresses ; https://docs.peach.ag/documentation/arc/launchpad/contract-address | 2026-09-17 | 24009 | PINNED |
| V3 factory | `0xf0db7b58379503491d857dB50AC9ece64c653918` | https://docs.achswap.app/technical/contract-addresses | 2026-09-17 | 24535 | PINNED |
| V2 factory | `0x89e5DB8B5aA49aA85AC63f691524311AEB649eba` | https://docs.achswap.app/technical/contract-addresses | 2026-09-17 | 13859 | PINNED |
| V2 Router02 | `0x1f7d7550B1b028f7571E69A784071F0205FD2EfA` | https://github.com/Uniswap/contracts/blob/main/deployments/json/5042.json ; https://docs.achswap.app/technical/contract-addresses | 2026-09-17 | 21902 | PINNED |
| SwapRouter02 | `0x53BF6B0684Ec7eF91e1387Da3D1a1769bC5A6F77` | https://github.com/Uniswap/contracts/blob/main/deployments/json/5042.json ; https://docs.achswap.app/technical/contract-addresses | 2026-09-17 | 24497 | PINNED |
| Universal Router | `0x4fca4a51ab4f23a7447b3284fbd7d73289a89fb1` | https://github.com/Uniswap/contracts/blob/main/deployments/json/5042.json ; https://docs.achswap.app/technical/contract-addresses (V4 Router dependency) | 2026-09-17 | 24546 | PINNED |
| QuoterV2 | `0x7DfD4F31be6814D2906BDE155c3e1B146EAc1468` | https://github.com/Uniswap/contracts/blob/main/deployments/json/5042.json | 2026-09-17 | 8273 | PINNED — quote V3, không phải venue pairbook |
| UniswapX DutchV3 Reactor | `0x0000000015134054eA82AE0bb9fda66b36402C36` | https://developers.uniswap.org/docs/liquidity/uniswapx/deployments ; https://github.com/Uniswap/UniswapX/blob/main/playbook/chains/arc.md | 2026-09-17 | 17206 | PINNED — filler/reactor, **không** AMM venue pairbook |

Decoder Swap logs: A1 xong (alloy `sol-types`). Router/UR/Quoter/UniswapX pin A2 khi getCode != 0x.

## AchSwap

| Contract | Address | source_url | pinned_date | getCode bytes | Trạng thái |
|---|---|---|---|---|---|
| V2 factory | `0xb0C2B0acb9c13079dDd871eDaF43Aabf6e88C530` | https://docs.achswap.app/technical/contract-addresses | 2026-09-17 | 13859 | PINNED |

## Flash (không phải DEX)

| Contract | Address | source_url | pinned_date | getCode bytes | Trạng thái |
|---|---|---|---|---|---|
| Morpho Blue | `0x34CD04070dD72b14E241112F6d83812Df5Af7fCD` | https://docs.morpho.org/get-started/resources/addresses/ (tab Arc) | 2026-09-17 | 15582 | PINNED — bytecode có `flashLoan(address,uint256,bytes)` `0xe0232b42`. `0xBBBB…FFCb` getCode `0x` trên 5042, không copy. IERC3156 `flashFee` revert. |
| Morpho Adaptive Curve IRM | `0xF02615d094Fc02fC031C35fe705e175aA4653f20` | https://docs.morpho.org/get-started/resources/addresses/ (tab Arc) | 2026-09-17 | 2282 | PINNED (IRM, không flash) |
| Aave V4 Core Hub | `0x17288dfc86205301064577b98B02b81017e6F79C` | https://github.com/bgd-labs/aave-address-book/blob/main/src/ts/AaveV4Arc.ts | 2026-09-17 | 1419 | PINNED fallback — flash CÓ phí. Main Spoke `0xB843bdC3a87A05E77E07Df9FE48928b3A34b134d` 1419 bytes. |

## Aero Lite (Slipstream CL)

| Contract | Address | source_url | pinned_date | getCode bytes | Trạng thái |
|---|---|---|---|---|---|
| CLFactory | `0xb89df768af2cfe637ceb352c587fe8edaf491d03` | on-chain `factory()` từ pool aero-arc + truncated `0xb89d…d03`; https://aero.xyz/articles/aero-lite-is-live-on-arc/ ; https://api.geckoterminal.com/api/v2/networks/arc/dexes/aero-arc/pools | 2026-09-17 | 9492 | PINNED. `allPoolsLength()=7`. `allPools(0)=0xbe080ac37ad1305dfcc9521f5e6f68cfdc41b7fa` (EURC/USDC). Base factory `0x420D…40Da` getCode `0x` trên 5042 — không pin. |

Đo 2026-09-17: 7 pool. USDC `balanceOf*2/1e6` ≥ 8000: 3 pool (EURC/USDC, cirBTC/USDC, WETH/USDC). Claim “7 / 5 có liq” — **số thật** `allPoolsLength=7`; chân USDC sâu = 3 (WETH/cirBTC không USDC; 2 pool USDC depth 0 hoặc 101).

## Aggregator (không đưa vào venue pairbook)

Kyber / 1inch / LI.FI / UniswapX reactor = aggregator hoặc filler, không phải AMM chân pairbook.

## Chưa pin

| Tên | Lý do |
|---|---|
| Curve / fomo | Không pin (lệnh A2). |
| Uni V3 NPM / V4 PositionManager / V4Quoter | Có getCode; A2 không bắt buộc. |
| Dual-venue depth | A2 đo: 2 token PASS (`EURC`, `cirBTC`) aero_cl+uni_v3, min_depth ≥ 8000, tax 0/0. |
