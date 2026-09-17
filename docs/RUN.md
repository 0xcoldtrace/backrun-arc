# RUN — khung A1 (dry_run)

Máy: WSL Ubuntu, repo `/home/dmin/backrun-arc`. Không `/mnt/c`.

```bash
cd /home/dmin/backrun-arc
# không cần .env cho boot; mặc định ARC_HTTP=https://rpc.mainnet.arc.io
cargo test --offline
cargo run --release -- --watch-once
```

`cargo run --release -- --watch-once` in `config.ok` + pairbook + 1 block Swap decode + `send=0` rồi thoát.
`cargo run` (không flag) = boot A0: `eth_blockNumber` + `baseFeePerGas`.

Không gửi tx. Không điền `PRIVATE_KEY`. Copy `.env.example` → `.env` chỉ khi Chủ tự điền; file `.env` gitignore.

Thiếu field `config.toml` → `FAIL refuse: missing required fields: …` exit 1.
`pending_enabled=true` → refuse.
`chain_id != 5042` hoặc `strategy != "backrun"` → refuse.
