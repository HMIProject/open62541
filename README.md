# open62541

[![crates.io](https://img.shields.io/crates/v/open62541.svg)](https://crates.io/crates/open62541)
[![Docs](https://docs.rs/open62541/badge.svg)](https://docs.rs/open62541)
[![Dependencies](https://deps.rs/repo/github/HMIProject/open62541/status.svg)](https://deps.rs/repo/github/HMIProject/open62541)
[![Testing](https://github.com/HMIProject/open62541/actions/workflows/test.yaml/badge.svg)](https://github.com/HMIProject/open62541/actions/workflows/test.yaml)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-blue.svg)](https://opensource.org/licenses/MPL-2.0)

This crate provides high-level, safe bindings for the C99 library
[open62541](https://www.open62541.org), an open source and free implementation of
[OPC UA](https://opcfoundation.org/about/opc-technologies/opc-ua/).

## Overview

Use this crate when you want to implement an OPC UA client or server in Rust, or add these
capabilities to an existing program.

## Examples

You can find examples in our [documentation](https://docs.rs/open62541) and in the `examples/`
folder in our repository.

## Contributing

Make sure to use `LF` line endings and run `just pre-commit` before committing your changes.

Visit the [Just Programmer's Manual](https://just.systems/man/en/) for information on installing
`just`. After installing, run `just setup` to setup the environment required by this repository.

Tip: Configure your IDE to apply `cargo fmt` formatting when saving a file.
