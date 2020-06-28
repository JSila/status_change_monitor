test:
	cargo run --bin daemon -- ./testdata/plan.json ./logs/status_change_monitor.log

release:
	cargo build --release

check:
	cargo check