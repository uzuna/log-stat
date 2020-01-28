RUSTFLAGS='-Zstrip-debuginfo-if-disabled=yes'
TARGET_AMD64=x86_64-unknown-linux-musl
TARGET_ARM64=aarch64-unknown-linux-gnu

setup:
	cargo install cross	

.PHONY: build_dev_amd64 build_dev_arm
build_dev_amd64:
	cargo build

build_dev_arm:
	cross build --target=${TARGET_ARM64}

# release build
# debug optionもなにもつかわない
.PHONY: build build_amd64 build_arm
build: build_amd64 build_arm

build_amd64: target/${TARGET_AMD64}/release/logstat

target/${TARGET_AMD64}/release/logstat:
	# RUSTFLAGS=${RUSTFLAGS} cross build --target=x86_64-unknown-linux-gnu --release
	docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder:nightly-2020-01-26 cargo build --release


build_arm: target/${TARGET_ARM64}/release/logstat

target/${TARGET_ARM64}/release/logstat:
	RUSTFLAGS=${RUSTFLAGS} cross build --target=${TARGET_ARM64} --release


# deb package
.PHONY: deb deb_amd64 deb_arm
deb: build deb_amd64 deb_arm

deb_amd64:
	cargo deb --target=${TARGET_AMD64} --no-build

deb_arm:
	cargo deb --target=${TARGET_ARM64} --no-build

# clean target
.PHONY: clean
clean:
	cargo clean

.PHONY: format
format:
	cargo fix

# check link
.PHONY: check_link
check_link:
	ldd target/${TARGET_AMD64}/release/logstat || true
	ldd target/${TARGET_ARM64}/release/logstat || true
