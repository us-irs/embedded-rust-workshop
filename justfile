all: fmt check

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
