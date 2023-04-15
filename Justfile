set dotenv-load
set positional-arguments

run *ARGS:
    just panko/run "$@"

test *ARGS:
    cargo test -- "$@"

build:
    cargo build

install:
    cargo install --path .

check:
    cargo check
