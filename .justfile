# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Format source code
fmt:
    cargo fmt --all

# Run code checks
clippy:
    cargo clippy --locked --no-deps --all-targets --all-features -- -D warnings --cap-lints warn

# Check the standalone build of all crates for each feature
check-features:
    cargo hack check --each-feature --no-dev-deps

# Check the dependencies of all crates with default features
check-crates:
    cargo check

# Run unit tests
test:
    # The options `--doc` and `--all-targets` cannot be used together and `--all-targets`
    # would exclude the doctests. Specifying none of them gives the desired result.
    RUST_BACKTRACE=1 cargo test --locked --all-features -- --nocapture

# Set up (and update) development tools
setup:
    # Ignore rustup failures, because not everyone might use it
    rustup self update || true
    # cargo-edit is needed for `cargo upgrade`
    cargo install cargo-edit cargo-hack just
    pip install -U pre-commit

# Upgrade (and update) dependencies
upgrade: setup
    pre-commit autoupdate
    # Agressively upgrade dependencies, only exclude pinned versions
    cargo upgrade --incompatible
    cargo update

# Run pre-commit hooks
pre-commit:
    pre-commit run --all-files
