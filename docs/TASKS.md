# TASKS

## A1 (xong — decoder + rpc list)

- Swap logs V2/V3/V4 PoolManager decode bằng alloy (`rpc_crate=alloy`).
- Nguồn = logs block đã final (HTTP latest head / newHeads). Không pending.
- Pairbook skeleton đọc `pairs_arb.txt` (header-only, Chủ chưa vet).
- `--watch-once`: 1 block, in số swap decode, send=0.

## Nợ (không chặn A2 sim)

- `web/` stub dashboard — chưa làm.
- Aero Lite factory Arc — MISSING (Base factory getCode `0x` trên 5042).
- Dual-venue depth/TVL — 20 block logs suy ra 6 token ≥2 pool; depth **CHƯA ĐO**.
- `flashFee` IERC3156 revert trên Morpho Arc; xác minh hoàn vốn 0 phí bằng sim/eth_call sau, không gửi tx.
- Depth/min_swap filter theo config — load rồi, chưa sim đa venue.
- sim_arb / getAmountOut / slot0 — chưa (A1 cấm sim_arb).

## Cấm tới khi lệnh riêng

- Contract executor / `ArbExecutor.sol`.
- live / sendRaw / bundle / PRIVATE_KEY runtime.
- `pending_enabled=true`, subscribe pending, 48Club/Flashbots.
- Sandwich path. Fork nguyên `bsc-sandwich`.
- Sửa `$ALL`.
