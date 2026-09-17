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

Decoder Swap logs: A1 xong (alloy `sol-types`, topic chuẩn Uni V2/V3/V4). Router/Quoter chưa pin.

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

## Chưa pin

| Tên | Lý do |
|---|---|
| Aero Lite factory | Announced live (https://aero.xyz/articles/aero-lite-is-live-on-arc/ 2026-09-16). Factory Arc chưa thấy explorer/logs. Base PoolFactory `0x420DD381b31aEf6683db6B902084cB0FFECe40Da` getCode `0x` trên 5042 — không pin. |
| Uni V2/V3/V4 router, quoter, PositionManager | Ứng viên docs có; A0 chỉ pin factory/PM bắt buộc. |
| Dual-venue pool count | 20 block Swap logs: 6 token ≥2 pool. Depth CHƯA ĐO. |
