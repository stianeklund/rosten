# Rosten - track Bring & Posten packages.

Query the Bring API with tracking numbers to get the latest shipment status, written in Rust.
Compatible with Rust stable & nightly.

```
Rosten 1.1
Stian Eklund. <stiane@protonmail.com>
Shipment status of your Bring / Posten packages

USAGE:
    rosten [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --track <track>    Get package status
```
Rosten uses Serde & reqwest (previously used hyper), and is compatible with Rust stable & nightly.

Note: This API implementation is for learning purposes, this is not a complete nor a full featured implementation.
