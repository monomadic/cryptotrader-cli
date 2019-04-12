install:
	cargo build --release && cp ./target/release/cryptotrader-cli ~/.bin/ct

run:
	cargo build && ./target/debug/cryptotrader-cli pairs
