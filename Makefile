.PHONY: build-server build-client run-server run-client clean

SERVER_BIN = auth-server
CLIENT_BIN = auth-client

build-server:
	cargo build --bin $(SERVER_BIN)

build-client:
	cargo build --bin $(CLIENT_BIN)

run-server: build-server
	cargo run --bin $(SERVER_BIN)

run-client: build-client
	cargo run --bin $(CLIENT_BIN)

clean:
	cargo clean