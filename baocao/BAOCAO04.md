# BAOCAO04 — cụm A3 (paper quote 2 chân EURC/cirBTC)

Ngày: 2026-09-17. Thợ = Grok Code. Không live. Không sendRaw. Không ArbExecutor.

## 1. Lát A3

Pin Uni V3 QuoterV2 khi getCode != 0x. Aero Quoter **MISSING** → quote aero_cl bằng eth_call swap-static MiniQuoter overlay (pool.swap, callback revert amounts). Paper 2 token PASS, size 100/1000/5000 USDC 6 dec, 2 chân aero_cl vs uni_v3. `--paper-seconds 30` trên VPS, jsonl send=0. Không V4 quote (depth unread). Không PRIVATE_KEY.

Path: `/home/dmin/backrun-arc`. Remote `https://github.com/0xcoldtrace/backrun-arc.git`.

## 2. File đụng

- `src/quote.rs` `src/paper.rs` `src/main.rs` `src/lib.rs` `src/logs.rs` `src/bytecode/MiniQuoter.hex`
- `docs/DEX_REGISTRY.md` `docs/STATE.md` `docs/TASKS.md` `docs/RUN.md` `docs/DOC_MAP.md`
- `config.toml` comment A3
- `baocao/BAOCAO04.md` `baocao/evidence/paper_vps.jsonl` `baocao/evidence/paper_vps.log` `baocao/evidence/quote_local.log`
- Không add `.env`. Không add `key/`. Không sửa `$ALL`. Không `ArbExecutor.sol`. Không live/sendRaw/sandwich/pending. Không Morpho.flashLoan.

## 3. git HEAD

Commit message: `A3: paper quote EURC cirBTC aero vs v3`.
SHA đầy đủ in kèm chat Thợ sau push.

## 4. Bảng quote 3 size × 2 token (VPS, send=0)

Gas: `gas_est_wei = 20e9 * 400000`; native USDC 18 dec; `gas_est_usdc = gas_est_wei / 1e18 = 0.008`.
`after_gas_est` = round-trip USDC (mua chân rẻ, bán chân đắt) − size − 0.008. `min_profit_usdc=2` → `profitable_yes_no`.

| token | size_usdc | price_a (aero_cl) | price_b (uni_v3) | spread_bps | after_gas_est | profitable_yes_no |
|---|---|---|---|---|---|---|
| cirBTC `0x171a4217b86a807a64eb94757db6849fb4bdbaa0` | 100 | 130590 | 130660 (`uni_v3:100`) | 5.3603 | -0.037857 | no |
| cirBTC | 1000 | 1305738 | 1306565 (`uni_v3:100`) | 6.3336 | -0.452732 | no |
| cirBTC | 5000 | 6525073 | 6531891 (`uni_v3:100`) | 10.4489 | -5.713892 | no |
| EURC `0xbef5f6d51cb62b58e6a8f77868681825c6fe21c1` | 100 | 87134965 | 87102688 (`uni_v3:500`) | 3.7056 | -0.071444 | no |
| EURC | 1000 | 871333552 | 870924677 (`uni_v3:500`) | 4.6947 | -0.778143 | no |
| EURC | 5000 | 4356309877 | 1820860599 (`uni_v3:500`) | 13924.4557 | -6.978498 | no |

EURC 5000 `spread_bps` lớn vì uni_v3 fee 500 mỏng (USDC reserve ~6380) — impact one-way, **không** phải round-trip dương. `after_gas_est` cả 6 dòng âm; không chữ profit khi 1 chân fail (0 fail).

## 5. Quoter

- Uni V3 QuoterV2 **PINNED** `0x7DfD4F31be6814D2906BDE155c3e1B146EAc1468` getCode **8273** bytes (Uniswap 5042.json / AchSwap docs).
- Aero Quoter **MISSING**. SwapRouter đo `0xb4702e1375f712da2e0d5f534c30c0c1513edb2b` (10060 bytes, `factory()`=CLFactory) — không phải quoter.
- V4 Quoter có getCode; A3 không quote V4.

## 6. VPS paper 30s

`baocao/evidence/paper_vps.jsonl`: **112** dòng (56 tick × 2 token), **send=0** mọi dòng, `ticks_spread_gt0=56`. PRIVATE_KEY absent. Không copy `key/` `.env`. Build trên VPS (glibc 2.35). Không systemd live.

## 7. WETH Aero

`0x128cc466b61f542da60c70e3aa11c10e19b84edb` proxy getCode **735** / impl **21953**; symbol WETH; **18 dec**; **không** wrap native USDC (impl không có `deposit`/`withdraw`). Uni V3 USDC depth ~0 → không thêm pairbook.

## 8. cargo test

`cargo test --offline`: 35 test pass (32 unit lib + 1 bin + 2 integration). Không gửi tx. Không PRIVATE_KEY.

## 9. key/ ignored

key/ ignored = yes. Không cat key. Không commit `.pem` / `.env` / SSH key.

## 10. Go/No-Go

**Contract executor = No-Go.**

A3 paper quote EURC/cirBTC aero vs v3 — không live.
