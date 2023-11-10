import { Hono } from 'hono';
import { P, match } from 'ts-pattern';

import type { Env } from './global';
import { BatchData } from './types';

export const batch = new Hono<{ Bindings: Env }>().post('/', async c => {
  const batchData = await c.req.json<BatchData>();

  for (const event of batchData.events) {
    console.info('event:', event.event, event.data);
    await match(event)
      .with({ event: 'new_partnership', data: P.select() }, data => {
        const addr = c.env.INFO.idFromName('');
        const obj = c.env.INFO.get(addr);
        return obj.fetch(`${new URL(c.req.url).origin}/new_partnership`, {
          method: 'POST',
          body: data.partnership_id
        });
      })
      .with({ event: 'send_trade', data: P.select() }, data => {
        const addr = c.env.PARTNERSHIPS.idFromName(data.partnership_id);
        const obj = c.env.PARTNERSHIPS.get(addr);
        return obj.fetch(`${new URL(c.req.url).origin}/send_trade`, {
          method: 'POST',
          body: JSON.stringify(data)
        });
      })
      .with({ event: 'settle_trade', data: P.select() }, () => {
        // noop
      })
      .exhaustive();
  }
  return new Response(null, { status: 204 });
});
