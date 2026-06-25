all:
	cargo build --release && cp target/release/proedufi .

run:
	cargo run --release

clean:
	cargo clean
	rm -f proedufi
