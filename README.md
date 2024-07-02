# ICP Token Wallet

## Setup

1. Install Rust and DFINITY SDK.
2. Clone the repository.
3. Start a local ICP testnet: `dfx start --background`.
4. Deploy the canister: `dfx deploy`.

## Usage

- Send tokens: `dfx canister call icp_token_wallet send_tokens '("recipient", amount)'`.
- Check balance: `dfx canister call icp_token_wallet get_balance`.

## Testing

- Run unit tests: `cargo test`.
