# walcft

This repository contains the source for the fungible token contract for the WALC project, which is deployed on Near mainnet at address [walc.near](https://nearblocks.io/address/walc.near).

In order to interact with the contract please install [Near CLI](https://github.com/near/near-cli).

## Deployment

```sh
# set environment to mainnet
export NEAR_ENV=mainnet
export CONTRACT_ID=walc.near

# login with Near wallet
near login

# build contract
./build_docker.sh

# deploy WASM binary to Near
near deploy --wasmFile out/fungible_token.wasm --accountId $CONTRACT_ID

# initialize contract state
near call $CONTRACT_ID new_default_meta '{"owner_id": "'$CONTRACT_ID'", "total_supply": "5000000000000000000000000000000000"}' --accountId $CONTRACT_ID

# check balance for owner
near view $CONTRACT_ID ft_balance_of '{"account_id": "'$CONTRACT_ID'"}'
```

## Upgrade & migration

The v1 version of the contract, which is the first version deployed on Near mainnet needs an upgrade and state migration for the new WASM binary to work. The reasoning behind the upgrade can be read in this [Pull Request](https://github.com/walc-labs/walcft/pull/1).

In order to run the migration on Near mainnet the new WASM binary needs to be deployed first. Then the `migrate` function needs to be run, which can only be done by the contract address itself.

```sh
# set environment to mainnet
export NEAR_ENV=mainnet
export CONTRACT_ID=walc.near

# login with Near wallet, if not already done
#near login

# deploy new WASM binary to Near
near deploy --wasmFile out/fungible_token.wasm --accountId $CONTRACT_ID

# migrate state
near call $CONTRACT_ID migrate '{ "owner": "walc.sputnik-dao.near" }' --accountId $CONTRACT_ID

# verify that state migration worked, by calling any function without throwing an error
near view $CONTRACT_ID ft_balance_of '{"account_id": "'$CONTRACT_ID'"}'
```

## DAO setup & contract locking

The WALC Sputnik DAO contract has been set up with the contract address [walc.sputnik-dao.near](https://nearblocks.io/address/walc.sputnik-dao.near). You can either use [AstroDAO UI](https://app.astrodao.com/dao/walc.sputnik-dao.near) or [Astra++](https://near.org/astraplusplus.ndctools.near/widget/home?daoId=walc.sputnik-dao.near&page=dao) to interact with it.

Before the `walc.near` can be locked, the upgrade and migration to the latest WASM binary needs to be done as explained above. The token bridging via [Portal Bridge](portalbridge.com) should also be done before locking. In theory it would be possible to do the bridging after locking, but it will no longer be possible to use Portal Bridge's UI for that.

Afterwards the following steps need to be done:

```sh
# set environment to mainnet
export NEAR_ENV=mainnet
export CONTRACT_ID=walc.near

# check if there are any WALC tokens left in the contract
near view $CONTRACT_ID ft_balance_of '{ "account_id": "'$CONTRACT_ID'" }'

# if yes, send them to the DAO
near call $CONTRACT_ID ft_transfer '{ "receiver_id": "walc.sputnik-dao.near", "amount": "'$AMOUNT'" }' --accountId $CONTRACT_ID

# print your `walc.near` public key
cat ~/.near-credentials/mainnet/walc.near.json | jq '.public_key'

# get list of all access keys
curl https://rpc.mainnet.near.org --request POST --header 'Content-Type: application/json' --data '{
  "jsonrpc": "2.0",
  "id": "dontcare",
  "method": "query",
  "params": {
    "request_type": "view_access_key_list",
    "finality": "final",
    "account_id": "walc.near"
  }
}' | jq

# delete any access key other than yours
near delete-key walc.near $ACCESS_KEY

# delete YOUR access key last to lock contract
near delete-key walc.near $ACCESS_KEY
```
