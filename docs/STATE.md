# STATE — A2 (2026-09-17)

Repo: `/home/dmin/backrun-arc`. Remote: `https://github.com/0xcoldtrace/backrun-arc.git`. Nhánh `master`.

## Chain đo thật

- `eth_chainId` = `0x13b2` (5042). KHÔNG testnet 5042002. KHÔNG chain 1243.
- Quote = USDC ERC-20 `0x3600000000000000000000000000000000000000` (6 dec). Gas native 18 dec.
- `pending_enabled=false`. strategy=backrun. dry_run=true allow_live=false bot_armed=false.
- Contract executor = No-Go. Morpho.flashLoan không gọi.

## Pin A2 (getCode != 0x)

Xem `docs/DEX_REGISTRY.md`.

- Universal Router `0x4fca4a51ab4f23a7447b3284fbd7d73289a89fb1` 24546 bytes.
- Uni V2 Router `0x1f7d7550B1b028f7571E69A784071F0205FD2EfA` 21902 bytes.
- SwapRouter02 `0x53BF6B0684Ec7eF91e1387Da3D1a1769bC5A6F77` 24497 bytes.
- UniswapX Reactor `0x0000000015134054eA82AE0bb9fda66b36402C36` 17206 bytes (không phải venue AMM).
- QuoterV2 `0x7DfD4F31be6814D2906BDE155c3e1B146EAc1468` 8273 bytes.
- Aero CLFactory `0xb89df768af2cfe637ceb352c587fe8edaf491d03` 9492 bytes. `allPoolsLength=7`. `allPools(0)=0xbe080ac37ad1305dfcc9521f5e6f68cfdc41b7fa`. Base `0x420D…40Da` getCode `0x`.

## Pairbook (A2 vet, không copy BSC)

Công thức depth: `depth_usd = reserve_usdc * 2 / 1e6` (USDC ERC-20 6 dec). `min_depth_usd=8000`.

Scan: Uni V2 `allPairsLength=485` (447 cặp USDC); AchSwap V2 `allPairsLength=5`; Aero 7 pool; V3 PoolCreated ~3895 log / ~100k block; V4 Initialize đọc được (~46k log) nhưng depth unread → không tính chân.

PASS **2** / FAIL **452**:

- `EURC` `0xbef5f6d51cb62b58e6a8f77868681825c6fe21c1` venues=aero_cl+uni_v3 depth_usd=13804.20 tax 0/0
- `cirBTC` `0x171a4217b86a807a64eb94757db6849fb4bdbaa0` venues=aero_cl+uni_v3 depth_usd=371421.86 tax 0/0

Tax: eth_call stateOverride overlay pair+probe, dust transfer pair→probe→pair. Cửa 100 bps.

## Watch

`--watch-once`: HTTP latest head, `send=0`. Candidate in nếu ≥2 venue quote được (V2 getAmountsOut / V3 QuoterV2). Aero không có quoter pin → `no_quote` (không bịa profit).

## VPS NJ

`addresses/vps_latency.csv` host=REDACTED region=NJ. TTFB eth_chainId n=5 UA. Circle/QuickNode/PublicNode/dRPC ~50–55 ms; Pinax ~132 ms; Blockdaemon ~230 ms. chainId `0x13b2`. Không systemd live. Không PRIVATE_KEY.
