install:
	cargo b --release
	mv ./target/release/slime-cli ~/.cargo/bin/slime
