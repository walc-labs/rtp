# ⚙ Cloudflare Workers API

[Cloudflare Workers](https://workers.cloudflare.com/) (CW) is an edge & serverless computing platform developed and hosted by Cloudflare at its distributed computing network. The serverless approach allows developers to not have to maintain their own servers. The underlying infrastructure does not need any maintenance. Furthermore scaling on high traffic is done automatically.

CW has built in methods to spin up computing nodes (called [Durable Objects](https://developers.cloudflare.com/durable-objects/)), that can both be used to run computations and store private and consistent data on a unique location world wide. A node will be automatically spun up, when a request has been sent and it will automatically be put into sleep mode when no new requests arrive after a short time frame.

CW nodes can be configured to be GDPR compliant by [setting the location to a specific jurisdiction](https://developers.cloudflare.com/durable-objects/platform/data-location).
