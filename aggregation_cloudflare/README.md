# Cloudflare aggregation simple use case

This project is a simple case of aggregation for a Cloudflare worker. It will get 3 txt on a http server, concatenate them and return the result.

## Demo

You will find this Worker live on [miguel-gouveia.me/aggregation](http://miguel-gouveia.me/aggregation)

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

## Installation

### In order to run this Workers, you will need to have :

1. #### Have a [Cloudflare Account](https://dash.cloudflare.com/sign-up/workers).

2. #### Install Rust

3. #### Install wrangler (Cloudflare Workers CLI) with cargo

```bash
cargo install wrangler
```

If you are on Windows, you will need to have Perl v5.10.0 or higher.

4. #### Authenticates wrangler with your Cloudflare account.

```bash
wrangler login
```

if wrangler login doesn't work, use:

```bash
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

To modify the file to aggregate, should change the variable link1,link2,link3 in src/lib.r.
