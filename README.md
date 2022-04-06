# Rust developer exercise

## Exercise

1. Admin contract creates voting with parameters (minimum votes, percentage number of votes required to settlement).
2. Each address has only 1 vote -> For / Against / Abstain.
3. Summary for e.g rejected, accepted, not resolved with stats how much voted For / Against / Abstain.
4. [optional] Whitelist for addresses which can participate in voting.
5. [optional] Voting based on some cw20 token balance. 

General docs:
https://docs.terra.money

Best Rust docs:
https://doc.rust-lang.org/book/

Installing Rust:
https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup

When Rust is installed:
```bash
rustup target add wasm32-unknown-unknown
```

Build contracts:
```bash
cargo build
```

