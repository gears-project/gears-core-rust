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

## Build

    cargo build --release
    cargo build --features embedded
