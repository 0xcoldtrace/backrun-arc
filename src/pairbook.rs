//! Pairbook skeleton: đọc `pairs_arb.txt`. Header-only không panic.
//! Dòng lỗi bỏ qua. File thiếu → entries rỗng, không panic.

use crate::logs::parse_address;
use alloy::primitives::Address;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairLine {
    pub token: Address,
    pub quote: Option<Address>,
    pub note: String,
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

fn parse_pair_line(line: &str) -> Option<PairLine> {
    let (addr_part, note) = match line.split_once('#') {
        Some((a, n)) => (a.trim(), n.trim().to_string()),
        None => (line.trim(), String::new()),
    };
    if addr_part.is_empty() {
        return None;
    }
    let mut parts = addr_part.split(',').map(|s| s.trim()).filter(|s| !s.is_empty());
    let token = parse_address(parts.next()?)?;
    let quote = match parts.next() {
        Some(q) => Some(parse_address(q)?),
        None => None,
    };
    if parts.next().is_some() {
        return None;
    }
    Some(PairLine { token, quote, note })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::address;

    #[test]
    fn header_only_no_panic() {
        let raw = include_str!("../pairs_arb.txt");
        let b = PairBook::from_str("pairs_arb.txt", raw);
        assert!(!b.missing_file);
        assert!(b.header_only);
        assert!(b.entries.is_empty());
        assert_eq!(b.skipped_bad_lines, 0);
    }

    #[test]
    fn missing_file_no_panic() {
        let b = PairBook::load("/tmp/arc_arb_pairs_does_not_exist_a1.txt");
        assert!(b.missing_file);
        assert!(b.entries.is_empty());
        assert!(b.header_only);
    }

    #[test]
    fn parses_token_and_skips_junk() {
        let raw = r#"
# header
0x3600000000000000000000000000000000000000 # USDC
0xbef5f6d51cb62b58e6a8f77868681825c6fe21c1,0x3600000000000000000000000000000000000000 # EURC
not-an-address
"#;
        let b = PairBook::from_str("x", raw);
        assert_eq!(b.entries.len(), 2);
        assert_eq!(b.skipped_bad_lines, 1);
        assert!(!b.header_only);
        assert_eq!(
            b.entries[0].token,
            address!("0x3600000000000000000000000000000000000000")
        );
        assert_eq!(
            b.entries[1].quote,
            Some(address!("0x3600000000000000000000000000000000000000"))
        );
    }
}
