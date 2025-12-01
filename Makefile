.PHONY: new build run test check clippy fmt clean help

help:
	@echo "Available targets:"
	@echo "  make new YEAR=2025 DAY=1  - Create boilerplate for a new day"
	@echo "  make build                - Build the project"
	@echo "  make run                   - Run the interactive solver"
	@echo "  make test                  - Run tests"
	@echo "  make check                 - Check code without building"
	@echo "  make clippy                - Run clippy linter"
	@echo "  make fmt                   - Format code"
	@echo "  make clean                 - Clean build artifacts"

new:
	@if [ -z "$(YEAR)" ] || [ -z "$(DAY)" ]; then \
		echo "Usage: make new YEAR=2025 DAY=1"; \
		exit 1; \
	fi
	@python3 scripts/new_day.py $(YEAR) $(DAY)

build:
	cargo build

run:
	cargo run

test:
	cargo test

check:
	cargo check

clippy:
	cargo clippy

fmt:
	cargo fmt

clean:
	cargo clean

