# Rosten - async tracking of Bring & Posten packages.

Query the Bring API with tracking numbers to get the latest shipment status, written in Rust.
Compatible with Rust stable & nightly.

This project has been updated to Rust 2018 edition and uses `async` and the `reqwest` crate.
The functionality and API implementation is sparse, but could serve as an easy introduction to REST APIs or Rust
for those new to Rust or REST API's.

```
Rosten 0.1.1
Stian Eklund. <stian.eklund@gmail.com>

Shipment status of your Bring / Posten packages

USAGE:
    rosten [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --track <tracking number> Get package status
```
Rosten uses Serde & reqwest (previously used hyper), and is compatible with Rust stable & nightly.

Note: This API implementation is for learning purposes, this is not a complete nor a full featured implementation.
