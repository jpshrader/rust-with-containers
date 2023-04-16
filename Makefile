build:
	cargo build

run:
	make build
	cargo run

restore:
	cargo clean
	cargo fetch

update:
	cargo update