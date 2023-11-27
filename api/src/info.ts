import { Hono } from 'hono';

import type { Env } from './global';

export type InfoResult = {
  last_block_height: number;
  init_block_height: number;
  bank_ids: string[];
};

export const info = new Hono<{ Bindings: Env }>()
  .get('/', async c => {
    const addr = c.env.INFO.idFromName('');
    const obj = c.env.INFO.get(addr);
    const res = await obj.fetch(`${new URL(c.req.url).origin}/info`);
    const info = await res.json<InfoResult>();
    return c.jsonT(info);
  })
  .delete('/', async c => {
    const addr = c.env.INFO.idFromName('');
    const obj = c.env.INFO.get(addr);
    await obj.fetch(`${new URL(c.req.url).origin}/info`, { method: 'DELETE' });
    return new Response(null, { status: 204 });
  });

export class Info {
  private state: DurableObjectState;
  private app: Hono<{ Bindings: Env }>;
  private info?: InfoResult;

  constructor(state: DurableObjectState) {
    this.state = state;
    this.state.blockConcurrencyWhile(async () => {
      const info = await this.state.storage.get<InfoResult>('info');
      this.info = info ?? {
        last_block_height: 0,
        init_block_height: 0,
        bank_ids: []
      };
    });

    this.app = new Hono();
    this.app
      .get('*', c => {
        return c.json(this.info);
      })
      .delete('*', async c => {
        if (!this.info) return c.text('', 500);
        this.info.bank_ids = [];
        await this.state.storage.put('info', this.info);
        return new Response(null, { status: 204 });
      })
      .post('/last_block_height', async c => {
        if (!this.info) return c.text('', 500);
        const lastBlockHeight = Number(await c.req.text());
        this.info.last_block_height = lastBlockHeight;
        await this.state.storage.put('info', this.info);
        return new Response(null, { status: 204 });
      })
      .post('/init_block_height', async c => {
        if (!this.info) return c.text('', 500);
        const initBlockHeight = Number(await c.req.text());
        this.info.init_block_height = initBlockHeight;
        await this.state.storage.put('info', this.info);
        return new Response(null, { status: 204 });
      })
      .post('/new_bank', async c => {
        if (!this.info) return c.text('', 500);
        this.info.bank_ids.push(await c.req.text());
        await this.state.storage.put('info', this.info);
        return new Response(null, { status: 204 });
      });
  }

  async fetch(request: Request): Promise<Response> {
    return this.app.fetch(request);
  }
}
