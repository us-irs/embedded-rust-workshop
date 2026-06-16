all: fmt check build

# fmt --format both crates
fmt: fmt-apps fmt-exercises

fmt-apps:
  @cd microbit-apps && cargo fmt

fmt-exercises:
  @cd microbit-exercises && cargo fmt

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
