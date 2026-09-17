# RUN — khung A3 (dry_run)

Máy: WSL Ubuntu, repo `/home/dmin/backrun-arc`. Không `/mnt/c`.

```bash
cd /home/dmin/backrun-arc
# không cần .env cho boot; mặc định ARC_HTTP=https://rpc.mainnet.arc.io
# quote/paper: nên ARC_HTTP=https://arc-rpc.publicnode.com (Circle dễ 429)
cargo test --offline
cargo run --release -- --discover    # ghi pairs_arb.txt + pairs_arb.rejected.txt
cargo run --release -- --watch-once  # 1 block + candidates/no_quote, send=0
cargo run --release -- --paper-seconds 0   # bảng 3 size × 2 token
cargo run --release -- --paper-seconds 30  # paper loop jsonl baocao/evidence/paper_vps.jsonl
```

Không gửi tx. Không điền `PRIVATE_KEY`. Copy `.env.example` → `.env` chỉ khi Chủ tự điền; file `.env` gitignore.

Thiếu field `config.toml` → `FAIL refuse: missing required fields: …` exit 1.
`pending_enabled=true` → refuse.
`chain_id != 5042` hoặc `strategy != "backrun"` → refuse.
