# AGENTS.md — Arc backrun-arb (chain 5042)

Thợ = Grok Code (phiên trắng, chỉ tin file repo + khối lệnh).
Điều hành = Grok chat (không đọc đĩa Chủ). Chủ copy lệnh ↔ BAOCAO.

## Luật cụm

- 1 cụm = 1 commit = 1 `baocao/BAOCAO{NN}.md`. Hết phiên không BAOCAO = FAIL.
- Cấm chữ ĐẠT trong BAOCAO. Cấm in IP, token, PRIVATE_KEY, GitHub key.
- Cấm sửa `/home/dmin/all`. Cấm copy file từ `$ALL` sang repo này (chỉ kê `docs/INVENTORY.md` + `docs/REUSE.md`).
- Cấm sandwich. Cấm đứng trước victim. Cấm pending public. Cấm Flashbots/48Club bịa. Cấm `ArbExecutor.sol`. Cấm live / sendRaw / bundle.
- `strategy="backrun"` cố định. `pending_enabled=false` cố định (Arc pending RPC chết).
- `dry_run=true` `allow_live=false` `bot_armed=false` `live_mode="off"` mặc định. Thiếu field config = refuse load.
- Contract executor = No-Go cho tới lệnh riêng của Điều hành.

## Chain (không sửa)

- chainId 5042 (`0x13b2`). RPC `https://rpc.mainnet.arc.io`. Explorer `https://explorer.arc.io`. Docs `https://docs.arc.io`.
- Gas = USDC native 18 dec. USDC ERC-20 `0x3600000000000000000000000000000000000000` 6 dec (landmine).
- EURC `0xbEf5f6d51CB62b58e6A8f77868681825C6fe21c1`.
- Block ~500ms, Malachite BFT, finality 1 block, không reorg. min maxFeePerGas 20 gwei.
- Proposer = PoA rotate. Không cổng builder public đã document.
- Pin address chỉ khi `eth_getCode != 0x` + nguồn URL + ngày. Cấm copy Morpho `0xBBBB` Ethereum nếu code 5042 không phải Morpho.

## Chiến lược

1. Trigger: swap LỚN đã nằm trong block (`newHeads` / logs). Token ≥ 2 venue đủ depth.
2. Trigger: 2 pool cùng token đã lệch giá.
3. 1 tx: vay flash phí protocol 0 (Morpho) → mua pool rẻ → bán pool đắt → trả đúng principal → giữ chênh.

## Stack

Rust + tokio (A0: boot load + JSON-RPC đọc). RPC: reqwest blocking JSON-RPC; A1+ chọn 1 alloy **hoặc** ethers-rs, ghi `docs/STATE.md`. Cấm npm app runtime.
