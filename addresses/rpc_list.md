# RPC list — Arc mainnet chain 5042 (`0x13b2`)

utc_probe=2026-09-17
paper = `newHeads` / logs block đã final. **Không pending.**
UA bắt buộc (A0: urllib không UA = Cloudflare 403/1010).
timeout_probe=8s. chainId phải `0x13b2`. Không testnet `5042002` / `rpc.testnet.arc.io` / chain 1243.
Không bịa RTT — số dưới là đo thật.

Client: `Mozilla/5.0 (compatible; arc_arb/0.1; +https://github.com/0xcoldtrace/backrun-arc)`

## Đo thật — HTTP `eth_chainId`

| URL | http | chainId | rtt_ms | note |
|---|---|---|---|---|
| https://rpc.mainnet.arc.io | 200 | 0x13b2 | 517.38 | Circle — OK, UA bắt buộc |
| https://rpc.blockdaemon.mainnet.arc.io | 200 | 0x13b2 | 735.03 | OK |
| https://rpc.drpc.mainnet.arc.io | 200 | 0x13b2 | 825.08 | OK |
| https://rpc.quicknode.mainnet.arc.io | 200 | 0x13b2 | 503.29 | OK |
| https://arc.drpc.org | 200 | 0x13b2 | 529.91 | OK |
| https://5042.rpc.thirdweb.com | 200 | 0x13b2 | 884.67 | OK (5042 = mainnet, không phải testnet 5042002) |
| https://arc.rpc.thirdweb.com | 200 | 0x13b2 | 821.66 | OK |
| https://arc-rpc.publicnode.com | 200 | 0x13b2 | 450.92 | OK |
| https://arc.gateway.tenderly.co | 200 | 0x13b2 | 541.98 | OK |
| https://gateway.tenderly.co/public/arc | 200 | 0x13b2 | 582.03 | OK |
| https://arc.gateway.tenderly.co/public | 200 | 0x13b2 | 498.54 | OK |
| https://nodes.sequence.app/arc | 200 | 0x13b2 | 569.88 | OK |
| https://rpc.nodeflare.app/arc/public | 200 | 0x13b2 | 905.88 | OK |
| https://api.zan.top/arc-mainnet | 200 | 0x13b2 | 642.30 | OK |
| https://rpc.arc-scan.org | 200 | 0x13b2 | 866.83 | HTTP only, UA OK, chainId 0x13b2 |
| https://arc.rpc.pinax.network | 200 | 0x13b2 | 1344.04 | Pinax chainId 0x13b2 |
| https://arc.rpc.pinax.network/v1/ | 200 | 0x13b2 | 1291.01 | Pinax /v1/ cùng chainId |

## HTTP fail (đo, không dùng làm primary)

| URL | rtt_ms | error |
|---|---|---|
| https://arc-mainnet.gateway.tatum.io | 435.71 | HTTP 404 |
| https://arc.gateway.tatum.io | 414.32 | HTTP 404 |
| https://lb.routeme.sh/rpc/evm/5042 | 410.58 | HTTP 429 |

## Đo thật — WSS `eth_subscribe newHeads` (timeout 8s)

Circle docs ghi HTTP-only — **đo thật: ALIVE** (subscribe id trả về).

| URL | newHeads | rtt_ms | note |
|---|---|---|---|
| wss://rpc.mainnet.arc.io | ALIVE | 720 | Circle WSS sống; subscribed id |
| wss://rpc.blockdaemon.mainnet.arc.io/websocket | ALIVE | 772 | subscribed |
| wss://rpc.quicknode.mainnet.arc.io | ALIVE | 894 | subscribed — `.env.example` ARC_WS |
| wss://rpc.drpc.mainnet.arc.io | DEAD | 3503 | error code 23 `Unsupported subscription: newHeads` |
| wss://arc.drpc.org | DEAD | 538 | error code 23 `Unsupported subscription: newHeads` |
| wss://arc-rpc.publicnode.com | ALIVE | 573 | subscribed — `.env.example` ARC_WS_2 |
| wss://arc.gateway.tenderly.co | ALIVE | 593 | subscribed |
| wss://gateway.tenderly.co/public/arc | ALIVE | 659 | subscribed |
| wss://arc.gateway.tenderly.co/public | ALIVE | 830 | subscribed |

`--watch-once` dùng HTTP `eth_blockNumber` latest (tương đương 1 head) rồi `eth_getLogs` 1 block. Không pending.

## Primary (khớp `.env.example`)

- ARC_HTTP=https://rpc.mainnet.arc.io
- ARC_HTTP_2=https://rpc.drpc.mainnet.arc.io
- ARC_HTTP_3=https://arc-rpc.publicnode.com
- ARC_HTTP_4=https://arc.rpc.pinax.network
- ARC_WS=wss://rpc.quicknode.mainnet.arc.io
- ARC_WS_2=wss://arc-rpc.publicnode.com
