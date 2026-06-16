all: fmt check clippy build

# fmt --format both crates
fmt: fmt-apps fmt-exercises

fmt-apps:
  @cd microbit-apps && cargo fmt

fmt-exercises:
  @cd microbit-exercises && cargo fmt

# clippy --lint both crates
clippy: clippy-apps clippy-exercises

clippy-apps:
	@cd microbit-apps && cargo clippy

clippy-exercises:
	@cd microbit-exercises && cargo clippy

# check --check both crates
check: check-apps check-exercises

check-apps:
  @cd microbit-apps && cargo check

check-exercises:
  @cd microbit-exercises && cargo check

# build both crates
build: build-apps build-exercises build-book

build-apps:
  @cd microbit-apps && cargo build

build-exercises:
  @cd microbit-exercises && cargo build

build-book:
  @cd exercise-book && mdbook build
