TARGET:=x86_64-unknown-linux-musl
BIN:=mcmodsmgr
PROFILE:=release
BIN_PATH:=target/$(TARGET)/$(PROFILE)/$(BIN)

build:
	docker run -it -u "$()" -v $(shell pwd):/home/rust/src ekidd/rust-musl-builder cargo build --target $(TARGET) --$(PROFILE)
	strip $(BIN_PATH)
	-ldd $(BIN_PATH)
	ls -lh $(BIN_PATH)
