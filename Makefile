
.PHONY: build run

build:
	cd source-server-cli && cargo build

run:
	cd source-server-cli && cargo run
