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
            .timeout(std::time::Duration::from_secs(20))
            .build()
            .map_err(|e| RpcError::Http(e.to_string()))?;
        Ok(Self { url, http })
    }

    pub fn call(&self, method: &str, params: Value) -> Result<Value, RpcError> {
        let body = json!({"jsonrpc":"2.0","id":1,"method":method,"params":params});
        let resp = self
            .http
            .post(&self.url)
            .json(&body)
            .send()
            .map_err(|e| RpcError::Http(e.to_string()))?;
        let status = resp.status();
        let v: Value = resp.json().map_err(|e| RpcError::Http(e.to_string()))?;
        if !status.is_success() {
            return Err(RpcError::Http(format!("http {status} body={v}")));
        }
        if let Some(err) = v.get("error") {
            return Err(RpcError::Rpc(err.to_string()));
        }
        Ok(v.get("result").cloned().unwrap_or(Value::Null))
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
}
