# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Breaking: Remove default `Display` implementation for most `ua` wrapper types (using the `Debug`
  implementation is more appropriate in these cases).

### Fixed

- Fix handling of empty and invalid strings.
- Include values in log messages.

## [0.2.2] - 2024-01-12

[0.2.2]: https://github.com/HMIProject/open62541/compare/v0.2.1...v0.2.2

### Changed

- Fix typo in README file and changelog.

## [0.2.1] - 2024-01-12

[0.2.1]: https://github.com/HMIProject/open62541/releases/tag/v0.2.1

### Added

- First public release.