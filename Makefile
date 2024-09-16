.PHONY: build run test clean

build:
	cargo build

run:
	cargo run

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy

doc:
	cargo doc --open
