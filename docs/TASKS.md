# TASKS

## A1 (tiếp theo — Go)

- Pin venue còn thiếu decoder: Swap logs only (V2 `Swap`, V3 `Swap`, V4 `Swap`/PoolManager) trên factory/PM đã pin.
- Không decoder pending tx. Nguồn = logs block đã final (`newHeads`).
- Pairbook skeleton đọc `pairs_arb.txt` (vẫn header-only cho tới Chủ vet).
- Depth/min_swap filter theo config (chưa sim đa venue).

## Nợ A0 (không chặn A1)

- `web/` stub dashboard — chưa làm.
- Aero Lite factory Arc — MISSING (cần explorer/logs, không lấy Base).
- Dual-venue count (số token ≥2 pool đủ depth) — CHƯA ĐO.
- `flashFee` IERC3156 revert trên Morpho Arc; xác minh hoàn vốn 0 phí bằng sim/eth_call sau, không gửi tx.
- Chọn alloy **hoặc** ethers-rs (đúng 1) khi nối decoder.

## Cấm tới khi lệnh riêng

- Contract executor / `ArbExecutor.sol`.
- live / sendRaw / bundle / PRIVATE_KEY runtime.
- `pending_enabled=true`, subscribe pending, 48Club/Flashbots.
- Sandwich path. Fork nguyên `bsc-sandwich`.
- Sửa `$ALL`.
