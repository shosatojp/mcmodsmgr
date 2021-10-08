TARGET:=x86_64-unknown-linux-musl
BIN:=mcmodsmgr
PROFILE:=release
BIN_PATH:=target/$(TARGET)/$(PROFILE)/$(BIN)
RUST_MUSL_BUILDER:=docker run -v $(shell pwd):/home/rust/src ekidd/rust-musl-builder

build:
	$(RUST_MUSL_BUILDER) cargo build --target $(TARGET) --$(PROFILE)
	strip $(BIN_PATH)
	-ldd $(BIN_PATH)
	ls -lh $(BIN_PATH)

test:
	$(RUST_MUSL_BUILDER) cargo test
