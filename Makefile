test:
	cargo run -- ./testdata/plan.json ./logs/status_change_monitor.log

release:
	cargo build --release

check:
	clear
	cargo check