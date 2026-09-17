# BAOCAO01 — cụm INV-01 + A0

Ngày: 2026-09-17. Thợ = Grok Code. Không live. Không sendRaw. Không ArbExecutor.

## 1. Lát INV-01+A0

Khung repo `arc_arb` trên Arc mainnet 5042: inventory `$ALL` (chỉ đọc, không copy file), probe RPC thật, pin DEX khi getCode != 0x, config refuse-nếu-thiếu, boot load config + `eth_blockNumber` + `baseFee`. `pending_enabled=false`. `strategy=backrun`. Contract executor = No-Go.

Path dùng: `/home/dmin/backrun-arc` (lệnh ghi `ARC=/home/dmin/backrun-arb`; workspace + remote `backrun-arc.git` → làm trong `backrun-arc`).

## 2. File đụng

- `AGENTS.md` `.gitignore` `.env.example` `config.toml` `pairs_arb.txt` `Cargo.toml` `Cargo.lock`
- `src/lib.rs` `src/config.rs` `src/rpc.rs` `src/main.rs`
- `tests/config_load.rs` `tests/fixtures/config.missing.toml`
- `docs/DOC_MAP.md` `docs/STATE.md` `docs/TASKS.md` `docs/RUN.md` `docs/INVENTORY.md` `docs/REUSE.md` `docs/DEX_REGISTRY.md`
- `addresses/rpc_probe.txt` `addresses/arc.discovered.json`
- `scripts/.gitkeep`
- `baocao/BAOCAO01.md`
- Không add `.env`. Không sửa `$ALL`. Không `ArbExecutor.sol`. Không `web/` (nợ).

## 3. git HEAD

Commit message: `A0 INV-01: inventory ALL + Arc probe + khung backrun-arb`. SHA đầy đủ = `git rev-parse HEAD` trên repo sau push (in kèm chat Thợ; 1 commit, amend trước push).

## 4. `$ALL` đọc được / có Cargo.toml

**14 / 14** folder đọc được.

**11 / 14** có `Cargo.toml` (không: `jay`, `pancakeswap`, `trading`).

Extra: `/home/dmin/bsc-sandwich` có (chỉ đọc). `/home/dmin/all/bsc-sandwich` không có.

## 5. Bảng REUSE tóm tắt

| Nhóm | Nội dung |
|---|---|
| COPY-LOGIC | tax/vet, pairbook, sim đa venue, dashboard, dry_run/armed/shadow, cap borrow |
| CẤM-COPY | pancake/WBNB decoder, pending WS, 48club/bloxroute/private tx BSC, sandwich front, address 56/8453/4663, file nguyên `$ALL`, Morpho `0xBBBB` khi code 5042 = `0x` |
| ƯU TIÊN ĐỌC | cross-dex, cross-dex-live, vet-bsc-token, bsc, thanhcong-bsc, liquidation |

Chi tiết: `docs/REUSE.md`.

## 6. Raw chainId / block / pending-error / getCode USDC / V4

RPC `https://rpc.mainnet.arc.io` (UA bắt buộc; urllib không UA = HTTP 403 Cloudflare 1010).

```
eth_chainId          → {"jsonrpc":"2.0","id":1,"result":"0x13b2"}
eth_blockNumber      → {"jsonrpc":"2.0","id":2,"result":"0x1451030"}
eth_getBlockByNumber latest.baseFeePerGas = "0x4a817c800"  (20 gwei)
eth_getCode USDC     → 1798 bytes, prefix 0x60806040…  decimals()=6 symbol()=USDC
eth_getCode V4 PM    → 24009 bytes, prefix 0x60a08060…  0x8366a39CC670B4001A1121B8F6A443A643e40951
pending getBlock     → {"jsonrpc":"2.0","id":40,"error":{"message":"requested data not available","code":-32014}}
newPendingTxFilter   → {"jsonrpc":"2.0","id":41,"error":{"code":-32601,"message":"method not supported"}}
```

(Lệnh ghi kỳ vọng pending `-32001`. Đo thật: `-32014` / `-32601`. Ghi số đo, không sửa thành `-32001`.)

Raw đầy đủ: `addresses/rpc_probe.txt`.

## 7. Morpho

**`0x34CD04070dD72b14E241112F6d83812Df5Af7fCD`**

- Nguồn: https://docs.morpho.org/get-started/resources/addresses/ tab Arc.
- getCode 15582 bytes trên 5042. Bytecode chứa `flashLoan(address,uint256,bytes)` selector `0xe0232b42`.
- Ethereum `0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb` getCode `0x` trên 5042 — không copy.
- IERC3156 `flashFee(address,uint256)` eth_call revert code 3. Morpho Blue không ERC-3156; phí flash = 0 theo spec protocol, chưa gọi `flashLoan` (cấm tx).

Aave V4 Core Hub (fallback, CÓ phí): `0x17288dfc86205301064577b98B02b81017e6F79C` getCode 1419 bytes — verified address-book + getCode, không MISSING.

## 8. So BSC: tái dùng gì / chết gì

Tái dùng: luật đo (getCode trước pin, dry_run mặc định, 3 cờ live, cap borrow, pairbook, tax/vet, sim V2/V3/V4, dashboard, 1 cụm 1 commit 1 BAOCAO). `strategy=backrun` như kế hoạch B `bsc-sandwich` (sandwich TẮT).

Chết trên Arc: pending mempool (`-32014`/`-32601`), builder public (PoA rotate, không 48club/Flashbots), WBNB/Pancake-56 decoder, gas token ≠ BNB (USDC native 18 + ERC-20 6 dec). Sandwich front chết theo lệnh. Contract executor No-Go (B1 sandwich No-Go cùng luật).

## 9. Nợ A1

Pin venue + decoder **Swap logs only** (V2/V3/V4 trên factory/PM đã pin). Nguồn = logs block đã final (`newHeads`). Không decoder pending. Pairbook skeleton. Aero factory Arc vẫn MISSING. Dual-venue count CHƯA ĐO. `web/` stub nợ. Chọn alloy hoặc ethers-rs (đúng 1).

## 10. Go/No-Go A1

- chainId đúng (`0x13b2`): có
- `$ALL` đọc được (14/14): có
- ≥1 DEX getCode != 0x: có (V4 PM 24009, V3 factory 24535, V2 factory 13859, AchSwap V2 13859)

**A1 = Go.**

**Contract executor = No-Go.**

## Phụ

- `cargo test --offline`: 6 test pass (4 unit + 2 integration). Không gửi tx. Không PRIVATE_KEY.
- `cargo run`: `rpc.ok eth_blockNumber=0x1451352 baseFeePerGas=0x4a817c800` (đọc, không ký).
- RTT RPC avg ~623 ms (min 324, max 1633).
- Dual-venue: CHƯA ĐO số pool (dexscreener search có tag `arc` — không PIN, không đếm TVL).
- Aero Lite factory: MISSING. Base factory getCode `0x` trên 5042.
- Push: xem ô dưới sau `git push`.

### Push

Kết quả `git push -u origin master` in ở chat Thợ (cấm token trong file này).
