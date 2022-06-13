.PHONY: watch

watch:
	cargo watch -x 'run -- -r /tmp'

.PHONY: release

release:
	cargo build --release --target x86_64-unknown-linux-musl

all: watch
