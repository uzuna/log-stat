RUSTFLAGS='-Zstrip-debuginfo-if-disabled=yes'

setup:
	cargo install cross	

.PHONY: build_dev_amd64 build_dev_arm
build_dev_amd64:
	cargo build

build_dev_arm:
	cross build --target=aarch64-unknown-linux-gnu

# release build
# debug optionもなにもつかわない
.PHONY: build build_amd64 build_arm
build: build_amd64 build_arm

build_amd64: target/release/logstat

target/release/logstat:
	RUSTFLAGS=${RUSTFLAGS} cargo build --release

build_arm: target/aarch64-unknown-linux-gnu/release/logstat

target/aarch64-unknown-linux-gnu/release/logstat:
	RUSTFLAGS=${RUSTFLAGS} cross build --target=aarch64-unknown-linux-gnu --release


# deb package
.PHONY: deb deb_amd64 deb_arm
deb: build deb_amd64 deb_arm

deb_amd64:
	cargo deb --no-build

deb_arm:
	cargo deb --target=aarch64-unknown-linux-gnu --no-build

# clean target
.PHONY: clean
clean:
	cargo clean