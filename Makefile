export RUST_LOG=info
# export RUST_LOG=debug

ipconfig:
	cargo run -- ipconfig

cloc:
	cloc . --exclude-dir=target
