RUSTFLAGS='-Zstrip-debuginfo-if-disabled=yes'

.PHONY: build_dev
build_dev:
	cargo build

.PHONY: build_arm_dev
build_arm_dev:
	cross build --target=aarch64-unknown-linux-gnu

# release build
# debug optionもなにもつかわない
.PHONY: build
build:
	RUSTFLAGS=${RUSTFLAGS} cargo build --release

.PHONY: build_arm
build_arm:
	RUSTFLAGS=${RUSTFLAGS} cross build --target=aarch64-unknown-linux-gnu --release

setup:
	cargo install cross