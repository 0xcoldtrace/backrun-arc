# REUSE — luật lấy từ `$ALL` + `bsc-sandwich` (không copy file)

## COPY-LOGIC (bắt chước luật/ý, viết lại trên 5042)

- **tax/vet**: cửa tax 2 chiều, honeypot, owner/renounce — `vet-bsc-token` (`src/vet.rs`), `scan-pancake-v2-token-pool`, `cross-dex` watchlist. Đổi quote WBNB→USDC 6 dec, bỏ GoPlus-BSC nếu có.
- **pairbook**: file candidate Chủ vet (`pairs.txt` / `pairs_arb.txt`), hot-reload `config_reload_sec`, dòng lỗi bỏ — `bsc-sandwich` + `vet-bsc-token`.
- **sim đa venue**: V2 getAmountOut/reserves, V3 slot0, V4 PoolManager — `cross-dex`/`cross-dex-live` simulator, `thanhcong-bsc` bot-dex, `bsc` math crate. Phải đổi fee/tick/hooks theo registry Arc, không hardcode Pancake 56.
- **dashboard**: axum/web đọc state, không ký — `bsc` dashboard crate, `mexc-base` crates/dashboard, `trading` dashboard.ts (TS, chỉ luật UI).
- **dry_run / armed / shadow**: 3 cờ cùng lúc, lẫn = refuse; shadow log không gửi — `bsc-sandwich` config, `mexc-base` crates/shadow, `thanhcong-bsc` dryrun.
- **cap borrow**: `arb_max_borrow_*` kẹp size vay flash — `bsc-sandwich` `arb_max_borrow_bnb/usdt` → field Arc `arb_max_borrow_usdc`.

## CẤM-COPY

- Pancake/WBNB decoder (router 56, WBNB `0xbb4C…`, path `swapExactETHForTokens`).
- Pending WS: `newPendingTransactions` / `eth_subscribe pending` / `newPendingTransactionFilter` (Arc: filter `-32601`, pending block `-32014`).
- 48club / bloxroute / private tx BSC / puissant-builder.
- Sandwich front (đứng trước victim, 2 tx front+back).
- Address chain 56 / 8453 / 4663 (Pancake factory, WBNB, Base Aero `0x420D…`, Morpho Ethereum `0xBBBB` khi getCode 5042 = `0x`).
- File nguyên từ `$ALL` (cụm này chỉ kê).
- `ArbExecutor.sol` / FlashArb.sol fork.

## ƯU TIÊN ĐỌC

1. `cross-dex` — sim đa venue + flashloan crate + scanner logs.
2. `cross-dex-live` — cùng kiến trúc, vận hành live (học fail-closed, không học chain).
3. `vet-bsc-token` — tax/vet chỉ đọc, format pairs.
4. `bsc` — math offline + dashboard + flash (đổi token/gas).
5. `thanhcong-bsc` — graph V3/V4/CLAMM, dry-run override, WS newHeads (không pending).
6. `liquidation` — flash 0/ít phí, fail-closed docs.

`bsc-sandwich` (ngoài `$ALL`): luật đo (getCode trước pin, dry_run mặc định, strategy=backrun, cap borrow, PairBook, 1 cụm 1 BAOCAO). Sandwich path TẮT. Contract B1 No-Go.
