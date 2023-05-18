# Gossip Glomers

Distributed computing challenges.

Resources:
* https://fly.io/dist-sys/1/ 
* https://github.com/jepsen-io/maelstrom


## Usage


To run the maelstrom test:

```
cargo build

test -w echo --bin target/debug/gossip_glomers --time-limit 20 --log-stderr               
```