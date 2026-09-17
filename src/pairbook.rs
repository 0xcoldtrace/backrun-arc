//! Pairbook: đọc `pairs_arb.txt`.
//! Format A2: token,symbol,venues,depth_usd,tax_buy_bps,tax_sell_bps,ok
//! Dòng lỗi bỏ qua. File thiếu → entries rỗng, không panic.

use crate::logs::parse_address;
use alloy::primitives::Address;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct PairLine {
    pub token: Address,
    pub symbol: String,
    pub venues: String,
    pub depth_usd: f64,
    pub tax_buy_bps: u32,
    pub tax_sell_bps: u32,
    pub ok: bool,
}

#[derive(Debug, Clone)]
pub struct PairBook {
    pub path: PathBuf,
    pub entries: Vec<PairLine>,
    pub skipped_bad_lines: usize,
    pub missing_file: bool,
    pub header_only: bool,
}

impl PairBook {
    /// Không panic. Header-only (chỉ comment) → entries rỗng, header_only=true.
    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        let raw = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => {
                return Self {
                    path,
                    entries: Vec::new(),
                    skipped_bad_lines: 0,
                    missing_file: true,
                    header_only: true,
                };
            }
        };
        Self::from_str(&path, &raw)
    }

    pub fn from_str(path: impl AsRef<Path>, raw: &str) -> Self {
        let mut entries = Vec::new();
        let mut skipped_bad_lines = 0usize;
        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            match parse_pair_line(trimmed) {
                Some(e) => entries.push(e),
                None => skipped_bad_lines += 1,
            }
        }
        let header_only = entries.is_empty();
        Self {
            path: path.as_ref().to_path_buf(),
            entries,
            skipped_bad_lines,
            missing_file: false,
            header_only,
        }
    }
}

pub fn parse_pair_line(line: &str) -> Option<PairLine> {
    let line = match line.split_once('#') {
        Some((a, _)) => a.trim(),
        None => line.trim(),
    };
    if line.is_empty() {
        return None;
    }
    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
    if parts.len() < 7 {
        return None;
    }
    let token = parse_address(parts[0])?;
    let symbol = parts[1].to_string();
    if symbol.is_empty() {
        return None;
    }
    let venues = parts[2].to_string();
    if venues.is_empty() {
        return None;
    }
    let depth_usd: f64 = parts[3].parse().ok()?;
    let tax_buy_bps: u32 = parts[4].parse().ok()?;
    let tax_sell_bps: u32 = parts[5].parse().ok()?;
    let ok = matches!(
        parts[6].to_ascii_lowercase().as_str(),
        "ok" | "true" | "1" | "pass"
    );
    Some(PairLine {
        token,
        symbol,
        venues,
        depth_usd,
        tax_buy_bps,
        tax_sell_bps,
        ok,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;

    #[test]
    fn header_only_no_panic() {
        let raw = "# pairs_arb.txt\n# token,symbol,venues,depth_usd,tax_buy_bps,tax_sell_bps,ok\n";
        let b = PairBook::from_str("pairs_arb.txt", raw);
        assert!(!b.missing_file);
        assert!(b.header_only);
        assert!(b.entries.is_empty());
        assert_eq!(b.skipped_bad_lines, 0);
    }

    #[test]
    fn missing_file_no_panic() {
        let b = PairBook::load("/tmp/arc_arb_pairs_does_not_exist_a2.txt");
        assert!(b.missing_file);
        assert!(b.entries.is_empty());
        assert!(b.header_only);
    }

    #[test]
    fn parses_csv_and_skips_junk() {
        let raw = r#"
# header
0xbef5f6d51cb62b58e6a8f77868681825c6fe21c1,EURC,uni_v3+aero_cl,347738.56,0,0,ok
not-an-address,FOO,uni_v2,1,0,0,ok
0x3600000000000000000000000000000000000000,USDC,uni_v2,100.0,200,0,fail
"#;
        let b = PairBook::from_str("x", raw);
        assert_eq!(b.entries.len(), 2);
        assert_eq!(b.skipped_bad_lines, 1);
        assert!(!b.header_only);
        assert_eq!(
            b.entries[0].token,
            address!("0xbef5f6d51cb62b58e6a8f77868681825c6fe21c1")
        );
        assert!(b.entries[0].ok);
        assert_eq!(b.entries[0].venues, "uni_v3+aero_cl");
        assert!(!b.entries[1].ok);
        assert_eq!(b.entries[1].tax_buy_bps, 200);
    }
}
