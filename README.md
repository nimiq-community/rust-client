# Nimiq Rust Client

> Rust implementation of the Nimiq RPC client specs.

## About
A Nimiq RPC client library in Rust. This client library implements the [Nimiq RPC specification](https://github.com/nimiq/core-js/wiki/JSON-RPC-API). The client uses the jsonrpc library to handle JSON-RPC 2.0 requests and responses. For more information about this library see the [jsonrpc documentation](https://docs.rs/jsonrpc/0.11.0/jsonrpc/)

## Usage

``` rust
use nimiq_rpc::Client;

fn main() {
	let client = Client::new("http://seed-host.com:8648/");
	// If your node uses credentials
	let client = Client::new_with_credentials("http://seed-host.com:8648/", "user", "password");
	
	println!("{:?}", client.accounts().unwrap());
	println!("{:?}", client.block_number().unwrap());
	println!("{:?}", client.hashrate().unwrap());
	println!("{:?}", client.log("*", "log").unwrap());
}
```

## Documentation

[Docs.rs link](https://docs.rs/nimiq_rpc/0.1.1/nimiq_rpc/struct.Client.html) to the official crate documentation.

## Installation

Add the [crate](https://crates.io/crates/nimiq_rpc) to your `Cargo.toml` file.

## Contributions

This implementation was originally contributed by [Eligioo](https://github.com/eligioo/).

Please send your contributions as pull requests.
Refer to the [issue tracker](https://github.com/nimiq-community/rust-client/issues) for ideas.

## License

[Apache 2.0](LICENSE.md)
