.phony: server daemon dev-server dev-daemon test

server:
	RUST_LOG=debug cargo run --bin server

daemon:
	RUST_LOG=debug cargo run --bin daemon

dev-server:
	RUST_LOG=debug cargo watch -x 'run --bin server'

dev-daemon:
	RUST_LOG=debug cargo watch -x 'run --bin daemon'






