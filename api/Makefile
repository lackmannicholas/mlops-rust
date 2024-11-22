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

# make build
build-release:
	@$(CARGO) build --release

# make build local
build:
	@$(CARGO) build

# make docker build
docker-build:
	@docker build -t rust-api .

# make docker run
docker-run:
	@docker run -p 8080:8080 rust-api
