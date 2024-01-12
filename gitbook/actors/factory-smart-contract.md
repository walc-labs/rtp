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
    
    fn perform_trade(
        &mut self,
        bank_id: String,
        trade_details: TradeDetails,
    );
    
    fn set_matching_status(
        &mut self,
        partnership_id: String,
        bank_a_id: String,
        bank_b_id: String,
        trade_id: String,
        matching_status: MatchingStatus,
    );
    
    fn confirm_payment(
        &mut self,
        creditor_id: String,
        debitor_id: String,
        trade_id: String,
    );
    
    fn set_payment_status(
        &mut self,
        partnership_id: String,
        bank_a_id: String,
        bank_b_id: String,
        trade_id: String,
        payment_status: PaymentStatus,
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

enum MatchingStatus {
    Pending,
    Confirmed(String),
    Rejected(String),
    Error,
}

enum PaymentStatus {
    Pending,
    Confirmed(String),
    Rejected(String),
    Error,
}
```

The FSC serves as a "factory" to deploy Bank Smart Contracts (BSCs). By deploying a Smart Contract for every bank we can take advantage of Near Protocols advanced scaling possibilities via sharding. We also make sure that the data for every bank is separated.

The `store_contract` function is used to initialize the FSC by storing the raw bytes of a BSC. These bytes are necessary to deploy a new instance of a BSC and must be called after the FSC has been deployed.

The `create_bank` function can be called to deploy a new BSC. Every BSC is deployed on a newly created [sub-address](../terminology.md#addresses) of the FSC, where the prefix is calculated by hashing the Bank's name using the default Rust hashing algorithm.

The `perform_trade` function can be called to send trade information to the respective BSC with `bank_id`. The trade will be stored in the respective BSC and processed further by the off-chain systems.

The `set_matching_status` function is called by the off-chain engine after two trades with the same `trade_id` have been received and matched against each other.

The `confirm_payment` function can be called to confirm payment of successfully matched trades for a specific side of the trade. Thus in order to fully confirm a trade, both sides of the payment confirmation need to be received.

The `set_payment_status` function is called by the off-chain engine after payments have been confirmed for a trade on both sides.
