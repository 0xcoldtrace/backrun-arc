# TASKS

## A3 (xong — paper quote 2 chân EURC/cirBTC)

- Pin Uni V3 QuoterV2 (getCode != 0x). Aero Quoter MISSING → swap-static MiniQuoter overlay.
- Paper 100/1000/5000 USDC, aero_cl vs uni_v3. `--paper-seconds`. send=0.
- WETH 1 dòng docs; không pairbook (1 venue sâu).

## Nợ (không chặn)

- `web/` stub dashboard — chưa làm.
- V4 per-pool depth (PoolManager gộp token) — unread, không tính chân, không quote V4 A3.
- `flashFee` IERC3156 revert trên Morpho Arc; không gọi `flashLoan`.
- Aero Quoter contract — MISSING (dùng overlay).

## Cấm tới khi lệnh riêng

- Contract executor / `ArbExecutor.sol`.
- live / sendRaw / bundle / PRIVATE_KEY runtime.
- `pending_enabled=true`, subscribe pending, 48Club/Flashbots.
- Sandwich path. Fork nguyên `bsc-sandwich`.
- Sửa `$ALL`.
