# 📖 Indexer

The indexer is a program that watches and downloads the latest [blocks](../terminology.md#block) of the blockchain. It then goes through all transactions that are tracked inside these blocks and checks for occurrences of [FSC](factory-smart-contract.md) or [PSC](partnership-smart-contract.md) function calls.\
\
The indexer is built on top of the [Near Lake Framework](https://docs.near.org/tools/near-lake). The way it works is that a trusted entity ([Pagoda](https://www.pagoda.co/) in this case) hosts a [node](../terminology.md#node) of the blockchain, which downloads all the block data. This data will then be uploaded to an Amazon Web Services S3 bucket, where they can be downloaded by applications, that use the Near Lake Framework.

The indexer initially only tracks the FSC and watches for [event](../terminology.md#events) emits. Possible events are described by the following Rust enum:

```rust
#[near_bindgen(event_json(standard = "rtp"))]
pub enum RtpEvent {
    #[event_version("1.0.0")]
    NewPartnership { partnership_id: String },
    #[event_version("1.0.0")]
    SendTrade {
        partnership_id: String,
        bank: String,
        trade: TradeDetails,
    },
    #[event_version("1.0.0")]
    SettleTrade {
        partnership_id: String,
        trade_id: String,
        deal_status: DealStatus,
    },
}
```

When an `RtpEvent::NewPartnership` event has been emitted, the indexer will also keep track of the resulting PSC function calls.

When an `RtpEvent::SendTrade` or `RtpEvent::SettleTrade` event was found it will be sent to the [Cloudflare Workers API](cloudflare-workers-api.md), where trades will be processed and matched.
