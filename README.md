# Real Time Payments

This repository contains the source for the Real Time Payments Proof of Concept.

## Documentation

Documentation via Gitbook can be found [here](https://mario-3.gitbook.io/rtp).

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

# run staging tests. Cannot run in parallel, so test threads is limited to 1
cargo test --features testnet -- --nocapture --test-threads=1
```

## Run via Docker

Build containers:

```sh
# build API logs container
docker build -t tarnadas/rtp-api -f docker/DockerfileApi .

# build indexer container
docker build -t tarnadas/rtp-indexer -f docker/DockerfileIndexer .

# build test runner container
docker build -t tarnadas/rtp-testnet -f docker/DockerfileTestnet .
```

Run containers:

```sh
# connect to API logs
docker run --rm -it -e CLOUDFLARE_API_TOKEN=$CLOUDFLARE_API_TOKEN -e CLOUDFLARE_ACCOUNT_ID=$CLOUDFLARE_ACCOUNT_ID -e CI=1 --name rtp-api tarnadas/rtp-api

# run indexer
docker run --rm -it -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY -e INDEXER_RPC_URL=$INDEXER_RPC_URL -e INDEXER_SECRET=$INDEXER_SECRET -e INDEXER_API_URL=$INDEXER_API_URL -e MASTER_ACCOUNT_ID=$MASTER_ACCOUNT_ID -e FACTORY_SUB_ACCOUNT=$FACTORY_ACCOUNT_ID --name rtp-indexer tarnadas/rtp-indexer

# run test runner
# give it the test you want to run
docker run --rm -it -e MASTER_ACCOUNT_ID=$MASTER_ACCOUNT_ID -e MASTER_SECRET_KEY=$MASTER_SECRET_KEY -e FACTORY_SUB_ACCOUNT=$FACTORY_SUB_ACCOUNT -e FACTORY_SECRET_KEY=$FACTORY_SECRET_KEY --name rtp-testnet tarnadas/rtp-testnet [TEST_NAME]
# list of tests:
# test_settle_trade_basic::spot
# test_settle_trade_basic::ndf
# test_settle_trade_basic::fwd
# test_settle_trade_basic::swap
# test_settle_trade_complex::spot
# test_settle_trade_complex::ndf
# test_settle_trade_complex::fwd
# test_settle_trade_complex::swap
# test_settle_trade_fail_match::spot
# test_settle_trade_fail_match::ndf
# test_settle_trade_fail_match::fwd
# test_settle_trade_fail_match::swap
# test_settle_trade_fail_payment::spot
# test_settle_trade_fail_payment::ndf
# test_settle_trade_fail_payment::fwd
# test_settle_trade_fail_payment::swap
# test_trade_timeout::spot
# test_trade_timeout::ndf
# test_trade_timeout::fwd
# test_trade_timeout::swap
```
