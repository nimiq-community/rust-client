use jsonrpc::client::Client as RpcClient;
use jsonrpc::error::Error;

use crate::primitives::*;

pub struct Client {
    agent: RpcClient,
}

impl Client {
    pub fn new(host: String) -> Client {
        Client {
            agent: RpcClient::new(host, None, None),
        }
    }

    pub fn new_with_credentials(host: String, username: String, password: String) -> Client {
        Client {
            agent: RpcClient::new(host, Some(username), Some(password)),
        }
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.accounts();
    /// ```
    pub fn accounts(&self) -> Result<Vec<Account>, Error> {
        self.agent
            .send_request(&self.agent.build_request("accounts", &[]))
            .and_then(|res| res.into_result::<Vec<Account>>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.block_number();
    /// ```
    pub fn block_number(&self) -> Result<u32, Error> {
        self.agent
            .send_request(&self.agent.build_request("blockNumber", &[]))
            .and_then(|res| res.into_result::<u32>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.consensus();
    /// ```
    pub fn consensus(&self) -> Result<String, Error> {
        self.agent
            .send_request(&self.agent.build_request("consensus", &[]))
            .and_then(|res| res.into_result::<String>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.create_account();
    /// ```
    pub fn create_account(&self) -> Result<Wallet, Error> {
        self.agent
            .send_request(&self.agent.build_request("createAccount", &[]))
            .and_then(|res| res.into_result::<Wallet>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let tx = nimiq_rpc::primitives::OutgoingTransaction {
    ///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42".to_string(),
    ///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3".to_string(),
    ///    value: 100, //Lunas
    ///    fee: 0
    /// };
    /// let result = client.create_raw_transaction(&tx);
    /// ```
    pub fn create_raw_transaction(
        &self,
        raw_transaction: &OutgoingTransaction,
    ) -> Result<String, Error> {
        let params = &[serde_json::to_value(raw_transaction)?];
        self.agent
            .send_request(&self.agent.build_request("createRawTransaction", params))
            .and_then(|res| res.into_result::<String>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_account("ad25610feb43d75307763d3f010822a757027429");
    /// ```
    pub fn get_account(&self, id: &str) -> Result<Account, Error> {
        let params = &[serde_json::to_value(id)?];
        self.agent
            .send_request(&self.agent.build_request("getAccount", params))
            .and_then(|res| res.into_result::<Account>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_balance("ad25610feb43d75307763d3f010822a757027429");
    /// ```
    pub fn get_balance(&self, id: &str) -> Result<u64, Error> {
        let params = &[serde_json::to_value(id)?];
        self.agent
            .send_request(&self.agent.build_request("getBalance", params))
            .and_then(|res| res.into_result::<u64>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_block_by_hash("14c91f6d6f3a0b62271e546bb09461231ab7e4d1ddc2c3e1b93de52d48a1da87", false);
    /// ```
    pub fn get_block_by_hash(
        &self,
        block_hash: &str,
        full_transactions: bool,
    ) -> Result<Block, Error> {
        let params = &[
            serde_json::to_value(block_hash)?,
            serde_json::to_value(full_transactions)?,
        ];
        self.agent
            .send_request(&self.agent.build_request("getBlockByHash", params))
            .and_then(|res| res.into_result::<Block>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_block_by_number(1234, false);
    /// ```
    pub fn get_block_by_number(
        &self,
        block_number: u32,
        full_transactions: bool,
    ) -> Result<Block, Error> {
        let params = &[
            serde_json::to_value(block_number)?,
            serde_json::to_value(full_transactions)?,
        ];
        self.agent
            .send_request(&self.agent.build_request("getBlockByNumber", params))
            .and_then(|res| res.into_result::<Block>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_block_template();
    /// ```
    pub fn get_block_template(&self) -> Result<FullBlock, Error> {
        self.agent
            .send_request(&self.agent.build_request("getBlockTemplate", &[]))
            .and_then(|res| res.into_result::<FullBlock>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_block_transaction_count_by_hash("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f");
    /// ```
    pub fn get_block_transaction_count_by_hash(&self, block_hash: &str) -> Result<u16, Error> {
        let params = &[serde_json::to_value(block_hash)?];
        self.agent
            .send_request(
                &self
                    .agent
                    .build_request("getBlockTransactionCountByHash", params),
            )
            .and_then(|res| res.into_result::<u16>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_block_transaction_count_by_number(76415);
    /// ```
    pub fn get_block_transaction_count_by_number(&self, block_number: u32) -> Result<u16, Error> {
        let params = &[serde_json::to_value(block_number)?];
        self.agent
            .send_request(
                &self
                    .agent
                    .build_request("getBlockTransactionCountByNumber", params),
            )
            .and_then(|res| res.into_result::<u16>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_transaction_by_block_hash_and_index("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f", 20);
    /// ```
    pub fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: &str,
        index: u16,
    ) -> Result<Transaction, Error> {
        let params = &[
            serde_json::to_value(block_hash)?,
            serde_json::to_value(index)?,
        ];
        self.agent
            .send_request(
                &self
                    .agent
                    .build_request("getTransactionByBlockHashAndIndex", params),
            )
            .and_then(|res| res.into_result::<Transaction>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_transaction_by_block_number_and_index(76415, 20);
    /// ```
    pub fn get_transaction_by_block_number_and_index(
        &self,
        block_number: u32,
        index: u16,
    ) -> Result<Transaction, Error> {
        let params = &[
            serde_json::to_value(block_number)?,
            serde_json::to_value(index)?,
        ];
        self.agent
            .send_request(
                &self
                    .agent
                    .build_request("getTransactionByBlockNumberAndIndex", params),
            )
            .and_then(|res| res.into_result::<Transaction>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_transaction_by_hash("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554");
    /// ```
    pub fn get_transaction_by_hash(&self, transaction_hash: &str) -> Result<Transaction, Error> {
        let params = &[serde_json::to_value(transaction_hash)?];
        self.agent
            .send_request(&self.agent.build_request("getTransactionByHash", params))
            .and_then(|res| res.into_result::<Transaction>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_transaction_receipt("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554");
    /// ```
    pub fn get_transaction_receipt(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionReceipt, Error> {
        let params = &[serde_json::to_value(transaction_hash)?];
        self.agent
            .send_request(&self.agent.build_request("getTransactionReceipt", params))
            .and_then(|res| res.into_result::<TransactionReceipt>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_transactions_by_address("NQ69 9A4A MB83 HXDQ 4J46 BH5R 4JFF QMA9 C3GN", 10);
    /// ```
    pub fn get_transactions_by_address(
        &self,
        address: &str,
        amount: u16,
    ) -> Result<Vec<Transaction>, Error> {
        let params = &[
            serde_json::to_value(address)?,
            serde_json::to_value(amount)?,
        ];
        self.agent
            .send_request(&self.agent.build_request("getTransactionsByAddress", params))
            .and_then(|res| res.into_result::<Vec<Transaction>>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.get_work();
    /// ```
    pub fn get_work(&self) -> Result<GetWork, Error> {
        self.agent
            .send_request(&self.agent.build_request("getWork", &[]))
            .and_then(|res| res.into_result::<GetWork>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.hashrate();
    /// ```
    pub fn hashrate(&self) -> Result<f64, Error> {
        self.agent
            .send_request(&self.agent.build_request("hashrate", &[]))
            .and_then(|res| res.into_result::<f64>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.log("*", "log");
    /// ```
    pub fn log(&self, tag: &str, level: &str) -> Result<bool, Error> {
        let params = &[serde_json::to_value(tag)?, serde_json::to_value(level)?];
        self.agent
            .send_request(&self.agent.build_request("log", params))
            .and_then(|res| res.into_result::<bool>())
    }

    pub fn mempool_content(&self) -> Result<Vec<String>, Error> {
        self.agent
            .send_request(&self.agent.build_request("mempoolContent", &[]))
            .and_then(|res| res.into_result::<Vec<String>>())
    }

    pub fn miner_address(&self) -> Result<String, Error> {
        self.agent
            .send_request(&self.agent.build_request("minerAddress", &[]))
            .and_then(|res| res.into_result::<String>())
    }

    pub fn miner_threads(&self) -> Result<u8, Error> {
        self.agent
            .send_request(&self.agent.build_request("minerThreads", &[]))
            .and_then(|res| res.into_result::<u8>())
    }

    pub fn miner_threads_with_update(&self, threads: u16) -> Result<u16, Error> {
        let params = &[serde_json::to_value(threads)?];
        self.agent
            .send_request(&self.agent.build_request("minerThreads", params))
            .and_then(|res| res.into_result::<u16>())
    }

    pub fn min_fee_per_byte(&self) -> Result<u32, Error> {
        self.agent
            .send_request(&self.agent.build_request("minFeePerByte", &[]))
            .and_then(|res| res.into_result::<u32>())
    }

    pub fn min_fee_per_byte_with_update(&self, fee: u32) -> Result<u32, Error> {
        let params = &[serde_json::to_value(fee)?];
        self.agent
            .send_request(&self.agent.build_request("minFeePerByte", params))
            .and_then(|res| res.into_result::<u32>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.mining();
    /// ```
    pub fn mining(&self) -> Result<bool, Error> {
        self.agent
            .send_request(&self.agent.build_request("mining", &[]))
            .and_then(|res| res.into_result::<bool>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.peer_count();
    /// ```
    pub fn peer_count(&self) -> Result<i8, Error> {
        self.agent
            .send_request(&self.agent.build_request("peerCount", &[]))
            .and_then(|res| res.into_result::<i8>())
    }

    pub fn peer_list(&self) -> Result<Vec<PeerList>, Error> {
        self.agent
            .send_request(&self.agent.build_request("peerList", &[]))
            .and_then(|res| res.into_result::<Vec<PeerList>>())
    }

    pub fn peer_state(&self, peer_address: &str) -> Result<PeerState, Error> {
        let params = &[serde_json::to_value(peer_address)?];
        self.agent
            .send_request(&self.agent.build_request("peerState", params))
            .and_then(|res| res.into_result::<PeerState>())
    }

    pub fn peer_state_with_update(
        &self,
        peer_address: &str,
        set: &str,
    ) -> Result<PeerState, Error> {
        let params = &[
            serde_json::to_value(peer_address)?,
            serde_json::to_value(set)?,
        ];
        self.agent
            .send_request(&self.agent.build_request("peerState", params))
            .and_then(|res| res.into_result::<PeerState>())
    }

    pub fn pool_confirmed_balance(&self) -> Result<u64, Error> {
        self.agent
            .send_request(&self.agent.build_request("poolConfirmedBalance", &[]))
            .and_then(|res| res.into_result::<u64>())
    }

    pub fn pool_connection_state(&self) -> Result<u8, Error> {
        self.agent
            .send_request(&self.agent.build_request("poolConnectionState", &[]))
            .and_then(|res| res.into_result::<u8>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let tx = nimiq_rpc::primitives::OutgoingTransaction {
    ///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42".to_string(),
    ///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3".to_string(),
    ///    value: 100, //Lunas
    ///    fee: 0
    /// };
    /// let result = client.create_raw_transaction(&tx);
    /// let hash = client.send_raw_transaction(&result);
    /// ```
    pub fn send_raw_transaction(&self, transaction_hash: &str) -> Result<String, Error> {
        let params = &[serde_json::to_value(transaction_hash)?];
        self.agent
            .send_request(&self.agent.build_request("sendRawTransaction", params))
            .and_then(|res| res.into_result::<String>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let tx = nimiq_rpc::primitives::OutgoingTransaction {
    ///    from: "NQ32 R6DB VFM5 M931 7X4E 0N5Q LJ56 9QCR 4T42".to_string(),
    ///    to: "NQ74 61S8 2FD3 RVPG HU09 1Y57 77E6 BL38 TQH3".to_string(),
    ///    value: 100, //Lunas
    ///    fee: 0
    /// };
    /// let result = client.send_transaction(&tx);
    /// ```
    pub fn send_transaction(&self, transaction: &OutgoingTransaction) -> Result<String, Error> {
        let params = &[serde_json::to_value(transaction)?];
        self.agent
            .send_request(&self.agent.build_request("sendTransaction", params))
            .and_then(|res| res.into_result::<String>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.submit_block("0da1....234");
    /// ```
    pub fn submit_block(&self, full_block: &str) -> Result<(), Error> {
        let params = &[serde_json::to_value(full_block)?];
        self.agent
            .send_request(&self.agent.build_request("submitBlock", params))
            .and_then(|res| res.into_result::<()>())
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
    /// let client = Client::new("http://seed-host.com:8648".to_string());
    /// let result = client.syncing();
    /// ```
    pub fn syncing(&self) -> Result<Syncing, Error> {
        self.agent
            .send_request(&self.agent.build_request("syncing", &[]))
            .and_then(|res| res.into_result::<Syncing>())
    }
}
