#[cfg(test)]
mod tests {
    use nimiq_rpc::*;

    fn client() -> Client {
        let host = "http://seed-host.com:8648";

        if host == "http://seed-host.com:8648" || host == "http://seed-host.com:8648/" {
            panic!("You have to change the host to your RPC server in the tests!")
        }
        Client::new(host.to_string())
    }

    #[test]
    fn accounts() {
        let client = client();
        client.accounts().unwrap();
    }

    #[test]
    fn block_number() {
        let client = client();
        client.block_number().unwrap();
    }

    #[test]
    fn consensus() {
        let client = client();
        assert_eq!(client.consensus().unwrap(), "established");
    }

    // #[test]
    // fn create_account() {
    //   let client = client();
    //   client.create_account().unwrap();
    // }

    #[test]
    fn get_block_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_block_by_hash(
                    "A9284B441B56E93DE62F557414CC9B850BAD2BD30CF84B013CFE2EF6E11B6DA6",
                    false
                )
                .unwrap()
                .number,
            882418
        );
    }

    #[test]
    fn get_block_and_tx_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_block_by_hash(
                    "A9284B441B56E93DE62F557414CC9B850BAD2BD30CF84B013CFE2EF6E11B6DA6",
                    true
                )
                .unwrap()
                .number,
            882418
        );
    }

    #[test]
    fn get_block_by_number() {
        let client = client();
        assert_eq!(
            client.get_block_by_number(882418, false).unwrap().hash,
            "a9284b441b56e93de62f557414cc9b850bad2bd30cf84b013cfe2ef6e11b6da6"
        );
    }

    #[test]
    fn get_block_and_tx_by_number() {
        let client = client();
        assert_eq!(
            client.get_block_by_number(882418, true).unwrap().hash,
            "a9284b441b56e93de62f557414cc9b850bad2bd30cf84b013cfe2ef6e11b6da6"
        );
    }

    #[test]
    fn get_block_template() {
        let client = client();
        client.get_block_template().unwrap();
    }

    #[test]
    fn get_block_transaction_count_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_block_transaction_count_by_hash(
                    "A9284B441B56E93DE62F557414CC9B850BAD2BD30CF84B013CFE2EF6E11B6DA6"
                )
                .unwrap(),
            2
        );
    }

    #[test]
    fn get_block_transaction_count_by_number() {
        let client = client();
        assert_eq!(
            client
                .get_block_transaction_count_by_number(882418)
                .unwrap(),
            2
        );
    }

    #[test]
    fn get_transaction_by_block_hash_and_index() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_by_block_hash_and_index(
                    "dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f",
                    20
                )
                .unwrap()
                .hash,
            "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
        );
    }

    #[test]
    fn get_transaction_by_block_number_and_index() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_by_block_number_and_index(76415, 20)
                .unwrap()
                .hash,
            "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
        );
    }

    #[test]
    fn get_transaction_by_hash() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_by_hash(
                    "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
                )
                .unwrap()
                .block_hash,
            "dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f"
        );
    }

    #[test]
    fn get_transaction_receipt() {
        let client = client();
        assert_eq!(
            client
                .get_transaction_receipt(
                    "465a63b73aa0b9b54b777be9a585ea00b367a17898ad520e1f22cb2c986ff554"
                )
                .unwrap()
                .block_hash,
            "dfe7d166f2c86bd10fa4b1f29cd06c13228f893167ce9826137c85758645572f"
        );
    }

    #[test]
    fn get_transactions_by_address() {
        let client = client();
        client
            .get_transactions_by_address("NQ69 9A4A MB83 HXDQ 4J46 BH5R 4JFF QMA9 C3GN", 5)
            .unwrap();
    }

    #[test]
    fn get_work() {
        let client = client();
        client.get_work().unwrap();
    }

    #[test]
    fn hashrate() {
        let client = client();
        client.hashrate().unwrap();
    }

    #[test]
    fn log() {
        let client = client();
        assert_eq!(client.log("*", "log").unwrap(), true);
    }

    #[test]
    fn mempool_content() {
        let client = client();
        client.mempool_content().unwrap();
    }

    #[test]
    fn miner_address() {
        let client = client();
        client.miner_address().unwrap();
    }

    #[test]
    fn miner_threads() {
        let client = client();
        client.miner_threads().unwrap();
    }

    #[test]
    fn miner_threads_with_update() {
        let client = client();
        assert_eq!(client.miner_threads_with_update(1).unwrap(), 1);
    }

    #[test]
    fn min_fee_per_byte() {
        let client = client();
        client.min_fee_per_byte().unwrap();
    }

    #[test]
    fn min_fee_per_byte_with_update() {
        let client = client();
        assert_eq!(client.min_fee_per_byte_with_update(1).unwrap(), 1);
    }

    #[test]
    fn mining() {
        let client = client();
        client.mining().unwrap();
    }

    #[test]
    fn peer_count() {
        let client = client();
        client.peer_count().unwrap();
    }

    #[test]
    fn peer_list() {
        let client = client();
        client.peer_list().unwrap();
    }

    #[test]
    fn peer_state() {
        let client = client();
        client
            .peer_state("wss://urp.best:8443/a400c3825edb8e00f1d99dea5299bce8")
            .unwrap();
    }

    #[test]
    fn peer_state_with_update() {
        let client = client();
        client
            .peer_state_with_update(
                "wss://urp.best:8443/a400c3825edb8e00f1d99dea5299bce8",
                "connect",
            )
            .unwrap();
    }

    #[test]
    fn pool_confirmed_balance() {
        let client = client();
        client.pool_confirmed_balance().unwrap();
    }

    #[test]
    fn pool_connection_state() {
        let client = client();
        client.pool_connection_state().unwrap();
    }

    #[test]
    fn syncing() {
        let client = client();
        let state = client.syncing().unwrap();
        match state {
            primitives::Syncing::IsSyncing(result) => assert_eq!(result, false),
            primitives::Syncing::Pending(_) => assert!(true),
        }
    }
}
