use arc_arb::Config;
use std::path::Path;

#[test]
fn repo_config_toml_loads() {
    let c = Config::from_path(Path::new("config.toml")).expect("config.toml");
    assert_eq!(c.chain_id, 5042);
    assert_eq!(c.strategy, "backrun");
    assert!(c.dry_run);
    assert!(!c.allow_live);
    assert!(!c.bot_armed);
    assert_eq!(c.live_mode, "off");
    assert!(!c.pending_enabled);
    assert_eq!(c.flash_provider, "morpho");
    assert_eq!(c.pairs_arb_path, "pairs_arb.txt");
    assert_eq!(c.min_profit_usdc, 2.0);
    assert_eq!(c.arb_max_borrow_usdc, 20000.0);
    assert_eq!(c.pairs_min_swap_usdc, 200.0);
    assert_eq!(c.min_depth_usd, 8000.0);
    assert_eq!(c.min_base_fee_gwei, 20);
    assert_eq!(c.config_reload_sec, 15);
}

#[test]
fn fixture_missing_refuse() {
    let raw = include_str!("fixtures/config.missing.toml");
    let err = Config::from_toml_str(raw).unwrap_err();
    assert!(err.starts_with("refuse:"), "{err}");
    assert!(err.contains("flash_provider"), "{err}");
}
