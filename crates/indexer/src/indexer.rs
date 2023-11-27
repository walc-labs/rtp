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
use owo_colors::OwoColorize;
use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client, Url,
};
use rtp_common::{ContractEvent, NewBank, RtpEvent, RtpEventKind};
use serde::Deserialize;
use std::{env, sync::Arc};

static FACTORY_ACCOUNT_ID: Lazy<AccountId> =
    Lazy::new(|| env::var("FACTORY_ACCOUNT_ID").unwrap().parse().unwrap());

#[derive(Deserialize)]
struct Info {
    last_block_height: u64,
    bank_ids: Vec<String>,
}

pub async fn start_indexing() -> Result<impl Stream<Item = (BlockHeight, u64, Vec<RtpEvent>)>> {
    let client = Arc::new(JsonRpcClient::connect(env::var("INDEXER_RPC_URL").unwrap()));
    let (start_block_height, partnership_ids) = get_current_block_height(&client).await?;
    let mut partnership_ids: Vec<_> = partnership_ids
        .into_iter()
        .map(|id| format!("{id}.{}", *FACTORY_ACCOUNT_ID).parse().unwrap())
        .collect();

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
            let events = handle_message(msg, &mut partnership_ids);

            yield (block_height, timestamp, events);
        }
    })
}

fn handle_message(msg: StreamerMessage, bank_ids: &mut Vec<AccountId>) -> Vec<RtpEvent> {
    let mut res = vec![];
    for shard in msg.shards {
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
            if receiver_id != *FACTORY_ACCOUNT_ID && !bank_ids.contains(&receiver_id) {
                continue;
            }
            match outcome.status {
                ExecutionStatusView::Unknown | ExecutionStatusView::Failure(_) => continue,
                _ => {}
            }

            if let ReceiptEnumView::Action { .. } = receipt {
                let mut events =
                    extract_events(msg.block.header.timestamp / 1_000_000, &outcome, bank_ids);
                res.append(&mut events);
            }
        }

        if let Some(chunk) = shard.chunk {
            for transaction in chunk.transactions {
                if transaction.transaction.receiver_id != *FACTORY_ACCOUNT_ID
                    && !bank_ids.contains(&transaction.transaction.receiver_id)
                {
                    continue;
                }
                match transaction.outcome.execution_outcome.outcome.status {
                    ExecutionStatusView::Unknown | ExecutionStatusView::Failure(_) => continue,
                    _ => {}
                }
                let mut events = extract_events(
                    msg.block.header.timestamp_nanosec / 1_000_000,
                    &transaction.outcome.execution_outcome.outcome,
                    bank_ids,
                );
                res.append(&mut events);
            }
        }
    }
    res
}

async fn get_current_block_height(
    rpc_client: &Arc<JsonRpcClient>,
) -> anyhow::Result<(u64, Vec<String>)> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("INDEXER_SECRET")?).parse()?,
    );
    let client = Client::builder().default_headers(headers).build()?;
    let base_url = Url::parse(&env::var("INDEXER_API_URL")?)?;
    client.delete(base_url.join("info")?).send().await?;
    let info: Info = client
        .get(base_url.join("info")?)
        .send()
        .await?
        .json()
        .await?;
    if info.last_block_height > 0 {
        Ok((info.last_block_height + 1, info.bank_ids))
    } else {
        let status = rpc_client.call(methods::status::RpcStatusRequest).await?;
        Ok((status.sync_info.latest_block_height, info.bank_ids))
    }
}

fn extract_events(
    timestamp_ms: u64,
    outcome: &ExecutionOutcomeView,
    bank_ids: &mut Vec<AccountId>,
) -> Vec<RtpEvent> {
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
                println!(
                    "\n{}{}{}\n{}",
                    "=== new event (".bright_yellow(),
                    timestamp_ms.bright_yellow(),
                    ") ===".bright_yellow(),
                    &event
                );
                if let RtpEvent {
                    event_kind: RtpEventKind::NewBank(NewBank { bank_id, .. }),
                    ..
                } = &event
                {
                    bank_ids.push(
                        format!("{}.{}", bank_id, *FACTORY_ACCOUNT_ID)
                            .parse()
                            .unwrap(),
                    );
                }
                Some(event)
            } else {
                None
            }
        })
        .collect()
}
