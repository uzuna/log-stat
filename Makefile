.PHONY: build_arm
build_arm:
	cross build --target=aarch64-unknown-linux-gnu --release


setup:
	cargo install cross