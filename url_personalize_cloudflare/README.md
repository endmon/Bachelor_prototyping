# Cloudflare url personalize

This project is a test of url parsing on Cloudflare workers. This project was made in anticipation of the project handlebars personalize Cloudflare.

This project tests: parameter retrieval, path analyse, kv Cloudflare Worker, method GET/POST.

## Demo

You will find this Worker live on [miguel-gouveia.me/url](http://miguel-gouveia.me/url)

## Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
* [`web-sys`](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html) provide imports for Web's API (here it will be for fetch)
* [`js-sys`](https://rustwasm.github.io/wasm-bindgen/contributing/js-sys/index.html) provide bindings for API JavaScript.
* [`serde`](https://github.com/serde-rs/serde) for serializing and deserializing.
* [`urlparse`](https://docs.rs/urlparse/0.7.3/urlparse/) for url parsing.
* [`worker_kv`](https://docs.rs/worker-kv/0.2.0/worker_kv/) for interact with Cloudflare Worker's KV.

## Installation

### In order to run this Workers, you will need to have :

1. #### Have a [Cloudflare Account](https://dash.cloudflare.com/sign-up/workers).

2. #### Install Rust

3. #### Install wrangler (Cloudflare Workers CLI) with cargo

```
cargo install wrangler
```

If you are on Windows, you will need to have Perl v5.10.0 or higher.

4.  #### Authenticates wrangler with your Cloudflare account.

```
wrangler login
```

if wrangler login doesn't work, use:

```
wrangler config
```

For more information on [authentication.](https://developers.cloudflare.com/workers/cli-wrangler/authentication)

5. #### Complete the wrangler.toml

```toml
name = "your-worker"
type = "rust"
account_id = "your-account-id"
# This field specifies that the Worker will be deployed to a *.workers.dev domain
workers_dev = true

kv_namespaces = [
    { binding = "my-kv", id = "kv-id" }
]
# This field specifies than the KV "my-kv" will be bind to this Worker

# These fields specify that the Worker will deploy to a custom domain
[env.production]
route = "exemple.com"
zone_id = "your-zone-id"
```

The account_id is necessary for use the preview.

It can be found with:

```
wrangler whoami
```

For create a Worker KV you will need to enter this command:
```
wrangler kv:namespace create "my-kv"
```

It will also give you the id for th kv.



route:  is where your Workers application will be served at.

zone_id: is an id you have on your Workers account after you have registered a domain name.

More configuration can be made in wrangler.toml, fro more details [see there](https://developers.cloudflare.com/workers/cli-wrangler/configuration).

### Build and Deploy

For building the projects:

```
wrangler build
```

Test the project:

```
wrangler dev
```

Will build the project and deploy on localhost:8787

Deploy:

```bash
wrangler publish
#publish your Worker to a *.workers.dev domain, if workers_dev is set
OR
wrangler publish --env production
#publish your Worker to the domain you have defined in your wrangler.toml
```

### Use

