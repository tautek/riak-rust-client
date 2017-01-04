# riak-rust-client

[![Build Status](https://travis-ci.org/shaneutt/riak-rust-client.svg?branch=master)](https://travis-ci.org/shaneutt/riak-rust-client)
[![crates.io](https://img.shields.io/crates/v/riak.svg)](https://crates.io/crates/riak)
[![License](https://img.shields.io/crates/l/riak.svg)](https://raw.githubusercontent.com/shaneutt/riak-rust-client/master/LICENSE)
[![Docs](https://img.shields.io/badge/docs-docs.rs-ff69b4.svg)](https://docs.rs/riak/)

A [Riak](https://github.com/basho/riak) client for Rust.

# Description

This client allows you to connect to the [Protocol Buffers API](http://docs.basho.com/riak/kv/latest/developing/api/protocol-buffers/) of a [Riak Node](http://basho.com/products/) and use the functionality provided to send data, retrieve data, and otherwise communicate with Riak.

This client communicates directly with the Protocol Buffer API on a specified Riak node, and does not provide any abstractions around a cluster of nodes (something TODO in a different crate later).

# Requirements

This client is tested against Rust's stable, beta, and nightly branches. It should work with any modern Rust.

You'll of course have to provide your own Riak nodes. If you don't already have some [check this documentation](http://docs.basho.com/riak/kv/latest/developing/getting-started/) to get you started using Riak.

# Usage

For complete documentation of what you can do with this client, see the [rustdoc](https://doc.rust-lang.org/book/documentation.html#about-rustdoc) documentation [available on docs.rs](https://docs.rs/riak/).

## Examples

Storing an object:

```rust
use riak::Client;
use riak::object::{ObjectContent, StoreObjectReq};

// connect to Riak and ping the server
let mut riak = Client::new("10.0.0.2:8087").unwrap();
riak.ping().unwrap();

// prepare an object
let contents = ObjectContent::new("This is a test!".as_bytes());

// build a request to store the object
let mut req = StoreObjectReq::new("testbucket", &contents);
req.set_key("testkey");

// store the object
riak.store_object(&req).unwrap();
```
