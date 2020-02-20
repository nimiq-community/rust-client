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

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FullBlock {
    pub header: Header,
    pub interlink: String,
    pub target: u64,
    pub body: Body
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Header {
    pub version: u64,
    #[serde(rename = "prevHash")]
    pub prev_hash: String,
    #[serde(rename = "interlinkHash")]
    pub interlink_hash: String,
    #[serde(rename = "accountsHash")]
    pub accounts_hash: String,
    #[serde(rename = "nBits")]
    pub n_bits: u64,
    pub height: u64
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Body {
    hash: String,
    #[serde(rename = "minerAddr")]
    miner_addr: String,
    #[serde(rename = "extraData")]
    extra_data: String,
    transactions: Vec<String>,
    #[serde(rename = "merkleHashes")]
    merkle_hashes: Vec<String>,
    #[serde(rename = "prunedAccounts")]
    pruned_accounts: Vec<String>,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GetWork {
	pub data: String,
	pub suffix: String,
	pub target: u64,
	pub algorithm: String
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PeerList {
	pub id: String,
	pub address: String,
	#[serde(rename = "addressState")]
	pub address_state: i64,
	#[serde(rename = "connectionState")]
	pub connection_state: Option<i64>,
	pub version: Option<i64>,
	#[serde(rename = "timeOffset")]
	pub time_offset: Option<i64>,
	#[serde(rename = "headHash")]
	pub head_hash: Option<String>,
	pub latency: Option<i64>,
	pub rx: Option<i64>,
	pub tx: Option<i64>
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PeerState {
	pub id: String,
	pub address: String,
	#[serde(rename = "addressState")]
	pub address_state: u8
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Syncing {
	IsSyncing(bool),
	Pending(Pending)
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Pending {
	#[serde(rename = "startingBlock")]
	starting_block: u64,
	#[serde(rename = "currentBlock")]
	current_block: u64,
	#[serde(rename = "highestBlock")]
	highest_block: u64
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transaction {
	pub hash: String,
	#[serde(rename = "blockHash")]
	pub block_hash: String,
	#[serde(rename = "blockNumber")]
	pub block_number: u64,
	pub timestamp: u64,
	pub confirmations: u64,
	#[serde(rename = "transactionIndex")]
	#[serde(skip_deserializing)] //TODO skip deserializing, have to look into making field optional
	transaction_index: ::serde_json::Value,
	pub from: String,
	#[serde(rename = "fromAddress")]
	pub from_address: String,
	pub to: String,
	#[serde(rename = "toAddress")]
	pub to_address: String,
	pub value: u64,
	pub fee: u64,
	pub data: ::serde_json::Value,
	pub flags: u32
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TransactionReceipt {
	#[serde(rename = "transactionHash")]
	pub transaction_hash: String,
	#[serde(rename = "transactionIndex")]
	pub transaction_index: i64,
	#[serde(rename = "blockNumber")]
	pub block_number: i64,
	#[serde(rename = "blockHash")]
	pub block_hash: String,
	pub confirmations: i64,
	pub timestamp: i64
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