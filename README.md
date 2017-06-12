# xflow-rust

[![Build Status](https://travis-ci.org/michiel/xflow-rust.svg?branch=master)](https://travis-ci.org/michiel/xflow-rust)
[![codecov](https://codecov.io/gh/michiel/xflow-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/michiel/xflow-rust)
[![Crates.io Status](http://meritbadge.herokuapp.com/xflow)](https://crates.io/crates/xflow)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/michiel/xflow-rust/master/LICENSE)
[![Documentation](https://docs.rs/xflow/badge.svg)](https://docs.rs/xflow)

## Testing

    cargo test -- --nocapture

    RUST_BACKTRACE=1 cargo test -- --nocapture

    RUST_BACKTRACE=1 cargo watch -x "test -- --nocapture"

    RUST_BACKTRACE=1 RUST_LOG=xflow=debug cargo watch -x "test -- --nocapture"

    RUST_BACKTRACE=1 RUST_LOG=xflow=debug cargo watch -x "test --features peg/trace -- --nocapture"

See https://doc.rust-lang.org/log/env_logger/ for more logging options

## Specifications

Use artifact-app

    art check
    art ls -p
    art export -o target/art html

## Build

    cargo build --release
    cargo build --features embedded

