# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Allow defining callback-driven variable nodes with `Server::add_data_source_variable_node()`.
- Allow deletion of server nodes with `Server::delete_node()`.
- Add known enum variants to `ua::StatusCode`.
- Add method `ua::ExpandedNodeId::numeric()` to create numeric expanded node IDs.
- Add types `ua::RelativePath`, `ua::RelativePathElement`, `ua::BrowsePath`, `ua::BrowsePathResult`,
  `ua::BrowsePathTarget`.
- Add `ua::NodeAttributes` and subtypes `ua::ObjectAttributes`, `ua::VariableAttributes`,
  `ua::MethodAttributes`, `ua::ObjectTypeAttributes`, `ua::VariableTypeAttributes`,
  `ua::ReferenceTypeAttributes`, `ua::DataTypeAttributes`, `ua::ViewAttributes`.
- Add generic way of adding nodes with `Server::addNode()` and associated `Node` struct.

### Changed

- Include appropriate trait bounds in return type of `AsyncMonitoredItem::into_stream()`.
- Breaking: Add prefix `with_` in `ua::BrowseNextRequest::with_release_continuation_points()`.
- Upgrade to open62541 released version 1.4.2.

## [0.6.0-pre.5] - 2024-05-31

[0.6.0-pre.5]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.4...v0.6.0-pre.5

### Changed

- Upgrade to open62541 released version 1.4.1. This removes the workaround introduced in
  0.6.0-pre.3, it is no longer necessary.

## [0.6.0-pre.4] - 2024-05-22

[0.6.0-pre.4]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.3...v0.6.0-pre.4

### Changed

- Coerce _empty_ arrays of `ua::ExtensionObject` into the requested data type. This mirrors the
  auto-unwrapping behavior of open62541 version 1.4.

## [0.6.0-pre.3] - 2024-05-18

[0.6.0-pre.3]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.2...v0.6.0-pre.3

### Added

- Add logical OR combinator for `ua::BrowseResultMask` and `ua::NodeClassMask`.
- Add const `ua::NodeClassMask` variants to initialize masks.
- Add `serde` serialization for `ua::Array` and `ua::Variant` with array value.
- Add constructors `ua::Variant::scalar()` and `ua::Variant::array()`.
- Add constructor `ua::DataValue::new()`.
- Add helper method `ua::Array::into_array()` for conversion into native Rust array.

### Changed

- Breaking: Remove associated functions for enum data types deprecated in 0.5.0, e.g.
  `ua::AttributedId::value()`. Use uppercase constants `ua::AttributedId::VALUE` instead.
- Breaking: Split `Server::new()` and `ServerBuilder::build()` result type into `Server` and
  `ServerRunner` to allow interacting with server's data tree while server is running.
- Upgrade to open62541 released version 1.4.0.
- Reintroduce internal mutex in `AsyncClient` to work around issue in open62541 version 1.4.

### Fixed

- Avoid memory leak when calling `ua::Variant::with_scalar()` multiple times on the same value.

## [0.6.0-pre.2] - 2024-04-12

[0.6.0-pre.2]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.1...v0.6.0-pre.2

### Added

- Add basic support for creating OPC UA server with static nodes (#89).

## [0.6.0-pre.1] - 2024-04-05

[0.6.0-pre.1]: https://github.com/HMIProject/open62541/compare/v0.5.0...v0.6.0-pre.1

### Added

- Add `ua::StatusCode::is_uncertain()`, `is_bad()` for checking status code severity (#63).
- Add `ua::StatusCode::name()` to get human-readable representation of status code (#93).
- Add support for `ua::Argument` data type and basic support for `ua::ExtensionObject` (#71).
- Add `Debug` implementation for `ua::Array<T>` data types.
- Add `ValueType` enum to check `ua::Variant` without unwrapping (also `ua::Argument`).
- Add tracing log messages when processing service requests and responses (#80).
- Add methods to `ClientBuilder` to set response timeout, client description, and connectivity check
  interval (#81).

### Changed

- Breaking: Return `Result` instead of `Option` for references in `AsyncClient::browse_many()` and
  `browse_next()` (#59, #60).
- Breaking: Return `Result` wrapping `ua::DataValue` from `AsyncClient::read_attributes()` (#61).
- Breaking: Move `ua::VariantValue` and `ua::ScalarValue` to top-level export outside `ua`.
- Breaking: Remove `ua::ArrayValue` for now (until we have a better interface).
- Breaking: Return output arguments without `Option` from `AsyncClient::call_method()` (#79).
- Breaking: Remove misleading `FromStr` trait implementation and offer `ua::String::new()` instead.
- Breaking: Upgrade to open62541 version 1.4 (#82). This affects the API of this crate as follows:
  - Automatically unwrap `ua::ExtensionObject` arrays inside `ua::Variant`.
- Breaking: Remove `cycle_time` parameter from `AsyncClient`'s interface (#91). The relevance of
  this parameter has been reduced by the upgrade to open62541 version 1.4.

### Fixed

- Return browsing error instead of empty references list from `AsyncClient::browse()` (#60).
- Return reading error instead of unset `ua::DataValue` from `AsyncClient::read_value()` and
  `read_attribute()` (#61).
- Check only severity in `ua::StatusCode::is_good()` (#63). Previously this would be an exact
  comparison to `ua::StatusCode::GOOD`.
- No longer panic when unwrapping `ua::Variant` with array value.
- Treat invalid references array as empty in `ua::BrowseResult` on successful request (#77).
- Handle graceful disconnection when dropping synchronous `Client`.
- Include subscription ID in request when deleting monitored items (#93).

## [0.5.0] - 2024-03-01

[0.5.0]: https://github.com/HMIProject/open62541/compare/v0.4.0...v0.5.0

### Added

- Allow reading node attributes with `AsyncClient::read_attribute()` and `read_attributes()`.
- Allow continuing browsing from continuation points with `AsyncClient::browse_next()`.

### Changed

- Provide uppercase variants for enum data types, e.g. `ua::AttributedId::VALUE`. This deprecates
  the associated functions such as `ua::AttributedId::value()` formerly used for this purpose.
- Breaking: Return continuation points from `AsyncClient::browse()` and `browse_many()` (when not
  all references were returned, to be used with `AsyncClient::browse_next()`).
- Breaking: Simplify argument type `node_ids: &[impl Borrow<ua::NodeId>]` to `&[ua::NodeId]` in
  `AsyncClient::browse_many()`.
- Rename `ua::String::as_slice()` to `as_bytes()`. Deprecate the former method.

## [0.4.0] - 2024-02-12

[0.4.0]: https://github.com/HMIProject/open62541/compare/v0.3.0...v0.4.0

### Added

- Fallible conversion from `time::OffsetDateTime` to `ua::DateTime`.

### Changed

- Breaking: Renamed `ua::DateTime::as_datetime()` to `ua::DateTime::to_utc()`.
- Use RFC 3339 variant of ISO 8601 for `ua::DateTime` serialization.

## [0.3.0] - 2024-01-23

[0.3.0]: https://github.com/HMIProject/open62541/compare/v0.2.2...v0.3.0

### Added

- Allow setting secure channel lifetime and requested session timeout in `ClientBuilder`.
- Allow fetching current client state to periodically check for disconnect.

### Changed

- Breaking: Remove default `Display` implementation for most `ua` wrapper types (using the `Debug`
  implementation is more appropriate in these cases).

### Fixed

- Fix handling of empty and invalid strings.
- Include values in log messages (#22).

## [0.2.2] - 2024-01-12

[0.2.2]: https://github.com/HMIProject/open62541/compare/v0.2.1...v0.2.2

### Changed

- Fix typo in README file and changelog.

## [0.2.1] - 2024-01-12

[0.2.1]: https://github.com/HMIProject/open62541/releases/tag/v0.2.1

### Added

- First public release.
