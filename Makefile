# contract compile
C := rust-contract
C_TARGET := riscv64-unknown-elf
CC := $(C_TARGET)-gcc
STRIP := $(C_TARGET)-strip
LD := $(C_TARGET)-gcc
CFLAGS := -O3 -Wall -Werror -Wno-nonnull -Wno-unused-function
LDFLAGS := -Wl,-static -flto -fdata-sections -ffunction-sections -Wl,--gc-sections -Wl,-s
# docker pull nervos/ckb-riscv-gnu-toolchain:bionic-20190702
BUILDER_DOCKER := nervos/ckb-riscv-gnu-toolchain@sha256:7b168b4b109a0f741078a71b7c4dddaf1d283a5244608f7851f5714fbad273ba

default: test

##@ Contracts

rust_contract: $C/src/lib.rs
	cd $C && cargo build --release

contracts: $C/entry.c
	$(CC) $< $C/target/riscv64imac-unknown-none-elf/release/librust_contract.a && $(STRIP) a.out

contracts-via-docker: rust_contract
	docker run --rm -v `pwd`:/code ${BUILDER_DOCKER} bash -c "cd /code && make contracts"

##@ Development
test: contracts-via-docker
	cargo run -- ./a.out
clean:
	cargo clean

# .PHONY:

