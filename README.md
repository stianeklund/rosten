#Rosten - track Bring & Posten packages.

Query the Bring API with tracking numbers to get the latest shipment status, written in Rust.
Compatible with Rust stable & nightly.

```
Rosten 1.0
Stian Eklund. <stiane@protonmail.com>
Get shipment status of your Bring / Posten packages

USAGE:
    rosten [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --track <track>    Get package status
```
Rosten uses Serde with Rust stable compatibility though code_gen, this means the main.rs is mostly empty and generated on build.
You can find the main program in main.rs.in.

Note: This API implementation is for learning purposes, this is not a complete fully featured implementation.
