use color_eyre::eyre::{Result, WrapErr, eyre};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<P> {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: P,
}

#[derive(Debug, Deserialize)]
pub struct RpcEnvelope<T> {
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

// getTransactionsForAddress types, unused while the app looks up one transaction by signature.
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct TxPage {
//     pub data: Vec<SignatureInfo>,
//     pub pagination_token: Option<String>,
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct SignatureInfo {
//     pub signature: String,
//     pub slot: u64,
//     pub err: Option<serde_json::Value>,
//     pub block_time: Option<i64>,
//     pub confirmation_status: String,
// }

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetail {
    pub slot: u64,
    pub block_time: Option<i64>,
    pub version: Option<Value>,
    pub meta: Option<TxMeta>,
    pub transaction: TxBody,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxMeta {
    pub err: Option<Value>,
    pub fee: u64,
    pub compute_units_consumed: Option<u64>,
    pub log_messages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TxBody {
    pub signatures: Vec<String>,
    pub message: TxMessage,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TxMessage {
    pub account_keys: Vec<AccountKey>,
    pub recent_blockhash: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccountKey {
    pub pubkey: String,
    pub signer: bool,
    pub writable: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub program_id: String,
    pub program: Option<String>,
}

#[derive(Clone)]
pub struct Rpc {
    signature: String,
    url: String,
    client: reqwest::blocking::Client,
}

impl Default for Rpc {
    fn default() -> Self {
        Self {
            signature: String::new(),
            url: String::new(),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl Rpc {
    pub fn add(&mut self, signature: impl Into<String>) -> Result<()> {
        let api_key = std::env::var("HELIUS_API_KEY")
            .wrap_err("HELIUS_API_KEY environment variable is not set")?;

        self.signature = signature.into();
        self.url = format!("https://mainnet.helius-rpc.com/?api-key={api_key}");
        Ok(())
    }

    // fn build_body(&self) -> JsonRpcRequest<Vec<String>> {
    //     JsonRpcRequest {
    //         jsonrpc: "2.0".into(),
    //         id: "1".into(),
    //         method: "getTransactionsForAddress".into(),
    //         params: vec![self.address.clone()],
    //     }
    // }

    fn build_body(&self) -> JsonRpcRequest<Value> {
        JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: "1".into(),
            method: "getTransaction".into(),
            params: json!([
                self.signature,
                { "encoding": "jsonParsed", "maxSupportedTransactionVersion": 0 }
            ]),
        }
    }

    // pub fn fetch_data(&self) -> Result<Vec<SignatureInfo>> {
    //     let resp = self
    //         .client
    //         .post(&self.url)
    //         .json(&self.build_body())
    //         .send()
    //         .wrap_err("failed to send request")?
    //         .error_for_status()
    //         .wrap_err("Helius returned an error status")?
    //         .json::<RpcEnvelope<TxPage>>()
    //         .wrap_err("failed to parse response")?;
    //
    //     if let Some(e) = resp.error {
    //         return Err(eyre!("RPC error {}: {}", e.code, e.message));
    //     }
    //
    //     Ok(resp
    //         .result
    //         .ok_or_else(|| eyre!("response had no result"))?
    //         .data)
    // }

    pub fn fetch_data(&self) -> Result<TransactionDetail> {
        let resp = self
            .client
            .post(&self.url)
            .json(&self.build_body())
            .send()
            .wrap_err("failed to send request")?
            .error_for_status()
            .wrap_err("Helius returned an error status")?
            .json::<RpcEnvelope<TransactionDetail>>()
            .wrap_err("failed to parse response")?;

        if let Some(e) = resp.error {
            return Err(eyre!("RPC error {}: {}", e.code, e.message));
        }

        resp.result
            .ok_or_else(|| eyre!("transaction not found: {}", self.signature))
    }
}
