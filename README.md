# Real Time Payments

This repository contains the source for the Real Time Payments solution.

## Setup

- install Nodejs
- install Rust
- install yarn

In order to run the staging tests you also need an AWS and Cloudflare account.

## Staging environment tests

```sh
# deploy staging API
yarn api wrangler deploy --env staging

# connect to staging API logs
yarn api wrangler tail --env staging

# start local indexer
cargo run -p rtp-indexer

# run staging tests
cargo test --features testnet -- --nocapture
```
