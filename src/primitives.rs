use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Address {
    pub id: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Account {
    Basic(BasicAccount),
    Vesting(VestingAccount),
    HTLC(HTLCAccount),
}

#[derive(Debug, Deserialize)]
pub struct BasicAccount {
    pub id: String,
    pub address: String,
    pub balance: u64,
    pub r#type: u8,
}

#[derive(Debug, Deserialize)]
pub struct VestingAccount {
    pub id: String,
    pub address: String,
    pub balance: u64,
    pub r#type: u8,
    pub owner: String,
    pub owner_address: String,
    pub vesting_start: u64,
    pub vesting_step_blocks: u64,
    pub vesting_step_amount: u64,
    pub vesting_total_amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HTLCAccount {
    pub id: String,
    pub address: String,
    pub balance: u64,
    pub r#type: u8,
    pub sender: String,
    pub sender_address: String,
    pub recipient: String,
    pub recipient_address: String,
    pub hash_root: String,
    pub hash_count: u64,
    pub timeout: u64,
    pub total_amount: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub number: u32,
    pub hash: String,
    pub pow: String,
    pub parent_hash: String,
    pub nonce: u32,
    pub body_hash: String,
    pub accounts_hash: String,
    pub miner: String,
    pub miner_address: String,
    pub difficulty: String,
    pub extra_data: String,
    pub size: u32,
    pub timestamp: u32,
    pub transactions: TransactionSequence,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBlock {
    pub header: Header,
    pub interlink: String,
    pub target: u64,
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub version: u16,
    pub prev_hash: String,
    pub interlink_hash: String,
    pub accounts_hash: String,
    pub n_bits: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    hash: String,
    miner_addr: String,
    extra_data: String,
    transactions: Vec<String>,
    merkle_hashes: Vec<String>,
    pruned_accounts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWork {
    pub data: String,
    pub suffix: String,
    pub target: u64,
    pub algorithm: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerList {
    pub id: String,
    pub address: String,
    pub address_state: u64,
    pub connection_state: Option<u64>,
    pub version: Option<u64>,
    pub time_offset: Option<i64>,
    pub head_hash: Option<String>,
    pub latency: Option<u64>,
    pub rx: Option<u64>,
    pub tx: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerState {
    pub id: String,
    pub address: String,
    pub address_state: u8,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Syncing {
    IsSyncing(bool),
    Pending(Pending),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending {
    starting_block: u32,
    current_block: u32,
    highest_block: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub hash: String,
    pub block_hash: String,
    pub block_number: u32,
    pub timestamp: u32,
    pub confirmations: u32,
    pub transaction_index: Option<i32>,
    pub from: String,
    pub from_address: String,
    pub to: String,
    pub to_address: String,
    pub value: u64,
    pub fee: u64,
    pub data: Option<String>,
    pub flags: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    pub hash: String,
    pub block_hash: String,
    pub block_number: u32,
    pub timestamp: u32,
    pub confirmations: u32,
    pub from: String,
    pub from_address: String,
    pub to: String,
    pub to_address: String,
    pub value: u64,
    pub fee: u64,
    pub data: Option<String>,
    pub proof: Option<String>,
    pub flags: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub transaction_hash: String,
    pub transaction_index: i32,
    pub block_number: u32,
    pub block_hash: String,
    pub confirmations: u32,
    pub timestamp: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum TransactionSequence {
    BlockHashes(Vec<String>),
    Transactions(Vec<Transaction>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingTransaction {
    pub from: String,
    pub to: String,
    pub value: u64,
    pub fee: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub public_key: String,
}
