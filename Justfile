release:
  cargo build --release

build:
  cargo build --frozen

check:
  clear
  cargo c --frozen

format:
  clear
  cargo fmt
  cargo clippy --fix --allow-staged --allow-dirty

vendor:
  cargo vendor

serve:
  cargo watch -w src -s sh -- sh -c "clear; cargo run --frozen"
