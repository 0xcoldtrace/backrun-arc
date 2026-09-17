# BAOCAO03 — cụm A2 (pairbook vet tax + aero verify + VPS latency)

Ngày: 2026-09-17. Thợ = Grok Code. Không live. Không sendRaw. Không ArbExecutor.

## 1. Lát A2

Pin UR / Uni V2 Router / SwapRouter02 / UniswapX reactor / Aero CLFactory khi getCode != 0x. Quét V2 allPairs + V3 getPool + Aero allPools; vet tax eth_call dust buy/sell; ghi `pairs_arb.txt` / `pairs_arb.rejected.txt`. `--watch-once` in candidate hoặc `no_quote`, không bịa profit, send=0. VPS NJ đo TTFB n=5.

Path: `/home/dmin/backrun-arc`. Remote `https://github.com/0xcoldtrace/backrun-arc.git`.

## 2. File đụng

- `src/discover.rs` `src/vet.rs` `src/quote.rs` `src/bytecode/PairTaxProbe.hex` `src/bytecode/ProbeWallet.hex`
- `src/lib.rs` `src/main.rs` `src/rpc.rs` `src/logs.rs` `src/pairbook.rs` `src/watch.rs`
- `pairs_arb.txt` `pairs_arb.rejected.txt` `config.toml`
- `docs/DEX_REGISTRY.md` `docs/STATE.md` `docs/TASKS.md` `docs/RUN.md` `docs/DOC_MAP.md`
- `addresses/vps_latency.csv`
- `baocao/BAOCAO03.md` `baocao/evidence/discover_local.log` `baocao/evidence/watch_once_local.log` `baocao/evidence/watch_once_vps.log`
- Không add `.env`. Không add `key/`. Không sửa `$ALL`. Không `ArbExecutor.sol`. Không live/sendRaw/sandwich/pending. Không Morpho.flashLoan.

## 3. git HEAD

Commit message: `A2: pairbook vet tax + aero verify + vps latency`.
SHA đầy đủ in kèm chat Thợ sau push.

## 4. Token PASS / FAIL

- PASS **2**
  - `EURC` `0xbef5f6d51cb62b58e6a8f77868681825c6fe21c1` venues=aero_cl+uni_v3 depth_usd=13804.20 tax_buy_bps=0 tax_sell_bps=0
  - `cirBTC` `0x171a4217b86a807a64eb94757db6849fb4bdbaa0` venues=aero_cl+uni_v3 depth_usd=371421.86 tax_buy_bps=0 tax_sell_bps=0
- FAIL **452** (`pairs_arb.rejected.txt`) — chủ yếu `lt_2_venues_deep` (một chân USDC hoặc depth < 8000). 0 honeypot/tax>1% trên tập đã đủ 2 chân sâu.

Công thức: `depth_usd = reserve_usdc * 2 / 1e6` (USDC ERC-20 6 dec). min_depth_usd=8000.

Scan: Uni V2 allPairsLength=485 (447 cặp USDC); AchSwap V2 allPairsLength=5; Aero allPoolsLength=7; V3 PoolCreated ~3895 log; V4 Initialize đọc được (~46k) nhưng depth unread → không tính chân. Kyber/1inch/LI.FI không vào venue. Không copy BSC.

## 5. Aero full addr / UR getCode

- Aero CLFactory **PINNED** `0xb89df768af2cfe637ceb352c587fe8edaf491d03` (khớp truncated `0xb89d…d03`). getCode **9492** bytes. `allPoolsLength=7`. `allPools(0)=0xbe080ac37ad1305dfcc9521f5e6f68cfdc41b7fa`. Base `0x420D…40Da` getCode `0x`.
- Số pool thật: **7**. Chân USDC depth≥8000: **3** (EURC, cirBTC, WETH). Claim “7 / 5 có liq” — đo lại không khớp 5; ghi 7 / 3 USDC-sâu.
- UR `0x4fca4a51ab4f23a7447b3284fbd7d73289a89fb1` getCode **24546** bytes (PINNED).
- Uni V2 Router 21902; SwapRouter02 24497; UniswapX reactor 17206 (không phải AMM venue).

## 6. VPS TTFB (host=REDACTED, region=NJ, n=5, UA, eth_chainId)

| rpc | ttfb_ms_avg | chain_id |
|---|---|---|
| rpc.mainnet.arc.io | 51.66 | 0x13b2 |
| rpc.quicknode.mainnet.arc.io | 51.22 | 0x13b2 |
| arc-rpc.publicnode.com | 53.91 | 0x13b2 |
| rpc.drpc.mainnet.arc.io | 54.79 | 0x13b2 |
| arc.rpc.pinax.network | 132.20 | 0x13b2 |
| rpc.blockdaemon.mainnet.arc.io | 229.59 | 0x13b2 |

Bảng đủ: `addresses/vps_latency.csv`.

## 7. watch-once VPS

`baocao/evidence/watch_once_vps.log`: source=http_latest_head, **send=0**, pairbook entries=2, aero PINNED, `no_quote` (Aero không có quoter pin — 1 quote V3, không bịa profit), `profit=not_computed`. Không systemd live. Không PRIVATE_KEY trên VPS. Không copy `key/` `.env`.

## 8. cargo test

`cargo test --offline`: 26 test pass (24 unit + 2 integration). Không gửi tx. Không PRIVATE_KEY.

## 9. key/ ignored

key/ ignored = yes (`git check-ignore -v key/` → `.gitignore:3:key/`). `git ls-files` không có `key/`. Không cat key. Không commit `.pem` / `.env` / SSH key.

## 10. Go/No-Go

**Contract executor = No-Go.**

A2 pairbook vet + aero verify + VPS latency — không live.
