# INVENTORY — `$ALL=/home/dmin/all` (chỉ đọc, 2026-09-17)

Quét đúng 14 folder. Không copy file sang `$ARC`. Module: tax_vet pairbook sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws.

Tổng: **14/14 folder đọc được**. **11/14 có Cargo.toml** (jay, pancakeswap, trading = không).

| path | Cargo.toml? | README 1 câu | chain | trạng thái | module | 3 file then chốt |
|---|---|---|---|---|---|---|
| `/home/dmin/all/bot-trade` | có | Session bot Base (AMM) | Base/8453 | dang_do | tax_vet sim_v3 sender | `src/main.rs` `src/lib.rs` `config.toml` |
| `/home/dmin/all/bsc` | có (workspace `bot/`) | không README; CLAUDE: sự thật nằm ở mã | BSC/56 | dang_do | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws | `bot/crates/core/src/lib.rs` `bot/crates/exec/src/main.rs` `contracts/src/FlashArb.sol` |
| `/home/dmin/all/cross-dex` | có (15 crate) | Cross-DEX Arbitrage Bot on Base | BSC/56, Base/8453 | complete | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws | `crates/simulator/src/lib.rs` `crates/flashloan/src/lib.rs` `crates/dex` (workspace) |
| `/home/dmin/all/cross-dex-live` | có (15 crate) | Cross-DEX Arbitrage Bot on Base (bản live) | BSC/56, Base/8453 | complete | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws | `bin/bot/src/main.rs` `crates/simulator/src/lib.rs` `PROGRESS.md` |
| `/home/dmin/all/jay` | không | không README; MASTER_SPEC + Python/solidity arb | ? (BSC dấu vết) | dang_do | tax_vet sim_v3 flash dashboard sender decoder pending_ws | `MASTER_SPEC.md` `CLAUDE.md` `solidity/` |
| `/home/dmin/all/liquidation` | có (`bot/`) | liquidation — Venus BSC liquidation bot | BSC/56 | complete | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder | `bot/liquidation-bot/src/main.rs` `docs/STATUS.md` `README.md` |
| `/home/dmin/all/mexc-base` | có (11 crate) | Bot Arbitrage CEX-DEX (MEXC ↔ Base) | BSC/56, Base/8453 | complete | tax_vet sim_v3 sim_v4 flash dashboard sender decoder | `crates/engine/src/lib.rs` `crates/shadow/src/lib.rs` `crates/dex/src/lib.rs` |
| `/home/dmin/all/pancakeswap` | không | (thư mục trống) | ? | dang_do | — | không file |
| `/home/dmin/all/robinhood` | có | bot arbitrage trên Robinhood Chain | Robinhood + dấu BSC/Base | dang_do | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws | `docs/STATE.md` `crates/rh-core/src/lib.rs` `contracts/test/V4Flash.t.sol` |
| `/home/dmin/all/scan-pancake-v2-token-pool` | có | scan_pancake_v2 | BSC/56 (Pancake V2) | complete | tax_vet pairbook sim_v2 decoder | `src/main.rs` `src/lib.rs` `Cargo.toml` |
| `/home/dmin/all/thanhcong-bsc` | có (`bot/`) | thanhcong-bsc — arb BSC Pancake/Uni/Thena flash Moolah | BSC/56 | tung_live | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws | `bot/bot-dex/src/lib.rs` `docs/STATE.md` `contracts/src/FlashProbe.sol` |
| `/home/dmin/all/trading` | không (TypeScript) | Session bot Base (AMM) — LIVE tiền thật, TypeScript + viem | Base/8453 | tung_live | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws | `config.toml` `docs/STATE.md` `src/dashboard_alive.ts` |
| `/home/dmin/all/vet-bsc-token` | có | vet_bsc_token — vet token BSC trước pairs.txt, chỉ đọc chain | BSC/56, Base/8453 | complete | tax_vet pairbook sim_v2 sim_v4 sender decoder | `src/vet.rs` `src/lib.rs` `src/main.rs` |
| `/home/dmin/all/VPS-22-08-2026` | có (snapshot `bsc-bot`) | không README; snapshot VPS 2026-08-22 của bot BSC | BSC/56 | dang_do | tax_vet sim_v2 sim_v3 sim_v4 flash dashboard sender decoder pending_ws | `bsc-vps-snapshot-20260822/bsc-bot/bot/crates/core/src/lib.rs` `.../crates/exec/src/main.rs` `.../contracts/src/FlashArb.sol` |

## Tham chiếu thêm (không nằm trong 14)

| path | ghi chú |
|---|---|
| `/home/dmin/bsc-sandwich` | Có. Kế hoạch B backrun-arb, sandwich TẮT, B1 contract No-Go. `strategy="backrun"`, dry_run mặc định, cap borrow, PairBook, measure tax. **Bắt chước luật đo, không fork nguyên repo.** |
| `/home/dmin/all/bsc-sandwich` | Không tồn tại. |
| https://github.com/0xcoldtrace/bsc-sandwich | Remote của kho trên. |

Không copy file từ các path trên sang `$ARC` ở cụm này.
