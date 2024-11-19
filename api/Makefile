# Makefile for Rust application

# Variables
CARGO := cargo

# Targets
.PHONY: all lint format test run

all: lint format test run

lint:
	@$(CARGO) clippy --all-targets --all-features -- -D warnings

format:
	@$(CARGO) fmt --all

test:
	@$(CARGO) test

run:
	@$(CARGO) run