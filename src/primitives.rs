use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize)]
pub struct Address {
	pub id: String,
	pub address: String
}

#[derive(Debug, Deserialize)]
pub struct Account {
	pub id: String,
	pub address: String,
	pub balance: u64,
	pub r#type: u8
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
	pub number: u64,
	pub hash: String,
	pub pow: String,
	pub parent_hash: String,
	pub nonce: u64,
	pub body_hash: String,
	pub accounts_hash: String,
	pub miner: String,
	pub miner_address: String,
	pub difficulty: String,
	pub extra_data: String,
	pub size: u32,
	pub timestamp: u64,
	pub transactions: TransactionSequence
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBlock {
    pub header: Header,
    pub interlink: String,
    pub target: u64,
    pub body: Body
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub version: u64,
    pub prev_hash: String,
    pub interlink_hash: String,
    pub accounts_hash: String,
    pub n_bits: u64,
    pub height: u64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    hash: String,
    miner_addr: String,
    extra_data: String,
    transactions: Vec<String>,
    merkle_hashes: Vec<String>,
    pruned_accounts: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWork {
	pub data: String,
	pub suffix: String,
	pub target: u64,
	pub algorithm: String
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
	pub tx: Option<u64>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerState {
	pub id: String,
	pub address: String,
	pub address_state: u8
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Syncing {
	IsSyncing(bool),
	Pending(Pending)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pending {
	starting_block: u64,
	current_block: u64,
	highest_block: u64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
	pub hash: String,
	pub block_hash: String,
	pub block_number: u64,
	pub timestamp: u64,
	pub confirmations: u64,
	pub transaction_index: Option<i64>,
	pub from: String,
	pub from_address: String,
	pub to: String,
	pub to_address: String,
	pub value: u64,
	pub fee: u64,
	pub data: Option<String>,
	pub flags: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
	pub transaction_hash: String,
	pub transaction_index: i64,
	pub block_number: u64,
	pub block_hash: String,
	pub confirmations: u64,
	pub timestamp: u64
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum TransactionSequence{
	BlockHashes(Vec<String>),
	Transactions(Vec<Transaction>)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutgoingTransaction {
	pub from: &'static str,
	pub to: &'static str,
	pub value: u64,
	pub fee: u32
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
	pub id: String,
	pub address: String,
	pub public_key: String
}