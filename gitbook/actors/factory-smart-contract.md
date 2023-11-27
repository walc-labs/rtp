# 🏭 Factory Smart Contract

The Factory Smart Contract (FSC) is the main entrypoint for interacting with the RTP system. All functions of the FSC are private by default, which means only the owner of a [full access key](../terminology.md#access-keys) to the FSC address can call its functions.

The FSC is described by the following interface. SCs on Near Protocol are generally written in Rust and compiled to WebAssembly, so for the interface description Rust will be used.

```rust
trait Contract {
    fn store_contract(&mut self);
    
    fn create_bank(&mut self, bank: String);

    fn perform_trade(
        &mut self,
        bank_id: String,
        trade_details: TradeDetails,
    );
    
    fn settle_trade(
        &mut self,
        partnership_id: String,
        bank_a_id: String,
        bank_b_id: String,
        trade_id: String,
        deal_status: DealStatus,
    );
}

struct TradeDetails {
    trade_id: String,
    timestamp: u64,
    amount: String,
    price: String,
    side: Side,
    // ...
}

enum Side {
    Buy,
    Sell,
}

enum DealStatus {
    Pending,
    Confirmed(String),
    Rejected(String),
    Executed(String),
}
```

The FSC serves as a "factory" to deploy Bank Smart Contracts (BSCs). By deploying a Smart Contract for every bank we can take advantage of Near Protocols advanced scaling possibilities via sharding. We also make sure that the data for every bank is separated.

The `store_contract` function is used to initialize the FSC by storing the raw bytes of a BSC. These bytes are necessary to deploy a new instance of a BSC and must be called after the FSC has been deployed.

The `create_bank` function can be called to deploy a new BSC. Every BSC is deployed on a newly created [sub-address](../terminology.md#addresses) of the FSC, where the prefix is calculated by hashing the Bank's name using the default Rust hashing algorithm.

The `perform_trade` function can be called to send trade information to the respective BSC with `bank_id`. The trade will be stored in the respective BSC and processed further by the off-chain systems.

The `settle_trade` function can be called to change the status of a trade to either be confirmed, rejected or executed. Trades are matched by the off-chain system and no matching happens during Smart Contract execution.
