RUSTFLAGS="-Ctarget-cpu=haswell"

run:
	cargo run

# build:
# 	cargo build

release:
		cargo run --release
