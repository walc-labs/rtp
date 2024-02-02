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
import {
  ConfirmPaymentData,
  MatchingStatus,
  PaymentStatus,
  SendTradeData,
  Trade
} from './types';

const MAX_TIMESTAMP_DIFF = 1_000 * 60; // 1 minute

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
    this.app
      .post('/send_trade', async c => {
        if (!this.near || !this.factoryContract) return c.text('', 500);
        const {
          partnership_id,
          bank_id: bank_a_id,
          trade: trade_details
        } = await c.req.json<SendTradeData>();
        try {
          const trade_a = await this.fetchOnChainTrade(
            env,
            bank_a_id,
            trade_details.trade_id
          );
          if (trade_a.matching_status.status !== 'Pending') {
            return new Response(null, { status: 204 });
          }
          const bank_b_id = await this.fetchBankId(
            env,
            trade_details.counterparty
          );
          const trade_b = await this.fetchOnChainTrade(
            env,
            bank_b_id,
            trade_details.trade_id
          );
          if (trade_b.matching_status.status !== 'Pending') {
            return new Response(null, { status: 204 });
          }

          console.info(
            `Trying to match trades. Trade A ID:\n`,
            trade_a.trade_details.trade_id,
            `\nTrade B ID:\n`,
            trade_b.trade_details.trade_id
          );

          let rejectedReason: string | undefined;
          for (const key of Object.keys(trade_a.trade_details)) {
            if (key === 'timestamp') {
              const timestampA = trade_a.trade_details[key];
              const timestampB = trade_b.trade_details[key];
              if (Math.abs(timestampA - timestampB) > MAX_TIMESTAMP_DIFF) {
                rejectedReason = 'trade matching timeout';
                break;
              }
            } else if (key === 'side') {
              const orderSideMatches = match(trade_a.trade_details.side)
                .with('Buy', () => trade_b.trade_details[key] === 'Sell')
                .with('Sell', () => trade_b.trade_details[key] === 'Buy')
                .exhaustive();
              if (!orderSideMatches) {
                rejectedReason = 'order side does not match';
                break;
              }
            } else if (key === 'counterparty') {
              const counterpartyMatches =
                trade_a.bank === trade_b.trade_details[key] &&
                trade_b.bank === trade_a.trade_details[key];
              if (!counterpartyMatches) {
                rejectedReason = `counterparties do not match. A:${trade_a.trade_details[key]}, B: ${trade_b.trade_details[key]}`;
                break;
              }
            } else {
              if (trade_a.trade_details[key] !== trade_b.trade_details[key]) {
                rejectedReason = `trade data with key "${key}" does not match. A: ${trade_a.trade_details[key]}, B: ${trade_b.trade_details[key]}`;
                break;
              }
            }
          }

          let newMatchingStatus: MatchingStatus;
          if (rejectedReason != null) {
            newMatchingStatus = {
              status: 'Rejected' as const,
              message: rejectedReason
            };
          } else {
            newMatchingStatus = {
              status: 'Confirmed' as const,
              message: `Trade with ID "${trade_details.trade_id}" confirmed`
            };
          }

          console.info(
            `Sending transaction to blockchain.\nMethod: 'set_matching_status'\nMatching status: ${JSON.stringify(
              newMatchingStatus,
              undefined,
              2
            )}'`
          );
          this.factoryContract
            .functionCall({
              contractId: this.factoryContract.accountId,
              methodName: 'set_matching_status',
              gas: '300000000000000',
              args: {
                partnership_id,
                bank_a_id,
                bank_b_id,
                trade_id: trade_details.trade_id,
                matching_status: newMatchingStatus
              }
            })
            .then(res => {
              console.info(
                `Transaction confirmed! Tx ID: ${res.transaction.hash}`
              );
            })
            .catch(err => {
              console.error(
                `Transaction could not be broadcast for trade ID: ${trade_a.trade_details.trade_id}\nError: ${err}`
              );
              // TODO
            });
          return new Response(null, { status: 204 });
        } catch (err) {
          console.error('Something went wrong:', err);
          return new Response(null, { status: 500 });
        }
      })
      .post('/payment_confirmed', async c => {
        if (!this.near || !this.factoryContract) return c.text('', 500);

        const { partnership_id, bank_id, trade_id }: ConfirmPaymentData =
          await c.req.json();

        try {
          const trade_a = await this.fetchOnChainTrade(env, bank_id, trade_id);
          const counterparty_id = await this.fetchBankId(
            env,
            trade_a.trade_details.counterparty
          );
          const trade_b = await this.fetchOnChainTrade(
            env,
            counterparty_id,
            trade_id
          );

          console.info(
            `Payment confirmed for trade A ID:\n`,
            trade_a.trade_details.trade_id,
            `\nand trade B ID:\n`,
            trade_b.trade_details.trade_id
          );

          if (
            trade_a.payments.credit &&
            trade_a.payments.debit &&
            trade_b.payments.credit &&
            trade_b.payments.debit
          ) {
            const payment_status = {
              status: 'Confirmed',
              message: `Payment for trade with ID "${trade_a.trade_details.trade_id}" confirmed`
            } satisfies PaymentStatus;
            console.info(
              `Sending transaction to blockchain.\nMethod: 'set_payment_status'\nPayment status: ${JSON.stringify(
                payment_status,
                undefined,
                2
              )}'`
            );
            this.factoryContract
              .functionCall({
                contractId: this.factoryContract.accountId,
                methodName: 'set_payment_status',
                gas: '300000000000000',
                args: {
                  partnership_id,
                  bank_a_id: bank_id,
                  bank_b_id: counterparty_id,
                  trade_id,
                  payment_status
                }
              })
              .then(res => {
                console.info(
                  `Transaction confirmed! Tx ID: ${res.transaction.hash}`
                );
              })
              .catch(err => {
                console.error(
                  `Transaction could not be broadcast for trade ID: ${trade_a.trade_details.trade_id}\nError: ${err}`
                );
                // TODO
              });
          }
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

  private async fetchOnChainTrade(
    env: Env,
    bank_id: string,
    trade_id: string
  ): Promise<Trade> {
    const res = await fetch(env.NEAR_RPC_URL, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        params: {
          request_type: 'call_function',
          finality: 'final',
          account_id: `${bank_id}.${env.FACTORY_ACCOUNT_ID}`,
          method_name: 'get_trade',
          args_base64: btoa(
            JSON.stringify({
              trade_id
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
    if (!json.result)
      throw new Error(
        `"get_trade" return type did not match expected: ${JSON.stringify(
          json
        )}`
      );
    const result = new Uint8Array(json.result.result);
    const decoder = new TextDecoder();
    const onChainTrade: Trade = JSON.parse(decoder.decode(result));
    return onChainTrade;
  }

  private async fetchBankId(env: Env, bank: string): Promise<string> {
    const res = await fetch(env.NEAR_RPC_URL, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        params: {
          request_type: 'call_function',
          finality: 'final',
          account_id: env.FACTORY_ACCOUNT_ID,
          method_name: 'get_bank_id',
          args_base64: btoa(
            JSON.stringify({
              bank
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
    if (!json.result)
      throw new Error(
        `"get_bank_id" return type did not match expected: ${JSON.stringify(
          json
        )}`
      );
    const result = new Uint8Array(json.result.result);
    const decoder = new TextDecoder();
    const bankId: string = JSON.parse(decoder.decode(result));
    return bankId;
  }
}
