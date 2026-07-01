all: fmt check clippy build

# fmt --format both crates
fmt: fmt-fw fmt-host

fmt-fw:
  @cd firmware && cargo fmt

fmt-host:
  @cd host && cargo fmt

# clippy --lint both crates
clippy: clippy-fw clippy-host

clippy-fw:
	@cd firmware && cargo clippy

clippy-host:
	@cd host && cargo clippy

# check --check both crates
check: check-fw check-host

check-fw:
  @cd firmware && cargo check

check-host:
  @cd host && cargo check

# build both crates
build: build-fw build-host build-book

build-fw:
  @cd firmware && cargo build

build-host:
  @cd host && cargo build

build-book:
  @cd exercise-book && mdbook build
