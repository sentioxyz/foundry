use std::collections::HashMap;
use alloy_eips::BlockId;
use alloy_primitives::{Bytes, B256, U256};

use alloy_rpc_types::{BlockOverrides, TransactionIndex, TransactionRequest};
use alloy_serde::WithOtherFields;
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde::Serializer;

/// Represents the result of `eth_getWork`
/// This may or may not include the block number
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Work {
    pub pow_hash: B256,
    pub seed_hash: B256,
    pub target: B256,
    pub number: Option<u64>,
}

#[cfg(feature = "serde")]
impl serde::Serialize for Work {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(num) = self.number {
            (&self.pow_hash, &self.seed_hash, &self.target, U256::from(num)).serialize(s)
        } else {
            (&self.pow_hash, &self.seed_hash, &self.target).serialize(s)
        }
    }
}

/// Represents the options used in `anvil_reorg`
#[derive(Debug, Clone, Deserialize)]
pub struct ReorgOptions {
    // The depth of the reorg
    pub depth: u64,
    // List of transaction requests and blocks pairs to be mined into the new chain
    pub tx_block_pairs: Vec<(TransactionData, u64)>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum TransactionData {
    JSON(TransactionRequest),
    Raw(Bytes),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceCallManyBundle {
    pub transactions: Vec<WithOtherFields<TransactionRequest>>,
    pub block_override: Option<BlockOverrides>
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceCallManyContext {
    pub block_number: Option<BlockId>,
    #[serde(default)]
    pub transaction_index: TransactionIndex
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRangeAtResult {
    pub storage: StorageMap,
    pub next_key: Option<B256>,
}

pub type StorageMap = HashMap<B256, StorageEntry>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageEntry {
    pub key: B256,
    pub value: B256
}