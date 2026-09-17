# RUN — khung A0 (dry_run)

Máy: WSL Ubuntu, repo `/home/dmin/backrun-arc`. Không `/mnt/c`.

```bash
cd /home/dmin/backrun-arc
# không cần .env cho boot; mặc định ARC_HTTP=https://rpc.mainnet.arc.io
cargo test --offline
cargo run
```

`cargo run` in `config.ok` + `pending_enabled=false` + `rpc.ok` (`eth_blockNumber`, `baseFeePerGas`) rồi thoát.

Không gửi tx. Không điền `PRIVATE_KEY`. Copy `.env.example` → `.env` chỉ khi Chủ tự điền; file `.env` gitignore.

Thiếu field `config.toml` → `FAIL refuse: missing required fields: …` exit 1.
`pending_enabled=true` → refuse.
`chain_id != 5042` hoặc `strategy != "backrun"` → refuse.
