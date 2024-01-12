# Real Time Payments

This repository contains the source for the Real Time Payments Proof of Concept.

## Setup

- install Nodejs
- install Rust
- install yarn

## Sandbox environment tests

You can run the sandbox tests in your local environment via:

```sh
cargo test -- --nocapture
```

The sandbox tests cannot be integrated with the off-chain infrastructure,
which is why there are also test cases running on Near testnet.

## Staging environment tests

In order to run the staging tests on Near testnet you need:

- AWS account for access to Near Lake
- paid Cloudflare account to have access to Durable Objects

```sh
# install dependencies
yarn

# login to Cloudflare
yarn api wrangler login

# store secret environment variables
yarn api wrangler secret --env staging put INDEXER_SECRET # this can be any crypto secure string and is used for API authorization
yarn api wrangler secret --env staging put FACTORY_PRIVATE_KEY # this is the private key generated for the factory account. If no account has been created so far the tests need to run once. Then you can copy the private key from "./.near/<factory_account_id>" file

# deploy staging API
yarn api wrangler deploy --env staging

# connect to staging API logs
yarn api wrangler tail --env staging

# start local indexer
cargo run -p rtp-indexer

# run staging tests. Cannot run in parallel, so #jobs is limited to 1
cargo test --features testnet -j 1 -- --nocapture
```
