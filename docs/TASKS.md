# TASKS

## A2 (xong — pairbook vet + aero + VPS latency)

- Pin UR / V2 Router / SwapRouter02 / UniswapX reactor / Aero CLFactory (getCode != 0x).
- `pairs_arb.txt` vet on-chain (depth USDC*2/1e6, tax eth_call dust, ≥2 venue).
- `--discover` + `--watch-once` candidates / no_quote. send=0.
- VPS NJ TTFB CSV + paper watch-once.

## Nợ (không chặn)

- `web/` stub dashboard — chưa làm.
- V4 per-pool depth (PoolManager gộp token) — unread, không tính chân.
- Aero quoter — chưa pin; watch in `no_quote` nếu chỉ 1 venue quote được.
- `flashFee` IERC3156 revert trên Morpho Arc; không gọi `flashLoan`.
- sim_arb / getAmountOut đa venue profit — chưa (cấm bịa profit).

## Cấm tới khi lệnh riêng

- Contract executor / `ArbExecutor.sol`.
- live / sendRaw / bundle / PRIVATE_KEY runtime.
- `pending_enabled=true`, subscribe pending, 48Club/Flashbots.
- Sandwich path. Fork nguyên `bsc-sandwich`.
- Sửa `$ALL`.
