# BAOCAO02 — cụm A1 (swap logs + RPC list + ignore key + VPS clean)

Ngày: 2026-09-17. Thợ = Grok Code. Không live. Không sendRaw. Không ArbExecutor.

## 1. Lát A1

Decoder Swap Uni V2 + V3 + V4 PoolManager (topic chuẩn Uni, alloy `sol-types`). Pairbook đọc `pairs_arb.txt` header-only không panic. `--watch-once`: 1 head HTTP latest → `eth_getLogs` 1 block trên venue đã pin. 0 send. RPC list đo thật chainId `0x13b2`. `key/` gitignore. VPS NJ chỉ dọn bot cũ.

Path: `/home/dmin/backrun-arc`. Remote `https://github.com/0xcoldtrace/backrun-arc.git`.

## 2. File đụng

- `.gitignore` `.env.example` `config.toml` `pairs_arb.txt` `Cargo.toml` `Cargo.lock`
- `src/lib.rs` `src/rpc.rs` `src/main.rs` `src/logs.rs` `src/pairbook.rs` `src/watch.rs`
- `docs/STATE.md` `docs/TASKS.md` `docs/RUN.md` `docs/DEX_REGISTRY.md` `docs/DOC_MAP.md`
- `addresses/rpc_list.md`
- `baocao/BAOCAO02.md`
- Không add `.env`. Không add `key/`. Không sửa `$ALL`. Không `ArbExecutor.sol`. Không sim_arb. Không live/sendRaw/sandwich/pending.

## 3. git HEAD

Commit message: `A1: swap log decoder + rpc list + ignore key + vps clean`.
SHA đầy đủ in kèm chat Thợ sau push.

## 4. Số swap / block

`--watch-once` 1 block `0x1452318`:

- logs=6 swaps_decoded=6 send=0
- family v2=0 v3=4 v4=2
- venue uni_v2=0 ach_v2=0 uni_v3=2 uni_v4=2 unknown=2
- unknown=2: Swap V3 topic decode được, `factory()` không khớp Uni V3 đã pin — không pin thêm.

Nguồn = HTTP latest head + `eth_getLogs` (không pending, không sendRaw).

## 5. rpc_crate

**alloy** (`rpc_crate=alloy`). HTTP JSON-RPC vẫn reqwest blocking + UA. Decode V2/V3/V4 bằng `alloy::sol_types`.

V4 topic `0x40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f`
= `Swap(bytes32,address,int128,int128,uint160,uint128,int24,uint24)`.
Nguồn: https://github.com/Uniswap/v4-core/blob/main/src/interfaces/IPoolManager.sol

## 6. WSS Circle / Pinax / arc-scan.org

- Circle WSS `wss://rpc.mainnet.arc.io` `eth_subscribe newHeads`: **ALIVE** (rtt 720 ms, subscribe id). Docs Circle HTTP-only — đo thật sống.
- Pinax HTTP `https://arc.rpc.pinax.network` chainId **`0x13b2`** (rtt 1344.04 ms).
- arc-scan.org HTTP `https://rpc.arc-scan.org` chainId **`0x13b2`** (rtt 866.83 ms, UA OK).
- Bảng đủ: `addresses/rpc_list.md`. dRPC WSS newHeads DEAD (code 23). Tatum HTTP 404. routeme.sh 429.

## 7. Dual-venue / Aero

- Dual-venue: 20 block Swap logs suy ra **6 token ≥ 2 pool** (V2/V3 `token0`/`token1`). Depth/TVL CHƯA ĐO.
- Aero Lite factory Arc: **MISSING**. Base `0x420DD381b31aEf6683db6B902084cB0FFECe40Da` getCode `0x` trên 5042 (đo lại A1). Không pin.

## 8. cargo test

`cargo test --offline`: 16 test pass (14 unit + 2 integration). Không gửi tx. Không PRIVATE_KEY.

`cargo run --release -- --watch-once`: in số swap decode, `send=0`. Morpho pin `0x34CD04070dD72b14E241112F6d83812Df5Af7fCD` — A1 không gọi `flashLoan`. Aave Hub không dùng.

## 9. VPS

VPS cleaned: `bsc-sandwich`. Host=REDACTED. cpu=8 nproc. mem MemTotal=8139324 kB MemAvailable=7384096 kB. disk `/` 34G used 5.3G avail 29G 16%.
`~/bsc` absent. Không rsync `backrun-arc`. Không đụng `.ssh` / `/etc`.

## 10. key/ ignored

key/ ignored = yes (`git check-ignore -v key/` → `.gitignore:3:key/`). `git ls-files` không có `key/`. Không cat key. Không commit `.pem` / `.env` / SSH key.

## 11. Go/No-Go

**Contract executor = No-Go.**

A1 decoder + rpc list + ignore key + VPS clean — không live.
