#[cfg(test)]
mod tests {
    use nimiq_rpc::*;
    use url::Url;

    fn client() -> Client {
        let host = "http://seed-host.com:8648";

        if host == "http://seed-host.com:8648" || host == "http://seed-host.com:8648/" {
            panic!("You have to change the host to your RPC server in the tests!")
        }
        Client::new(Url::parse(host).unwrap())
    }

    #[tokio::test]
    async fn accounts() {
        let client = client();
        client.accounts().await.unwrap();
    }

    #[tokio::test]
    async fn block_number() {
        let client = client();
        client.block_number().await.unwrap();
    }

    #[tokio::test]
    async fn consensus() {
        let client = client();
        assert_eq!(client.consensus().await.unwrap(), "established");
    }

    #[tokio::test]
    async fn create_account() {
        let client = client();
        client.create_account().await.unwrap();
    }

    #[tokio::test]
    async fn get_account() {
        let client = client();
        client
            .get_account("NQ07 0000 0000 0000 0000 0000 0000 0000 0000")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_block_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_block_by_hash(
                    "A9284B441B56E93DE62F557414CC9B850BAD2BD30CF84B013CFE2EF6E11B6DA6",
                    false
                )
                .await
                .unwrap()
                .number,
            882418
        );
    }

    #[tokio::test]
    async fn get_block_and_tx_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_block_by_hash(
                    "A9284B441B56E93DE62F557414CC9B850BAD2BD30CF84B013CFE2EF6E11B6DA6",
                    true
                )
                .await
                .unwrap()
                .number,
            882418
        );
    }

    #[tokio::test]
    async fn get_block_by_number() {
        let client = client();
        assert_eq!(
            client
                .get_block_by_number(882418, false)
                .await
                .unwrap()
                .hash,
            "a9284b441b56e93de62f557414cc9b850bad2bd30cf84b013cfe2ef6e11b6da6"
        );
    }

    #[tokio::test]
    async fn get_block_and_tx_by_number() {
        let client = client();
        assert_eq!(
            client.get_block_by_number(882418, true).await.unwrap().hash,
            "a9284b441b56e93de62f557414cc9b850bad2bd30cf84b013cfe2ef6e11b6da6"
        );
    }

    #[tokio::test]
    async fn get_block_template() {
        let client = client();
        client.get_block_template().await.unwrap();
    }

    #[tokio::test]
    async fn get_block_transaction_count_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_block_transaction_count_by_hash(
                    "A9284B441B56E93DE62F557414CC9B850BAD2BD30CF84B013CFE2EF6E11B6DA6"
                )
                .await
                .unwrap(),
            2
        );
    }

    #[tokio::test]
    async fn get_block_transaction_count_by_number() {
        let client = client();
        assert_eq!(
            client
                .get_block_transaction_count_by_number(882418)
                .await
                .unwrap(),
            2
        );
    }

    #[tokio::test]
    async fn get_transaction_by_block_hash_and_index() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_by_block_hash_and_index(
                    "dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f",
                    20
                )
                .await
                .unwrap()
                .hash,
            "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
        );
    }

    #[tokio::test]
    async fn get_transaction_by_block_number_and_index() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_by_block_number_and_index(76415, 20)
                .await
                .unwrap()
                .hash,
            "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
        );
    }

    #[tokio::test]
    async fn get_transaction_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_by_hash(
                    "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
                )
                .await
                .unwrap()
                .block_hash,
            "dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f"
        );
    }

    #[tokio::test]
    async fn get_transaction_receipt() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_receipt(
                    "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
                )
                .await
                .unwrap()
                .block_hash,
            "dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f"
        );
    }

    #[tokio::test]
    async fn get_transactions_by_address() {
        let client = client();
        client
            .get_transactions_by_address("NQ69 9A4A MB83 HXDQ 4J46 BH5R 4JFF QMA9 C3GN", 5)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_work() {
        let client = client();
        client.get_work().await.unwrap();
    }

    #[tokio::test]
    async fn hashrate() {
        let client = client();
        client.hashrate().await.unwrap();
    }

    #[tokio::test]
    async fn log() {
        let client = client();
        assert_eq!(client.log("*", "log").await.unwrap(), true);
    }

    #[tokio::test]
    async fn mempool_content() {
        let client = client();
        client.mempool_content().await.unwrap();
    }

    #[tokio::test]
    async fn miner_address() {
        let client = client();
        client.miner_address().await.unwrap();
    }

    #[tokio::test]
    async fn miner_threads() {
        let client = client();
        client.miner_threads().await.unwrap();
    }

    #[tokio::test]
    async fn miner_threads_with_update() {
        let client = client();
        assert_eq!(client.miner_threads_with_update(1).await.unwrap(), 1);
    }

    #[tokio::test]
    async fn min_fee_per_byte() {
        let client = client();
        client.min_fee_per_byte().await.unwrap();
    }

    #[tokio::test]
    async fn min_fee_per_byte_with_update() {
        let client = client();
        assert_eq!(client.min_fee_per_byte_with_update(1).await.unwrap(), 1);
    }

    #[tokio::test]
    async fn mining() {
        let client = client();
        client.mining().await.unwrap();
    }

    #[tokio::test]
    async fn peer_count() {
        let client = client();
        client.peer_count().await.unwrap();
    }

    #[tokio::test]
    async fn peer_list() {
        let client = client();
        client.peer_list().await.unwrap();
    }

    #[tokio::test]
    async fn peer_state() {
        let client = client();
        client
            .peer_state("wss://urp.best:8443/a400c3825edb8e00f1d99dea5299bce8")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn peer_state_with_update() {
        let client = client();
        client
            .peer_state_with_update(
                "wss://urp.best:8443/a400c3825edb8e00f1d99dea5299bce8",
                "connect",
            )
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn pool_confirmed_balance() {
        let client = client();
        client.pool_confirmed_balance().await.unwrap();
    }

    #[tokio::test]
    async fn pool_connection_state() {
        let client = client();
        client.pool_connection_state().await.unwrap();
    }

    #[tokio::test]
    async fn syncing() {
        let client = client();
        let state = client.syncing().await.unwrap();
        match state {
            primitives::Syncing::IsSyncing(result) => assert_eq!(result, false),
            primitives::Syncing::Pending(_) => assert!(true),
        }
    }

    #[tokio::test]
    async fn constant() {
        let client = client();
        let constant = client
            .get_constant("BaseConsensusAgent.FREE_TRANSACTIONS_PER_SECOND")
            .await
            .unwrap();
        assert_eq!(constant, 1);

        // Now set the constant
        let constant = client
            .set_constant("BaseConsensusAgent.TRANSACTION_RELAY_FEE_MIN", 2)
            .await
            .unwrap();
        assert_eq!(constant, 2);

        // Get constant again
        let constant = client
            .get_constant("BaseConsensusAgent.TRANSACTION_RELAY_FEE_MIN")
            .await
            .unwrap();
        assert_eq!(constant, 2);

        // Reset constant
        let constant = client
            .reset_constant("BaseConsensusAgent.TRANSACTION_RELAY_FEE_MIN")
            .await
            .unwrap();
        assert_eq!(constant, 1);
    }
}
