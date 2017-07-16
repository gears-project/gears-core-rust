# gears-core-rust

[![Build Status](https://travis-ci.org/gears-project/gears-core-rust.svg?branch=master)](https://travis-ci.org/gears-project/gears-core-rust)
[![codecov](https://codecov.io/gh/gears-project/gears-core-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/gears-project/gears-core-rust)
[![Crates.io Status](http://meritbadge.herokuapp.com/gearsg)](https://crates.io/crates/gears)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/gears-project/gears-core-rust/master/LICENSE)
[![Documentation](https://docs.rs/gearsg/badge.svg)](https://docs.rs/gears)

## Testing

    cargo test -- --nocapture

    RUST_BACKTRACE=1 cargo test -- --nocapture

    RUST_BACKTRACE=1 cargo watch -x "test -- --nocapture"

    RUST_BACKTRACE=1 RUST_LOG=gearsg=debug cargo watch -x "test -- --nocapture"

    RUST_BACKTRACE=1 RUST_LOG=gearsg=debug cargo watch -x "test --features peg/trace -- --nocapture"

See https://doc.rust-lang.org/log/env_logger/ for more logging options

## Specifications

Use artifact-app

    art check
    art ls -p
    art export -o target/art html

## Build

    cargo build --release
    cargo build --features embedded

