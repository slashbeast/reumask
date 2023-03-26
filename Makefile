all: release

release:
	cargo build --release

release-static:
	RUSTFLAGS='-C target-feature=+crt-static' cargo build --release
