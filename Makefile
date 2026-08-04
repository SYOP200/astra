PROJECT = astra
TARGET = target/release/$(PROJECT)

.PHONY: all build release run test clean fmt check install uninstall docker

all: build

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

clean:
	cargo clean

install:
	./bin/install

uninstall:
	sudo rm -f /usr/local/bin/$(PROJECT)

brew:
	brew install --build-from-source ./astra-shell.rb

docker:
	docker build -t $(PROJECT) .

docker-run:
	docker run -it --rm $(PROJECT)

verify:
	cargo fmt --check
	cargo clippy -- -D warnings
	cargo test
