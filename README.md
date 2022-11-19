# Txl wrapper for Rust

[`txl`](http://txl.ca) is a transformation system developed by James R. Cordy
at Queen's University. This crate provides a command line utility to invoke it,
while offering a convenient function to invoke it inside Rust code.

[toc]

## Installation

Install the command:
```bash
cargo install txl-rs
```

Install the library:
```toml
[dependencies]
txl-rs = "*"
```

## Usage

### Command line usage:
```bash
txl-rs [args]
```
which would run as if it is a `txl [args]` command.

### Library usage: 
```rust
use txl_rs::txl;

fn main() {
    match txl(["src/main.rs"]) {
        Ok(result) => {
            println!("{result}");
        }
        Err(e) => {
            println!("{e}");
        }
    }
}
```

## Acknowledgement

## Updates
- [x] Integrate with Rust
- [x] Publish the crate
- [ ] make it platform independent
- [ ] adaptively downlad relevant parser packages 
- [ ] implement transformations as clippy fix rules
