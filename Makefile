TARGET:=x86_64-unknown-linux-musl
BIN:=mcmodsmgr
PROFILE:=release
BIN_PATH:=target/$(TARGET)/$(PROFILE)/$(BIN)
RUST_MUSL_BUILDER:=docker run -v $(shell pwd):/home/rust/src ekidd/rust-musl-builder

ARTIFACTS_DIR:=artifacts

build:
	$(RUST_MUSL_BUILDER) cargo build --target $(TARGET) --$(PROFILE)
	strip $(BIN_PATH)
	-ldd $(BIN_PATH)
	ls -lh $(BIN_PATH)

test:
	$(RUST_MUSL_BUILDER) cargo test

pull:
	mkdir -p $(ARTIFACTS_DIR)
	gh run download -n executable -D $(ARTIFACTS_DIR)

release: pull
	VERSION=`cat Cargo.toml | grep ^version | sed -E 's/^version = "([0-9.]+)"$/\1/'`
	gh release create -t "v${VERSION}" $(ARTIFACTS_DIR)/*
