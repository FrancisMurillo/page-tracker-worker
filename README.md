# Page Tracker Worker

A simple Rust Cloudflare KV worker to store incremental counters per
blog path for [fnlog.dev](https://fnlog.dev).

This is runned with [wrangler](https://github.com/cloudflare/wrangler)
CLI:

```shell
cargo install wrangler

# Login with Cloudflare credentials
wrangler login

# To run a local dev server
wrangler dev

# Test server with `/my-blog-path/article-name += 1`
curl -XGET http://127.0.0.1:8787/my-blog-path/article-name

# To build or publish afterwards
# Make sure to check if the KV binding is correct
wrangler build
wrangler publish
```
