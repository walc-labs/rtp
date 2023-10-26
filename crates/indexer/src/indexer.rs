use anyhow::Result;
use async_stream::stream;
use futures_core::Stream;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_lake_framework::{
    near_indexer_primitives::{
        types::{AccountId, BlockHeight},
        views::{
            ExecutionOutcomeView, ExecutionOutcomeWithIdView, ExecutionStatusView, ReceiptEnumView,
            ReceiptView,
        },
        IndexerExecutionOutcomeWithReceipt, StreamerMessage,
    },
    LakeConfigBuilder,
};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Url,
};
use rtp_common::{ContractEvent, RtpEvent};
use serde::Deserialize;
use std::{env, sync::Arc};

static DEX_ID: Lazy<AccountId> = Lazy::new(|| {
    let mut dex_id = env::var("DEX_SUB_ACCOUNT").unwrap();
    dex_id.push('.');
    dex_id.push_str(&env::var("MASTER_ACCOUNT_ID").unwrap());
    dex_id.parse().unwrap()
});

#[derive(Deserialize)]
struct Info {
    last_block_height: u64,
}

pub async fn start_indexing() -> Result<impl Stream<Item = (BlockHeight, u64, Vec<RtpEvent>)>> {
    let client = Arc::new(JsonRpcClient::connect(env::var("INDEXER_RPC_URL").unwrap()));
    let start_block_height = get_current_block_height(&client).await?;

    let config = LakeConfigBuilder::default()
        .testnet()
        .start_block_height(start_block_height)
        .build()
        .unwrap();

    let (_, mut stream) = near_lake_framework::streamer(config);

    Ok(stream! {
        while let Some(msg) = stream.recv().await {
            let block_height = msg.block.header.height;
            let timestamp = msg.block.header.timestamp_nanosec;
            let events = handle_message(msg);

            yield (block_height, timestamp, events);
        }
    })
}

fn handle_message(msg: StreamerMessage) -> Vec<RtpEvent> {
    msg.shards
        .into_par_iter()
        .find_map_first(|shard| {
            let mut res = vec![];
            for IndexerExecutionOutcomeWithReceipt {
                execution_outcome: ExecutionOutcomeWithIdView { outcome, .. },
                receipt:
                    ReceiptView {
                        receipt,
                        receiver_id,
                        ..
                    },
            } in shard.receipt_execution_outcomes
            {
                if receiver_id != *DEX_ID {
                    continue;
                }
                match outcome.status {
                    ExecutionStatusView::Unknown | ExecutionStatusView::Failure(_) => continue,
                    _ => {}
                }

                if let ReceiptEnumView::Action { .. } = receipt {
                    let mut events = extract_events(&outcome);
                    res.append(&mut events);
                }
            }

            if let Some(chunk) = shard.chunk {
                for transaction in chunk.transactions {
                    if transaction.transaction.receiver_id != *DEX_ID {
                        continue;
                    }
                    match transaction.outcome.execution_outcome.outcome.status {
                        ExecutionStatusView::Unknown | ExecutionStatusView::Failure(_) => continue,
                        _ => {}
                    }
                    let mut events = extract_events(&transaction.outcome.execution_outcome.outcome);
                    res.append(&mut events);
                }
            }
            if res.is_empty() {
                None
            } else {
                Some(res)
            }
        })
        .unwrap_or_default()
}

async fn get_current_block_height(rpc_client: &Arc<JsonRpcClient>) -> anyhow::Result<u64> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("INDEXER_SECRET")?).parse()?,
    );
    let client = Client::builder().default_headers(headers).build()?;
    let base_url = Url::parse(&env::var("INDEXER_API_URL")?)?;
    let info: Info = client
        .get(base_url.join("info")?)
        .send()
        .await?
        .json()
        .await?;
    if info.last_block_height > 0 {
        Ok(info.last_block_height + 1)
    } else {
        let status = rpc_client.call(methods::status::RpcStatusRequest).await?;
        Ok(status.sync_info.latest_block_height)
    }
}

fn extract_events(outcome: &ExecutionOutcomeView) -> Vec<RtpEvent> {
    let prefix = "EVENT_JSON:";
    outcome
        .logs
        .iter()
        .filter_map(|untrimmed_log| {
            let log = untrimmed_log.trim();
            if !log.starts_with(prefix) {
                return None;
            }

            if let Ok(ContractEvent::Rtp(event)) =
                serde_json::from_str::<ContractEvent>(log[prefix.len()..].trim())
            {
                println!("  {}", &event);
                Some(event)
            } else {
                None
            }
        })
        .collect()
}
