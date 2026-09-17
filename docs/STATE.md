# STATE — A0 INV-01 (2026-09-17)

Repo: `/home/dmin/backrun-arc` (lệnh ghi `ARC=/home/dmin/backrun-arb`; workspace + remote = `backrun-arc`).
Remote: `https://github.com/0xcoldtrace/backrun-arc.git`. Nhánh `master`.

## Chain đo thật (RPC `https://rpc.mainnet.arc.io`)

- `eth_chainId` = `0x13b2` (5042).
- `eth_blockNumber` lúc probe = `0x1451030` (21303344). latest.number = `0x1451031`.
- `eth_gasPrice` = `0x4a817c800` (20 gwei). `baseFeePerGas` = `0x4a817c800` (20 gwei).
- USDC ERC-20 `0x3600…0000`: getCode 1798 bytes, `decimals()=6`, `symbol()=USDC`.
- `eth_getBlockByNumber "pending"`: error `-32014` `"requested data not available"` (result null).
- `eth_newPendingTransactionFilter`: error `-32601` `"method not supported"`.
- urllib không UA → Cloudflare HTTP 403 code 1010. curl / UA browser → 200. Client phải gửi User-Agent.
- RTT `eth_chainId` x5: 324–1633 ms, avg ~623 ms.

## Pin (getCode != 0x)

Xem `docs/DEX_REGISTRY.md`. Uni V4 PoolManager, Uni V3 factory, Uni V2 factory, AchSwap V2 factory, Permit2 đều có code.

## Flash

- Morpho Blue Arc: `0x34CD04070dD72b14E241112F6d83812Df5Af7fCD`. getCode 15582 bytes. Bytecode chứa selector `flashLoan(address,uint256,bytes)` = `0xe0232b42`. Nguồn: Morpho docs tab Arc.
- `0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb` getCode `0x` trên 5042 — không copy.
- `flashFee(address,uint256)` IERC3156 trên Morpho Arc revert (code 3). Morpho Blue không phải ERC-3156; phí flash = 0 theo protocol spec, chưa gọi `flashLoan` (cấm gửi tx).
- Aave V4 Core Hub Arc: `0x17288dfc86205301064577b98B02b81017e6F79C` getCode 1419 bytes (proxy). Nguồn aave-address-book `AaveV4Arc.ts`. Flash CÓ phí — fallback only.
- Aero Lite factory Arc: MISSING. Base `0x420D…40Da` getCode `0x` trên 5042 (không pin).

## Dual-venue

CHƯA ĐO số pool dual-venue (không bịa). Dexscreener search có pair tag `arc` — không dùng làm PIN.

## Khung bot

- Package `arc_arb`. Boot: load `config.toml` (thiếu field = refuse) + `eth_blockNumber` + `baseFeePerGas`. `pending_enabled=false`.
- RPC crate: reqwest blocking JSON-RPC (A0). A1+ chưa chọn alloy vs ethers-rs.
- `$ALL`: đọc 14/14 folder, không copy file sang repo này.
- Contract executor: No-Go.
- web/ stub: chưa làm (nợ TASKS).
