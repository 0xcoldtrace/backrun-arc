use serde::Deserialize;
use std::path::Path;
use toml::Value;

/// Field bắt buộc. Thiếu 1 cái = refuse (không default âm thầm).
pub const REQUIRED_FIELDS: &[&str] = &[
    "strategy",
    "chain_id",
    "dry_run",
    "allow_live",
    "bot_armed",
    "live_mode",
    "pairs_arb_path",
    "min_profit_usdc",
    "arb_max_borrow_usdc",
    "pairs_min_swap_usdc",
    "min_depth_usd",
    "min_base_fee_gwei",
    "pending_enabled",
    "flash_provider",
    "config_reload_sec",
];

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Config {
    pub strategy: String,
    pub chain_id: u64,
    pub dry_run: bool,
    pub allow_live: bool,
    pub bot_armed: bool,
    pub live_mode: String,
    pub pairs_arb_path: String,
    pub min_profit_usdc: f64,
    pub arb_max_borrow_usdc: f64,
    pub pairs_min_swap_usdc: f64,
    pub min_depth_usd: f64,
    pub min_base_fee_gwei: u64,
    pub pending_enabled: bool,
    pub flash_provider: String,
    pub config_reload_sec: u64,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("refuse: read {}: {e}", path.display()))?;
        Self::from_toml_str(&raw)
    }

    pub fn from_toml_str(raw: &str) -> Result<Self, String> {
        let value: Value =
            toml::from_str(raw).map_err(|e| format!("refuse: toml parse: {e}"))?;
        let table = value
            .as_table()
            .ok_or_else(|| "refuse: config root must be a table".to_string())?;
        let missing: Vec<&str> = REQUIRED_FIELDS
            .iter()
            .copied()
            .filter(|k| !table.contains_key(*k))
            .collect();
        if !missing.is_empty() {
            return Err(format!(
                "refuse: missing required fields: {}",
                missing.join(", ")
            ));
        }
        let cfg: Config = value
            .try_into()
            .map_err(|e| format!("refuse: type: {e}"))?;
        if cfg.chain_id != 5042 {
            return Err(format!("refuse: chain_id={} want 5042", cfg.chain_id));
        }
        if cfg.strategy != "backrun" {
            return Err(format!("refuse: strategy={} want backrun", cfg.strategy));
        }
        if cfg.pending_enabled {
            return Err(
                "refuse: pending_enabled=true — Arc pending RPC chết, phải false".into(),
            );
        }
        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: &str = r#"
strategy = "backrun"
chain_id = 5042
dry_run = true
allow_live = false
bot_armed = false
live_mode = "off"
pairs_arb_path = "pairs_arb.txt"
min_profit_usdc = 2.0
arb_max_borrow_usdc = 20000
pairs_min_swap_usdc = 200
min_depth_usd = 8000
min_base_fee_gwei = 20
pending_enabled = false
flash_provider = "morpho"
config_reload_sec = 15
"#;

    #[test]
    fn load_min_ok() {
        let c = Config::from_toml_str(MIN).expect("min config");
        assert_eq!(c.strategy, "backrun");
        assert_eq!(c.chain_id, 5042);
        assert!(c.dry_run);
        assert!(!c.allow_live);
        assert!(!c.bot_armed);
        assert_eq!(c.live_mode, "off");
        assert!(!c.pending_enabled);
        assert_eq!(c.flash_provider, "morpho");
        assert_eq!(c.min_base_fee_gwei, 20);
    }

    #[test]
    fn missing_field_refuse() {
        let broken = MIN.replace("pending_enabled = false\n", "");
        let err = Config::from_toml_str(&broken).unwrap_err();
        assert!(err.contains("refuse"), "{err}");
        assert!(err.contains("pending_enabled"), "{err}");
    }

    #[test]
    fn pending_true_refuse() {
        let broken = MIN.replace("pending_enabled = false", "pending_enabled = true");
        let err = Config::from_toml_str(&broken).unwrap_err();
        assert!(err.contains("pending_enabled"), "{err}");
    }

    #[test]
    fn wrong_chain_refuse() {
        let broken = MIN.replace("chain_id = 5042", "chain_id = 56");
        let err = Config::from_toml_str(&broken).unwrap_err();
        assert!(err.contains("5042"), "{err}");
    }
}
