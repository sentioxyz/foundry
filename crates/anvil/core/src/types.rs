use std::collections::HashMap;
use alloy_eips::BlockId;
use alloy_primitives::{Bytes, B256};
use alloy_rpc_types::{BlockOverrides, TransactionIndex, TransactionRequest};
use alloy_rpc_types::trace::geth::GethDebugTracingCallOptions;
use alloy_serde::WithOtherFields;
use serde::{Deserialize, Serialize};

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
#[expect(clippy::large_enum_variant)]
pub enum TransactionData {
    JSON(TransactionRequest),
    Raw(Bytes),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceCallManyBundle {
    pub transactions: Vec<WithOtherFields<TransactionRequest>>,
    pub block_override: Option<BlockOverrides>,
    #[serde(default)]
    pub tracer_start_index: usize
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugTraceTransactionOpts {
    #[serde(flatten)]
    pub tracing_call_options: GethDebugTracingCallOptions,
    #[serde(default)]
    pub force_replay: bool,
    #[serde(default)]
    pub force_replay_preceding: bool,
    #[serde(default)]
    pub force_replay_validation: bool,
}