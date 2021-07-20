# Cloudflare simple personalize use case

This project is a simple personalize case for a Cloudflare workers. It will check if a cookie is existing, if not it will return a Set-Cookie header in the response for set the cookie. The body of the response is a string with a message depending on whether the cookie is present or not.

## Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
* [`web-sys`](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html) provide imports for Web's API (here it will be for fetch)

## Installation

### In order to run this Workers, you will need to have :

1 Have a [Cloudflare Account](https://dash.cloudflare.com/sign-up/workers).

2 Install Rust

3 Install wrangler (Cloudflare Workers CLI) with cargo

```
cargo install wrangler
```

If you are on Windows, you will need to have Perl v5.10.0 or higher.

4 configure wrangler with your Cloudflare account.

```
wrangler login
```

if wrangler login doesn't work, use:

```
wrangler config
```

5 Complete the wrangler.toml

The account_id is necessary for use the preview.

It can be found with:

```
wrangler whoami
```

The [env.production] is for deploy at the Edge.

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

```
wrangler publish --env production
```

Will deploy the project on the domain you have defined in the wrangler.toml

A template for kick starting a Cloudflare worker project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting worker to Cloudflare's worker infrastructure.
