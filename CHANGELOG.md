# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Add `ua::StatusCode::is_uncertain()`, `is_bad()` for checking status code severity.
- Add support for `ua::Argument` data type and basic support for `ua::ExtensionObject`.
- Add `Debug` implementation for `ua::Array<T>` data types.
- Add `ValueType` enum to check `ua::Variant` without unwrapping (also `ua::Argument`).
- Add tracing log messages when processing service requests and responses.
- Add methods to `ClientBuilder` to set response timeout, client description, and connectivity check
  interval.

### Changed

- Breaking: Return `Result` instead of `Option` for references in `AsyncClient::browse_many()` and
  `browse_next()` (#59).
- Breaking: Return `Result` instead of raw `ua::DataValue` from `AsyncClient::read_attributes()`.
- Breaking: Move `ua::VariantValue` and `ua::ScalarValue` to top-level export outside `ua`
- Breaking: Remove `ua::ArrayValue` for now (until we have a better interface).
- Breaking: Return output arguments directly from `AsyncClient::call_method()`, without `Option`.
- Breaking: Remove misleading `FromStr` trait implementation and offer `ua::String::new()` instead.
- Upgrade to open62541 version 1.4. By itself, this should not affect the public API of this crate.

### Fixed

- Return browsing error instead of empty references list from `AsyncClient::browse()`.
- Return reading error instead of unset `ua::DataValue` from `AsyncClient::read_value()` and
  `read_attribute()`.
- Check only severity in `ua::StatusCode::is_good()`. Previously this would be an exact comparison
  to `ua::StatusCode::GOOD`.
- No longer panic when unwrapping `ua::Variant` with array value.
- Allow invalid references array in `ua::BrowseResult` when request was otherwise successful.
- Handle graceful disconnection when dropping synchronous `Client`.

## [0.5.0] - 2024-03-01

[0.5.0]: https://github.com/HMIProject/open62541/compare/v0.4.0...v0.5.0

### Added

- Allow reading node attributes with ``AsyncClient::read_attribute()` and `read_attributes()`.
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
