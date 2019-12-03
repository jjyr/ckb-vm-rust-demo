# CKB VM Rust Demo

[rust-contract](https://github.com/jjyr/ckb-vm-rust-demo/tree/master/rust-contract) shows how to build a minimal rust contract.

The contract is composited by a `entry.c` file and a rust static library that actually implement the contract, check Makefile to see how to build the contract.

-----------------

> I'm failed to directly build a Rust binary that works on ckb-vm, However if you succeed to build a binary please send me a PR or open an issue!

## Usage

Add riscv target:

``` sh
rustup target add riscv64imac-unknown-none-elf
```

Use `make test` to compile and run, try modify the contract youself.
