# near_vault

Stake. Earn. Trade

## Local Setup

See [NEAR docs](https://docs.near.org/smart-contracts/quickstart) for ecosystem background. To bootstrap this repository:

```zsh
# Install Rust first: https://www.rust-lang.org/tools/install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```bash
bash scripts/setup.sh
```

The setup script installs:

- the host Rust toolchain used for formatting and unit tests
- the pinned contract toolchain used for WASM linting and builds
- `cargo-near` for contract packaging

## Local Checks

```bash
bash scripts/lint_contract.sh
bash scripts/build_contract.sh
cargo test --locked
```

`bash scripts/build_contract.sh reproducible-wasm` can be used to verify the reproducible build path locally from a clean git tree when Docker is available.

## Contract API

The setup branch now ships a minimal ownership contract:

- `new(owner)` initializes the contract
- `get_owner()` returns the current owner
- `transfer_ownership(new_owner)` updates ownership and is owner-only

## Deployment

No automated deployment workflow is configured yet. To deploy manually, install and authenticate [NEAR CLI](https://near.cli.rs), then run:

```bash
cargo near deploy build-reproducible-wasm <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
