# Smart Property Lease Contract

This is a smart contract for leasing a property on a blockchain network, implemented in Rust using the Ink! language.

The contract allows a landlord and a tenant to enter into a lease agreement for a fixed duration, during which the tenant pays rent to the landlord and a deposit is held in escrow. The contract keeps track of the start and end dates of the lease and enforces the payment of rent and the release of the deposit according to the terms of the agreement.

## Usage

*Warning*! This contract is an *example*. It is neither audited nor endorsed for production use. Do **not** rely on it to keep anything of value secure. Fill free to use this as a template and make changes according to your bussiness logic. 

To use this smart contract, you need to deploy it on a compatible blockchain network, such as the Polkadot or Kusama networks. 

Once the contract is deployed, the landlord and tenant can interact with it using compatible blockchain wallets that support the contract's functions. For example, the tenant can pay rent by calling the pay_rent function and sending the rent amount to the contract, while the landlord can release the deposit by calling the release_deposit function after the end of the lease.

## Development
To develop this smart contract, you need to have a basic understanding of Rust and the Ink! language. You can learn more about Rust at https://www.rust-lang.org/ and about Ink! at https://use.ink/.

To build and test the contract, you can use the Ink! CLI tool, which provides a Rust-like development experience for Ink! smart contracts. You can install the Ink! CLI tool using the following command:

```
rustup component add rust-src
cargo install --force --locked cargo-contract
```
If everything worked: 

```
cargo contract --help
```
You can then build and test the contract using the following commands:


```
cargo contract build
cargo test
```

For more information about developing smart contracts with Ink!, refer to the Ink! documentation.




