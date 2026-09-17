# STATE — A3 (2026-09-17)

Repo: `/home/dmin/backrun-arc`. Remote: `https://github.com/0xcoldtrace/backrun-arc.git`. Nhánh `master`.

## Chain đo thật

- `eth_chainId` = `0x13b2` (5042). KHÔNG testnet 5042002. KHÔNG chain 1243.
- Quote = USDC ERC-20 `0x3600000000000000000000000000000000000000` (6 dec). Gas native 18 dec.
- `pending_enabled=false`. strategy=backrun. dry_run=true allow_live=false bot_armed=false.
- Contract executor = No-Go. Morpho.flashLoan không gọi.

## Pin (getCode != 0x)

Xem `docs/DEX_REGISTRY.md`.

- Universal Router `0x4fca4a51ab4f23a7447b3284fbd7d73289a89fb1` 24546 bytes.
- Uni V2 Router `0x1f7d7550B1b028f7571E69A784071F0205FD2EfA` 21902 bytes.
- SwapRouter02 `0x53BF6B0684Ec7eF91e1387Da3D1a1769Bc5A6F77` 24497 bytes.
- UniswapX Reactor `0x0000000015134054eA82AE0bb9fda66b36402C36` 17206 bytes (không phải venue AMM).
- Uni V3 QuoterV2 `0x7DfD4F31be6814D2906BDE155c3e1B146EAc1468` 8273 bytes. PINNED. (docs Uniswap 5042.json / AchSwap).
- Aero CLFactory `0xb89df768af2cfe637ceb352c587fe8edaf491d03` 9492 bytes. `allPoolsLength=7`. `allPools(0)=0xbe080ac37ad1305dfcc9521f5e6f68cfdc41b7fa`. Base `0x420D…40Da` getCode `0x`.
- Aero SwapRouter `0xb4702e1375f712da2e0d5f534c30c0c1513edb2b` 10060 bytes (`factory()`=CLFactory). **Aero Quoter = MISSING** — quote swap-static MiniQuoter overlay.
- V4 Quoter `0x8DC178efB8111bB0973dD9d722EbeFF267c98F94` có getCode; A3 không quote V4 (depth unread).

## Pairbook (A2 vet, không copy BSC)

Công thức depth: `depth_usd = reserve_usdc * 2 / 1e6` (USDC ERC-20 6 dec). `min_depth_usd=8000`.

PASS **2** / FAIL **452**:

- `EURC` `0xbef5f6d51cb62b58e6a8f77868681825c6fe21c1` venues=aero_cl+uni_v3 depth_usd=13804.20 tax 0/0
- `cirBTC` `0x171a4217b86a807a64eb94757db6849fb4bdbaa0` venues=aero_cl+uni_v3 depth_usd=371421.86 tax 0/0

## Paper quote A3

2 chân: aero_cl vs uni_v3. Size 100 / 1000 / 5000 USDC (6 dec).
`gas_est_wei = 20e9 * 400000`; native USDC 18 dec; `gas_est_usdc = gas_est_wei / 1e18 = 0.008`.
1 chân eth_call fail → `no_quote`, không chữ profit.
`--paper-seconds N` jsonl `baocao/evidence/paper_vps.jsonl` send=0.

## WETH Aero

`0x128cc466b61f542da60c70e3aa11c10e19b84edb` getCode proxy 735 / impl 21953; symbol WETH; 18 dec; **không** wrap native USDC (`deposit`/`withdraw` không có trong impl). Uni V3 USDC depth ~0 → không thêm pairbook.

## Watch

`--watch-once`: HTTP latest head, `send=0`. Candidate in nếu ≥2 venue quote được (V3 QuoterV2 / Aero MiniQuoter overlay).

## VPS NJ

`addresses/vps_latency.csv` host=REDACTED region=NJ. Không systemd live. Không PRIVATE_KEY.
