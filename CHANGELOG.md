# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
