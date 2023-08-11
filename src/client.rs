use base64::Engine;
use jsonrpsee::{
    core::{client::ClientT, Error},
    rpc_params,
};
use jsonrpsee_http_client::{HeaderMap, HttpClient, HttpClientBuilder};

use url::Url;

use crate::primitives::*;

pub struct Client {
    agent: HttpClient,
}

impl Client {
    pub fn new(url: Url) -> Client {
        Client {
            agent: HttpClientBuilder::default().build(url).unwrap(),
        }
    }

    pub fn new_with_credentials(url: Url, username: String, password: String) -> Client {
        let mut s = username;
        s.push(':');
        s.push_str(&password);
        let auth = format!(
            "Basic {}",
            &*base64::prelude::BASE64_STANDARD.encode(s.as_bytes())
        );
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", auth.parse().unwrap());
        Client {
            agent: HttpClientBuilder::default()
                .set_headers(headers)
                .build(url)
                .unwrap(),
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
    pub async fn accounts(&self) -> Result<Vec<Account>, Error> {
        let params = rpc_params![];
        self.agent.request("accounts", params).await
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
    /// let result = client.block_number().await;
    /// ```
    pub async fn block_number(&self) -> Result<u32, Error> {
        let params = rpc_params![];
        self.agent.request("blockNumber", params).await
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
    /// let result = client.consensus().await;
    /// ```
    pub async fn consensus(&self) -> Result<String, Error> {
        let params = rpc_params![];
        self.agent.request("consensus", params).await
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
    /// let result = client.create_account().await;
    /// ```
    pub async fn create_account(&self) -> Result<Wallet, Error> {
        let params = rpc_params![];
        self.agent.request("createAccount", params).await
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
    /// let result = client.create_raw_transaction(&tx).await;
    /// ```
    pub async fn create_raw_transaction(
        &self,
        raw_transaction: &OutgoingTransaction,
    ) -> Result<String, Error> {
        let params = rpc_params![raw_transaction];
        self.agent.request("createRawTransaction", params).await
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
    /// let result = client.get_account("ad25610feb43d75307763d3f010822a757027429").await;
    /// ```
    pub async fn get_account(&self, id: &str) -> Result<Account, Error> {
        let params = rpc_params![id];
        self.agent.request("getAccount", params).await
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
    /// let result = client.get_balance("ad25610feb43d75307763d3f010822a757027429").await;
    /// ```
    pub async fn get_balance(&self, id: &str) -> Result<u64, Error> {
        let params = rpc_params![id];
        self.agent.request("getBalance", params).await
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
    /// let result = client.get_block_by_hash("14c91f6d6f3a0b62271e546bb09461231ab7e4d1ddc2c3e1b93de52d48a1da87", false).await;
    /// ```
    pub async fn get_block_by_hash(
        &self,
        block_hash: &str,
        full_transactions: bool,
    ) -> Result<Block, Error> {
        let params = rpc_params![block_hash, full_transactions];
        self.agent.request("getBlockByHash", params).await
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
    /// let result = client.get_block_by_number(1234, false).await;
    /// ```
    pub async fn get_block_by_number(
        &self,
        block_number: u32,
        full_transactions: bool,
    ) -> Result<Block, Error> {
        let params = rpc_params![block_number, full_transactions];
        self.agent.request("getBlockByNumber", params).await
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
    /// let result = client.get_block_template().await;
    /// ```
    pub async fn get_block_template(&self) -> Result<FullBlock, Error> {
        let params = rpc_params![];
        self.agent.request("getBlockTemplate", params).await
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
    /// let result = client.get_block_transaction_count_by_hash("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f").await;
    /// ```
    pub async fn get_block_transaction_count_by_hash(
        &self,
        block_hash: &str,
    ) -> Result<u16, Error> {
        let params = rpc_params![block_hash];
        self.agent
            .request("getBlockTransactionCountByHash", params)
            .await
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
    /// let result = client.get_block_transaction_count_by_number(76415).await;
    /// ```
    pub async fn get_block_transaction_count_by_number(
        &self,
        block_number: u32,
    ) -> Result<u16, Error> {
        let params = rpc_params![block_number];
        self.agent
            .request("getBlockTransactionCountByNumber", params)
            .await
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
    /// let result = client.get_transaction_by_block_hash_and_index("dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f", 20).await;
    /// ```
    pub async fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: &str,
        index: u16,
    ) -> Result<Transaction, Error> {
        let params = rpc_params![block_hash, index];
        self.agent
            .request("getTransactionByBlockHashAndIndex", params)
            .await
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
    /// let result = client.get_transaction_by_block_number_and_index(76415, 20).await;
    /// ```
    pub async fn get_transaction_by_block_number_and_index(
        &self,
        block_number: u32,
        index: u16,
    ) -> Result<Transaction, Error> {
        let params = rpc_params![block_number, index];
        self.agent
            .request("getTransactionByBlockNumberAndIndex", params)
            .await
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
    /// let result = client.get_transaction_by_hash("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554").await;
    /// ```
    pub async fn get_transaction_by_hash(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionDetails, Error> {
        let params = rpc_params![transaction_hash];
        self.agent.request("getTransactionByHash", params).await
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
    /// let result = client.get_transaction_receipt("465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554").await;
    /// ```
    pub async fn get_transaction_receipt(
        &self,
        transaction_hash: &str,
    ) -> Result<TransactionReceipt, Error> {
        let params = rpc_params![transaction_hash];
        self.agent.request("getTransactionReceipt", params).await
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
    /// let result = client.get_transactions_by_address("NQ69 9A4A MB83 HXDQ 4J46 BH5R 4JFF QMA9 C3GN", 10).await;
    /// ```
    pub async fn get_transactions_by_address(
        &self,
        address: &str,
        amount: u16,
    ) -> Result<Vec<TransactionDetails>, Error> {
        let params = rpc_params![address, amount];
        self.agent.request("getTransactionsByAddress", params).await
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
    /// let result = client.get_work().await;
    /// ```
    pub async fn get_work(&self) -> Result<GetWork, Error> {
        let params = rpc_params![];
        self.agent.request("getWork", params).await
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
    /// let result = client.hashrate().await;
    /// ```
    pub async fn hashrate(&self) -> Result<f64, Error> {
        let params = rpc_params![];
        self.agent.request("hashrate", params).await
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
    /// let result = client.log("*", "log").await;
    /// ```
    pub async fn log(&self, tag: &str, level: &str) -> Result<bool, Error> {
        let params = rpc_params![tag, level];
        self.agent.request("log", params).await
    }

    pub async fn mempool_content(&self) -> Result<Vec<String>, Error> {
        let params = rpc_params![];
        self.agent.request("mempoolContent", params).await
    }

    pub async fn miner_address(&self) -> Result<String, Error> {
        let params = rpc_params![];
        self.agent.request("minerAddress", params).await
    }

    pub async fn miner_threads(&self) -> Result<u8, Error> {
        let params = rpc_params![];
        self.agent.request("minerThreads", params).await
    }

    pub async fn miner_threads_with_update(&self, threads: u16) -> Result<u16, Error> {
        let params = rpc_params![threads];
        self.agent.request("minerThreads", params).await
    }

    pub async fn min_fee_per_byte(&self) -> Result<u32, Error> {
        let params = rpc_params![];
        self.agent.request("minFeePerByte", params).await
    }

    pub async fn min_fee_per_byte_with_update(&self, fee: u32) -> Result<u32, Error> {
        let params = rpc_params![fee];
        self.agent.request("minFeePerByte", params).await
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
    /// let result = client.mining().await;
    /// ```
    pub async fn mining(&self) -> Result<bool, Error> {
        let params = rpc_params![];
        self.agent.request("mining", params).await
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
    /// let result = client.peer_count().await;
    /// ```
    pub async fn peer_count(&self) -> Result<i8, Error> {
        let params = rpc_params![];
        self.agent.request("peerCount", params).await
    }

    pub async fn peer_list(&self) -> Result<Vec<PeerList>, Error> {
        let params = rpc_params![];
        self.agent.request("peerList", params).await
    }

    pub async fn peer_state(&self, peer_address: &str) -> Result<PeerState, Error> {
        let params = rpc_params![peer_address];
        self.agent.request("peerState", params).await
    }

    pub async fn peer_state_with_update(
        &self,
        peer_address: &str,
        set: &str,
    ) -> Result<PeerState, Error> {
        let params = rpc_params![peer_address, set];
        self.agent.request("peerState", params).await
    }

    pub async fn pool_confirmed_balance(&self) -> Result<u64, Error> {
        let params = rpc_params![];
        self.agent.request("poolConfirmedBalance", params).await
    }

    pub async fn pool_connection_state(&self) -> Result<u8, Error> {
        let params = rpc_params![];
        self.agent.request("poolConnectionState", params).await
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
    /// let result = client.create_raw_transaction(&tx).await;
    /// let hash = client.send_raw_transaction(&result).await;
    /// ```
    pub async fn send_raw_transaction(&self, transaction_hash: &str) -> Result<String, Error> {
        let params = rpc_params![transaction_hash];
        self.agent.request("sendRawTransaction", params).await
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
    /// let result = client.send_transaction(&tx).await;
    /// ```
    pub async fn send_transaction(
        &self,
        transaction: &OutgoingTransaction,
    ) -> Result<String, Error> {
        let params = rpc_params![transaction];
        self.agent.request("sendTransaction", params).await
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
    /// let result = client.submit_block("0da1....234").await;
    /// ```
    pub async fn submit_block(&self, full_block: &str) -> Result<(), Error> {
        let params = rpc_params![full_block];
        self.agent.request("submitBlock", params).await
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
    /// let result = client.syncing().await;
    /// ```
    pub async fn syncing(&self) -> Result<Syncing, Error> {
        let params = rpc_params![];
        self.agent.request("syncing", params).await
    }
}
