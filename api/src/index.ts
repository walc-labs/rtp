import { Hono } from 'hono';
import { bearerAuth } from 'hono/bearer-auth';
import { cors } from 'hono/cors';
import { poweredBy } from 'hono/powered-by';
import { match } from 'ts-pattern';

import { batch } from './batch';
import { Env } from './global';
import { Info, info } from './info';
import { Partnerships } from './partnership';

const app = new Hono<{ Bindings: Env }>();

app.use('*', poweredBy());
app.use('*', cors());

app.use('*', async (c, next) => {
  const auth = bearerAuth({ token: c.env.INDEXER_SECRET });
  await auth(c, next);
});

app.route('/info', info);
app.route('/batch', batch);

app.onError(
  err =>
    new Response(null, {
      status: match(err.message)
        .with('Unauthorized', () => 401 as const)
        .with('Bad Request', () => 400 as const)
        .otherwise(() => {
          throw err;
        })
    })
);

app.notFound(() => {
  return new Response(null, { status: 404 });
});

export default app;

export { Info, Partnerships };
