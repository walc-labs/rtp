import {
  Account,
  Near,
  connect,
  keyStores,
  utils
} from '@tarnadas/near-api-js';
import { Hono } from 'hono';
import { match } from 'ts-pattern';

import { Env } from './global';
import { DealStatus, SendTradeData, Trade } from './types';

const MAX_TIMESTAMP_DIFF = 1_000 * 60 * 60 * 2; // 2 hours

export class Partnerships {
  private state: DurableObjectState;
  private near?: Near;
  private factoryContract?: Account;
  private app: Hono<{ Bindings: Env }>;

  constructor(state: DurableObjectState, env: Env) {
    this.state = state;
    this.state.blockConcurrencyWhile(async () => {
      const keyPair = utils.KeyPair.fromString(env.FACTORY_PRIVATE_KEY);
      const networkId = 'testnet';

      const keyStore = new keyStores.InMemoryKeyStore();
      keyStore.setKey(networkId, env.FACTORY_ACCOUNT_ID, keyPair);

      this.near = await connect({
        keyStore,
        networkId,
        nodeUrl: env.NEAR_RPC_URL
      });
      this.factoryContract = await this.near.account(env.FACTORY_ACCOUNT_ID);
    });

    this.app = new Hono();
    this.app.post('/send_trade', async c => {
      if (!this.near || !this.factoryContract) return c.text('', 500);
      const { partnership_id, trade } = await c.req.json<SendTradeData>();
      try {
        const res = await fetch(env.NEAR_RPC_URL, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            params: {
              request_type: 'call_function',
              finality: 'final',
              account_id: `${partnership_id}.${env.FACTORY_ACCOUNT_ID}`,
              method_name: 'get_trade',
              args_base64: btoa(
                JSON.stringify({
                  trade_id: trade.trade_id
                })
              )
            },
            jsonrpc: '2.0',
            id: 'dontcare',
            method: 'query'
          })
        });
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        const json: any = await res.json();
        if (!json.result) return;
        const result = new Uint8Array(json.result.result);
        const decoder = new TextDecoder();
        const onChainTrade: Trade = JSON.parse(decoder.decode(result));

        const { trade_a, trade_b, deal_status } = onChainTrade;
        if (deal_status.status !== 'Pending' || !trade_a || !trade_b) {
          return new Response(null, { status: 204 });
        }

        let rejectedReason: string | undefined;
        for (const key of Object.keys(trade_a)) {
          if (key === 'timestamp') {
            const timestampA = trade_a[key];
            const timestampB = trade_b[key];
            if (Math.abs(timestampA - timestampB) > MAX_TIMESTAMP_DIFF) {
              rejectedReason = 'timestamp diff too high';
              break;
            }
          } else if (key === 'side') {
            const orderSideMatches = match(trade_a.side)
              .with('Buy', () => trade_b.side === 'Sell')
              .with('Sell', () => trade_b.side === 'Buy')
              .exhaustive();
            if (!orderSideMatches) {
              rejectedReason = 'order side does not match';
              break;
            }
          } else {
            if (trade_a[key] !== trade_b[key]) {
              rejectedReason = `trade data with key ${key} does not match. A: ${trade_a[key]}, B: ${trade_b[key]}`;
              break;
            }
          }
        }

        let newDealStatus: DealStatus;
        if (rejectedReason != null) {
          newDealStatus = {
            status: 'Rejected' as const,
            message: rejectedReason
          };
        } else {
          newDealStatus = {
            status: 'Confirmed' as const,
            message: 'it works'
          };
        }

        await this.factoryContract.functionCall({
          contractId: this.factoryContract.accountId,
          methodName: 'settle_trade',
          gas: '300000000000000',
          args: {
            partnership_id: partnership_id,
            trade_id: trade.trade_id,
            deal_status: newDealStatus
          }
        });
        return new Response(null, { status: 204 });
      } catch (err) {
        console.error('Something went wrong:', err);
        return new Response(null, { status: 500 });
      }
    });
  }

  async fetch(request: Request): Promise<Response> {
    return this.app.fetch(request);
  }
}
