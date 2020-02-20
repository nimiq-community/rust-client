use jsonrpc::error::Error;

use super::primitives;

pub struct Client {
	agent: jsonrpc::client::Client
}

impl Client {
	pub fn new(host: &str) -> Client {
		Client { agent: jsonrpc::client::Client::new(host.to_owned(), None, None) }
	}

	pub fn new_with_credentials(host: &str, username: &str, password: &str) -> Client {
		Client { agent: jsonrpc::client::Client::new(host.to_owned(), Some(username.to_owned()), Some(password.to_owned())) }
	}

	/// Returns a list of addresses owned by client.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Vector of accounts owned by the client.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.accounts();
	/// ```
	pub fn accounts(&self) -> Result<Vec<primitives::Account>, Error>{
		let request = self.agent.build_request("accounts", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<Vec<primitives::Account>>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the height of most recent block.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// The current block height the client is on.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.block_number();
	/// ```
	pub fn block_number(&self) -> Result<u64, Error>{
		let request = self.agent.build_request("blockNumber", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u64>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns information on the current consensus state.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// String describing the consensus state. `"established"` is the value for a good state, other values indicate bad state.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.consensus();
	/// ```
	pub fn consensus(&self) -> Result<String, Error>{
		let request = self.agent.build_request("consensus", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<String>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Creates a new account and stores its private key in the client store.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Information on the wallet that was created using the command.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.create_account();
	/// ```
	pub fn create_account(&self) -> Result<primitives::Wallet, Error>{
		let request = self.agent.build_request("createAccount", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Wallet>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Creates and signs a transaction without sending it. The transaction can then be send via `sendRawTransaction` without accidentally replaying it.
	///
	/// # Arguments
	///
	/// * `raw_transaction`: `&primitives::OutgoingTransaction`
	/// 
	/// # Returns
	/// 
	/// The Hex-encoded transaction.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let tx = nimiq_rpc::primitives::OutgoingTransaction {
	///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42",
	///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3",
	///    value: 100, //Lunas
	///    fee: 0
	///	};
	/// let result = client.create_raw_transaction(&tx);
	/// ```
	pub fn create_raw_transaction(&self, raw_transaction: &primitives::OutgoingTransaction) -> Result<String, Error>{
		let params = &[
			serde_json::to_value(&raw_transaction).unwrap()
		];
		let request = self.agent.build_request("createRawTransaction", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<String>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns details for the account of given address.
	///
	/// # Arguments
	///
	/// * `String`: Address to check for balance.
	/// 
	/// # Returns
	/// 
	/// Details about the account. Returns the default empty basic account for non-existing accounts.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_account("ad25610feb43d75307763d3f010822a757027429");
	/// ```
	pub fn get_account(&self, id: &str) -> Result<primitives::Account, Error>{
		let params = &[
			serde_json::to_value(id).unwrap()
		];
		let request = self.agent.build_request("getAccount", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Account>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the balance of the account of given address.
	///
	/// # Arguments
	///
	/// * `String`: Address to check for balance.
	/// 
	/// # Returns
	/// 
	/// Details about the account. The current balance at the specified address (in smalest unit).
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_balance("ad25610feb43d75307763d3f010822a757027429");
	/// ```
	pub fn get_balance(&self, id: &str) -> Result<u64, Error>{
		let params = &[
			serde_json::to_value(id).unwrap()
		];
		let request = self.agent.build_request("getBalance", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u64>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns information about a block by hash.
	///
	/// # Arguments
	///
	/// * `String`: Hash of the block to gather information on.
	/// * `Boolean`: If `true` it returns the full transaction objects, if `false` only the hashes of the transactions.
	/// 
	/// # Returns
	/// 
	/// A block object or `null` when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_by_hash("14c91f6d6f3a0b62271e546bb09461231ab7e4d1ddc2c3e1b93de52d48a1da87", false);
	/// ```
	pub fn get_block_by_hash(&self, block_hash: &str, full_transactions: bool) -> Result<primitives::Block, Error>{
		let params = &[
			serde_json::to_value(block_hash).unwrap(),
			serde_json::to_value(full_transactions).unwrap()
		];
		let request = self.agent.build_request("getBlockByHash", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Block>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns information about a block by block number.
	///
	/// # Arguments
	///
	/// * `Int`: The height of the block to gather information on.
	/// * `Boolean`: If `true` it returns the full transaction objects, if `false` only the hashes of the transactions.
	/// 
	/// # Returns
	/// 
	/// A block object or `null` when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_by_number(1234, false);
	/// ```
	pub fn get_block_by_number(&self, block_number: u64, full_transactions: bool) -> Result<primitives::Block, Error>{
		let params = &[
			serde_json::to_value(block_number).unwrap(),
			serde_json::to_value(full_transactions).unwrap()
		];
		let request = self.agent.build_request("getBlockByNumber", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Block>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns a template to build the next block for mining. This will consider pool instructions when connected to a pool.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// A block template object.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_template();
	/// ```
	pub fn get_block_template(&self) -> Result<primitives::FullBlock, Error>{
		let request = self.agent.build_request("getBlockTemplate", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::FullBlock>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the number of transactions in a block from a block matching the given block hash.
	///
	/// # Arguments
	///
	/// * `String`: Hash of the block.
	/// 
	/// # Returns
	/// 
	/// Number of transactions in the block found, or `null`, when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_transaction_count_by_hash("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f");
	/// ```
	pub fn get_block_transaction_count_by_hash(&self, block_hash: &str) -> Result<u16, Error>{
		let params = &[
			serde_json::to_value(block_hash).unwrap()
		];
		let request = self.agent.build_request("getBlockTransactionCountByHash", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u16>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the number of transactions in a block matching the given block number.
	///
	/// # Arguments
	///
	/// * `Int`: Height of the block.
	/// 
	/// # Returns
	/// 
	/// Number of transactions in the block found, or `null`, when no block was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_block_transaction_count_by_number(76415);
	/// ```
	pub fn get_block_transaction_count_by_number(&self, block_number: u64) -> Result<u16, Error>{
		let params = &[
			serde_json::to_value(block_number).unwrap()
		];
		let request = self.agent.build_request("getBlockTransactionCountByNumber", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u16>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns information about a transaction by block hash and transaction index position.
	///
	/// # Arguments
	///
	/// * `String`: Hash of the block containing the transaction
	/// * `Int`: Index of the transaction in the block
	/// 
	/// # Returns
	/// 
	/// A transaction object or `null` when no transaction was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_by_block_hash_and_index("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f", 20);
	/// ```
	pub fn get_transaction_by_block_hash_and_index(&self, block_hash: &str, index: u64) -> Result<primitives::Transaction, Error>{
		let params = &[
			serde_json::to_value(block_hash).unwrap(),
			serde_json::to_value(index).unwrap()
		];
		let request = self.agent.build_request("getTransactionByBlockHashAndIndex", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Transaction>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns information about a transaction by block number and transaction index position.
	///
	/// # Arguments
	///
	/// * `Int`: Height of the block containing the transaction
	/// * `Int`: Index of the transaction in the block
	/// 
	/// # Returns
	/// 
	/// A transaction object or `null` when no transaction was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_by_block_number_and_index(76415, 20);
	/// ```
	pub fn get_transaction_by_block_number_and_index(&self, block_number: u64, index: u16) -> Result<primitives::Transaction, Error>{
		let params = &[
			serde_json::to_value(block_number).unwrap(),
			serde_json::to_value(index).unwrap()
		];
		let request = self.agent.build_request("getTransactionByBlockNumberAndIndex", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Transaction>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the information about a transaction requested by transaction hash.
	///
	/// # Arguments
	///
	/// * `String`: Hash of a transaction
	/// 
	/// # Returns
	/// 
	/// A transaction object or `null` when no transaction was found.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_by_hash("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554");
	/// ```
	pub fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<primitives::Transaction, Error>{
		let params = &[
			serde_json::to_value(transaction_hash).unwrap()
		];
		let request = self.agent.build_request("getTransactionByHash", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Transaction>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the receipt of a transaction by transaction hash.
	/// `Note` That the receipt is not available for pending transactions.
	///
	/// # Arguments
	///
	/// * `String`: Hash of a transaction
	/// 
	/// # Returns
	/// 
	/// A transaction receipt object, or `null` when no receipt was found
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transaction_receipt("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554");
	/// ```
	pub fn get_transaction_receipt(&self, transaction_hash: &str) -> Result<primitives::TransactionReceipt, Error>{
		let params = &[
			serde_json::to_value(transaction_hash).unwrap()
		];
		let request = self.agent.build_request("getTransactionReceipt", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::TransactionReceipt>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the latest transactions successfully performed by or for an address.
	/// `Note` That this information might change when blocks are rewinded on the local state due to forks.
	///
	/// # Arguments
	///
	/// * `String`: Address of which transactions should be gathered.
	/// * `Int`: Number of transactions that shall be returned.
	/// 
	/// # Returns
	/// 
	/// Vector of transactions linked to the requested address.
	/// `Note` The array will not contain more than the requested amount of transactions, but might contain less, even when more transactions happened. Any interpretation of the length of this array might result in worng assumptions.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_transactions_by_address("NQ69 9A4A MB83 HXDQ 4J46 BH5R 4JFF QMA9 C3GN", 10);
	/// ```
	pub fn get_transactions_by_address(&self, address: &str, amount: u16) -> Result<Vec<primitives::Transaction>, Error>{
		let params = &[
			serde_json::to_value(address).unwrap(),
			serde_json::to_value(amount).unwrap()
		];
		let request = self.agent.build_request("getTransactionsByAddress", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<Vec<primitives::Transaction>>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns instructions to mine the next block. This will consider pool instructions when connected to a pool.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Mining work instructions
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.get_work();
	/// ```
	pub fn get_work(&self) -> Result<primitives::GetWork, Error>{
		let request = self.agent.build_request("getWork", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::GetWork>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns the number of hashes per second that the node is mining with.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Number of hashes per second.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.hashrate();
	/// ```
	pub fn hashrate(&self) -> Result<f64, Error>{
		let request = self.agent.build_request("hashrate", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<f64>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Sets the log level of the node.
	///
	/// # Arguments
	///
	/// * `String`: Tag: If `'*'` the log level is set globally, otherwise the log level is applied only on this tag.
	/// * `String`: Minimum log level to display. (Valid options: `trace`, `verbose`, `debug`, `info`, `warn`, `error`, `assert`)
	/// 
	/// # Returns
	/// 
	/// `true`
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.log("*", "log");
	/// ```
	pub fn log(&self, tag: &str, level: &str) -> Result<bool, Error>{
		let params = &[
			serde_json::to_value(tag).unwrap(),
			serde_json::to_value(level).unwrap()
		];
		let request = self.agent.build_request("log", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<bool>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn mempool_content(&self) -> Result<Vec<String>, Error>{
		let request = self.agent.build_request("mempoolContent", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<Vec<String>>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn miner_address(&self) -> Result<String, Error>{
		let request = self.agent.build_request("minerAddress", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<String>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn miner_threads(&self) -> Result<u8, Error>{
		let request = self.agent.build_request("minerThreads", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u8>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn miner_threads_with_update(&self, threads: u16) -> Result<u16, Error>{
		let params = &[
			serde_json::to_value(threads).unwrap()
		];
		let request = self.agent.build_request("minerThreads", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u16>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn min_fee_per_byte(&self) -> Result<u32, Error>{
		let request = self.agent.build_request("minFeePerByte", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u32>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn min_fee_per_byte_with_update(&self, fee: u32) -> Result<u32, Error>{
		let params = &[
			serde_json::to_value(fee).unwrap()
		];
		let request = self.agent.build_request("minFeePerByte", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u32>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns `true` if client is actively mining new blocks.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Returns `true` if the client is mining, otherwise `false`.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.mining();
	/// ```
	pub fn mining(&self) -> Result<bool, Error>{
		let request = self.agent.build_request("mining", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<bool>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns number of peers currently connected to the client.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// Integer of the number of connected peers.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.peer_count();
	/// ```
	pub fn peer_count(&self) -> Result<i8, Error>{
		let request = self.agent.build_request("peerCount", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<i8>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn peer_list(&self) -> Result<Vec<primitives::PeerList>, Error>{
		let request = self.agent.build_request("peerList", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<Vec<primitives::PeerList>>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn peer_state(&self, peer_address: &str) -> Result<primitives::PeerState, Error>{
		let params = &[
			serde_json::to_value(peer_address).unwrap()
		];
		let request = self.agent.build_request("peerState", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::PeerState>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn peer_state_with_update(&self, peer_address: &str, set: &str) -> Result<primitives::PeerState, Error>{
		let params = &[
			serde_json::to_value(peer_address).unwrap(),
			serde_json::to_value(set).unwrap()
		];
		let request = self.agent.build_request("peerState", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::PeerState>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn pool_confirmed_balance(&self) -> Result<u64, Error>{
		let request = self.agent.build_request("poolConfirmedBalance", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u64>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	pub fn pool_connection_state(&self) -> Result<u8, Error>{
		let request = self.agent.build_request("poolConnectionState", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<u8>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Sends a signed message call transaction or a contract creation, if the data field contains code.
	///
	/// # Arguments
	///
	/// * `String`: The hex encoded signed transaction
	/// 
	/// # Returns
	/// 
	/// The Hex-encoded transaction hash.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let tx = nimiq_rpc::primitives::OutgoingTransaction {
	///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42",
	///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3",
	///    value: 100, //Lunas
	///    fee: 0
	///	};
	/// let result = client.create_raw_transaction(&tx);
	/// let hash = client.send_raw_transaction(&result);
	/// ```
	pub fn send_raw_transaction(&self, transaction_hash: &str) -> Result<String, Error>{
		let params = &[
			serde_json::to_value(transaction_hash).unwrap()
		];
		let request = self.agent.build_request("sendRawTransaction", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<String>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Creates new message call transaction or a contract creation, if the data field contains code.
	///
	/// # Arguments
	///
	/// * `OutgoingTransaction`: The transaction object
	/// 
	/// # Returns
	/// 
	/// The Hex-encoded transaction hash.
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let tx = nimiq_rpc::primitives::OutgoingTransaction {
	///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42",
	///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3",
	///    value: 100, //Lunas
	///    fee: 0
	///	};
	/// let result = client.send_transaction(&tx);
	/// ```
	pub fn send_transaction(&self, transaction: &primitives::OutgoingTransaction) -> Result<String, Error>{
		let params = &[
			serde_json::to_value(transaction).unwrap()
		];
		let request = self.agent.build_request("sendTransaction", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<String>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Submits a block to the node. When the block is valid, the node will forward it to other nodes in the network.
	///
	/// # Arguments
	///
	/// * `String`: Hex-encoded full block (including header, interlink and body). When submitting work from `getWork`, remember to include the `suffix`.
	/// 
	/// # Returns
	/// 
	/// Nothing
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.submit_block("0da1....234");
	/// ```
	pub fn submit_block(&self, full_block: &str) -> Result<(), Error>{
		let params = &[
			serde_json::to_value(full_block).unwrap()
		];
		let request = self.agent.build_request("submitBlock", params);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<()>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}

	/// Returns an object with data about the sync status or `false`.
	///
	/// # Arguments
	///
	/// * `none`
	/// 
	/// # Returns
	/// 
	/// An object with sync status data or `false`, when not syncing
	///
	/// # Example
	///
	/// ```
	/// use nimiq_rpc::Client;
	/// let client = Client::new("http://seed-host.com:8648");
	/// let result = client.syncing();
	/// ```
	pub fn syncing(&self) -> Result<primitives::Syncing, Error>{
		let request = self.agent.build_request("syncing", &[]);
    match self.agent.send_request(&request).and_then(|res| res.into_result::<primitives::Syncing>()) {
			Ok(r) => Ok(r),
			Err(e) => Err(e)
    }
	}
}