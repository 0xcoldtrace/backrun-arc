use serde_json::{json, Value};

/// Cloudflare trên rpc.mainnet.arc.io trả 1010 nếu thiếu User-Agent (urllib mặc định).
const UA: &str = "Mozilla/5.0 (compatible; arc_arb/0.1; +https://github.com/0xcoldtrace/backrun-arc)";

#[derive(Debug)]
pub enum RpcError {
    Http(String),
    Rpc(String),
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::Http(s) | RpcError::Rpc(s) => write!(f, "{s}"),
        }
    }
}

pub struct RpcClient {
    url: String,
    http: reqwest::blocking::Client,
}

impl RpcClient {
    pub fn new(url: String) -> Result<Self, RpcError> {
        let http = reqwest::blocking::Client::builder()
            .user_agent(UA)
            .timeout(std::time::Duration::from_secs(45))
            .build()
            .map_err(|e| RpcError::Http(e.to_string()))?;
        Ok(Self { url, http })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    fn is_rate_limit(status: reqwest::StatusCode, body: &Value) -> bool {
        if status.as_u16() == 429 {
            return true;
        }
        body.get("error")
            .map(|e| e.to_string().to_ascii_lowercase())
            .map(|s| s.contains("rate limit") || s.contains("too many") || s.contains("-32005"))
            .unwrap_or(false)
    }

    pub fn call(&self, method: &str, params: Value) -> Result<Value, RpcError> {
        let body = json!({"jsonrpc":"2.0","id":1,"method":method,"params":params});
        let mut last = String::new();
        for i in 0..8 {
            let resp = self
                .http
                .post(&self.url)
                .json(&body)
                .send()
                .map_err(|e| RpcError::Http(e.to_string()))?;
            let status = resp.status();
            let v: Value = resp.json().map_err(|e| RpcError::Http(e.to_string()))?;
            if Self::is_rate_limit(status, &v) {
                last = format!("http {status} body={v}");
                std::thread::sleep(std::time::Duration::from_millis(400 * (i as u64 + 1)));
                continue;
            }
            if !status.is_success() {
                return Err(RpcError::Http(format!("http {status} body={v}")));
            }
            if let Some(err) = v.get("error") {
                return Err(RpcError::Rpc(err.to_string()));
            }
            return Ok(v.get("result").cloned().unwrap_or(Value::Null));
        }
        Err(RpcError::Http(format!("rate_limit: {last}")))
    }

    /// JSON-RPC batch. Mỗi phần tử = Ok(result) hoặc Err(rpc/http).
    pub fn batch(&self, reqs: &[(String, Value)]) -> Result<Vec<Result<Value, RpcError>>, RpcError> {
        if reqs.is_empty() {
            return Ok(Vec::new());
        }
        let body: Vec<Value> = reqs
            .iter()
            .enumerate()
            .map(|(i, (m, p))| json!({"jsonrpc":"2.0","id":i,"method":m,"params":p}))
            .collect();
        let mut last = String::new();
        let v = {
            let mut got: Option<Value> = None;
            for i in 0..8 {
                let resp = self
                    .http
                    .post(&self.url)
                    .json(&body)
                    .send()
                    .map_err(|e| RpcError::Http(e.to_string()))?;
                let status = resp.status();
                let parsed: Value = resp.json().map_err(|e| RpcError::Http(e.to_string()))?;
                if Self::is_rate_limit(status, &parsed) {
                    last = format!("http {status} body={parsed}");
                    std::thread::sleep(std::time::Duration::from_millis(400 * (i as u64 + 1)));
                    continue;
                }
                if !status.is_success() {
                    return Err(RpcError::Http(format!("http {status} body={parsed}")));
                }
                got = Some(parsed);
                break;
            }
            got.ok_or_else(|| RpcError::Http(format!("rate_limit: {last}")))?
        };
        let arr = v
            .as_array()
            .ok_or_else(|| RpcError::Rpc(format!("batch not array: {v}")))?;
        let mut out: Vec<Result<Value, RpcError>> = (0..reqs.len())
            .map(|_| Err(RpcError::Rpc("batch missing id".into())))
            .collect();
        for item in arr {
            let id = item.get("id").and_then(|x| x.as_u64()).unwrap_or(u64::MAX) as usize;
            if id >= out.len() {
                continue;
            }
            if let Some(err) = item.get("error") {
                out[id] = Err(RpcError::Rpc(err.to_string()));
            } else {
                out[id] = Ok(item.get("result").cloned().unwrap_or(Value::Null));
            }
        }
        Ok(out)
    }

    pub fn batch_eth_call(&self, calls: &[(String, String)]) -> Result<Vec<Result<String, RpcError>>, RpcError> {
        let reqs: Vec<(String, Value)> = calls
            .iter()
            .map(|(to, data)| {
                (
                    "eth_call".to_string(),
                    json!([{"to": to, "data": data}, "latest"]),
                )
            })
            .collect();
        let raw = self.batch(&reqs)?;
        Ok(raw
            .into_iter()
            .map(|r| {
                r.and_then(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .ok_or_else(|| RpcError::Rpc(format!("eth_call not hex: {v}")))
                })
            })
            .collect())
    }

    pub fn block_number_hex(&self) -> Result<String, RpcError> {
        let v = self.call("eth_blockNumber", json!([]))?;
        v.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| RpcError::Rpc(format!("eth_blockNumber not string: {v}")))
    }

    pub fn latest_base_fee_hex(&self) -> Result<(String, String), RpcError> {
        let v = self.call("eth_getBlockByNumber", json!(["latest", false]))?;
        let num = v
            .get("number")
            .and_then(|x| x.as_str())
            .ok_or_else(|| RpcError::Rpc("latest.number missing".into()))?
            .to_string();
        let fee = v
            .get("baseFeePerGas")
            .and_then(|x| x.as_str())
            .ok_or_else(|| RpcError::Rpc("latest.baseFeePerGas missing".into()))?
            .to_string();
        Ok((num, fee))
    }

    /// eth_getLogs. `topic0_or` = OR-list for topics[0]. Không pending.
    pub fn get_logs(
        &self,
        from_block: &str,
        to_block: &str,
        address: Option<&str>,
        topic0_or: &[&str],
    ) -> Result<Vec<Value>, RpcError> {
        let mut filter = serde_json::Map::new();
        filter.insert("fromBlock".into(), json!(from_block));
        filter.insert("toBlock".into(), json!(to_block));
        if let Some(addr) = address {
            filter.insert("address".into(), json!(addr));
        }
        if !topic0_or.is_empty() {
            let t0: Vec<Value> = topic0_or.iter().map(|t| json!(t)).collect();
            filter.insert("topics".into(), json!([t0]));
        }
        let v = self.call("eth_getLogs", json!([Value::Object(filter)]))?;
        v.as_array()
            .cloned()
            .ok_or_else(|| RpcError::Rpc(format!("eth_getLogs not array: {v}")))
    }

    pub fn eth_call(&self, to: &str, data: &str) -> Result<String, RpcError> {
        self.eth_call_ex(to, data, None, None)
    }

    pub fn eth_call_ex(
        &self,
        to: &str,
        data: &str,
        from: Option<&str>,
        state_override: Option<Value>,
    ) -> Result<String, RpcError> {
        let mut tx = serde_json::Map::new();
        tx.insert("to".into(), json!(to));
        tx.insert("data".into(), json!(data));
        if let Some(f) = from {
            tx.insert("from".into(), json!(f));
        }
        let mut params = vec![Value::Object(tx), json!("latest")];
        if let Some(ov) = state_override {
            params.push(ov);
        }
        let v = self.call("eth_call", Value::Array(params))?;
        v.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| RpcError::Rpc(format!("eth_call not hex: {v}")))
    }

    pub fn get_code_hex(&self, addr: &str) -> Result<String, RpcError> {
        let v = self.call("eth_getCode", json!([addr, "latest"]))?;
        v.as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| RpcError::Rpc(format!("eth_getCode not hex: {v}")))
    }
}

pub fn hex_to_u256_str(s: &str) -> Option<u128> {
    let s = s.trim().trim_start_matches("0x");
    if s.is_empty() {
        return Some(0);
    }
    u128::from_str_radix(s, 16).ok()
}

#[cfg(test)]
mod tests {
    use super::hex_to_u256_str;

    #[test]
    fn parse_hex_amount() {
        assert_eq!(hex_to_u256_str("0x0"), Some(0));
        assert_eq!(hex_to_u256_str("0x0a"), Some(10));
        assert_eq!(hex_to_u256_str("0x2386f26fc10000"), Some(10_000_000_000_000_000));
    }
}
