# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2025-05-30

### Added

- Add `MonitoredItemBuilder::attribute()` to select monitored attribute at compile time, yielding
  values of the expected `DataValue` subtype.
- Add `MonitoredItemValue` enum wrapping both data change and event monitored item notifications.

### Changed

- Breaking: When using `MonitoredItemBuilder::attribute_id()` for attributes not known at compile
  time, monitored items yield values of `MonitoredItemValue`.
- Breaking: Replace `time::OffsetDateTime` with `time::UtcDateTime` in conversions from and/or to
  `ua::DateTime`.
- Breaking: Remove methods `ua::DataValue::status_code()` and `ua::DataValue::with_status_code()`
  deprecated in 0.7.1. Use `status()` and/or `with_status()` instead.
- Breaking: Remove method `ua::String::as_slice()` deprecated in 0.5.0. Use `as_bytes()` instead.
- Breaking: Bump Minimum Supported Rust Version (MSRV) to 1.83.
- Mark `AsyncMonitoredItem::into_stream()` as deprecated. Use the `Stream` implementation of this
  type directly instead.
- Remove `unsafe` qualifier from `Server::statistics()`, it may now be be called concurrently.
- Upgrade to open62541 version
  [1.4.12](https://github.com/open62541/open62541/releases/tag/v1.4.12).

### Fixed

- Fix runtime crash for monitored items with attribute ID `ua::AttributeId::EVENTNOTIFIER`. Using
  `MonitoredItemBuilder::attribute()`, yielded value type is `ua::Array<ua::Variant>` (instead of
  `ua::DataValue`, e.g. for `ua::AttributeId::VALUE`).

## [0.8.5] - 2025-05-14

### Added

- Add `ua::ServerStatusDataType` with `ua::ServerState` and `ua::BuildInfo`.
- Add methods `ua::DateTime::try_from_unix_timestamp_nanos()` and `as_unix_timestamp_nanos()` that
  work independently of the `time` feature.

## [0.8.4] - 2025-04-30

### Added

- Add `ua::Guid`.
- Add `ua::EnumDefinition` and `ua::EnumField`.

## [0.8.3] - 2025-03-27

### Added

- Add method `Server::statistics()` to get server statistics.
- Add `ua::ServerStatistics`, `ua::SecureChannelStatistics`, and `ua::SessionStatistics`.
- Add `ua::EUInformation` and accompanying _newtype_ `ua::UnitId`.
- Add `ua::Range`.

## [0.8.2] - 2025-03-19

### Added

- Add `ua::Enumeration`.
- Add `ValueType` variants `Structure` (inner type `ua::ExtensionObject`) and `Enumeration`.

### Changed

- Upgrade to open62541 version
  [1.4.11](https://github.com/open62541/open62541/releases/tag/v1.4.11.1).

## [0.8.1] - 2025-03-04

### Added

- Add method `ClientBuilder::private_key_password_callback()` to handle encrypted SSL private keys.

### Fixed

- Avoid default behaviour of blocking and asking for password on standard input when password for
  encrypted SSL private keys is not available.

## [0.8.0] - 2025-02-12

### Added

- Add method `with_policy_id()` to `ua::AnonymousIdentityToken` and `ua::UserNameIdentityToken`.
- Add method `ua::UserNameIdentityToken::with_encryption_algorithm()`.
- Add `ua::X509IdentityToken` and `ua::IssuedIdentityToken`.
- Breaking: Add enum variants `X509` and `Issued` to `UserIdentityToken`.

### Changed

- Breaking: Expect `ua::String`, `ua::ByteString` in `ua::UserNameIdentityToken::with_user_name()`,
  `ua::UserNameIdentityToken::with_password()`.

## [0.7.3] - 2025-02-06

### Added

- Add method `MonitoredItemBuilder::attribute_id()` to set a different attribute ID to monitor.
- Add method `MonitoredItemBuilder::filter()` and related data types `ua::DataChangeFilter`,
  `ua::EventFilter`, `ua::AggregateFilter` along with associated data types.
- Add methods `ClientBuilder::security_mode()` and `ClientBuilder::security_policy_uri()` to select
  security mode and security policy URI when filtering remote endpoints.

### Changed

- Upgrade to open62541 version
  [1.4.10](https://github.com/open62541/open62541/releases/tag/v1.4.10).

## [0.7.2] - 2025-01-13

### Added

- Add method `Server::write_data_value()` to allow setting variable value with timestamps (#190).
- Add `SubscriptionBuilder` and `MonitoredItemBuilder` to create subscription and monitored items
  with additional control over request options.

### Changed

- Return source and server timestamps when reading attributes through `Server::read_attribute()`.
- Return source and server timestamps when reading attributes with `AsyncClient::read_value()`,
  `AsyncClient::read_attribute()` and related methods that return `DataValue`.

## [0.7.1] - 2024-12-19

### Added

- Zeroize memory held by `PrivateKey` when dropped.
- Add methods `ua::DataValue::with_source_timestamp()`, `ua::DataValue::with_server_timestamp()`,
  `ua::DataValue::with_source_picoseconds()`, `ua::DataValue::with_server_picoseconds()`.

## Changed

- Rename methods `ua::DataValue::status_code()` to `status()`, `ua::DataValue::with_status_code()`
  to `with_status()`. Deprecate the former methods.

## [0.7.0] - 2024-12-13

### Added

- Add type `PrivateKey` to wrap private keys when using Mbed TLS.
- Add types `ua::SecurityLevel`, `ua::EndpointDescription`, `ua::MessageSecurityMode`.
- Add method `ClientBuilder::get_endpoints()` to get remote server endpoints.
- Add method `ClientBuilder::certificate_verification()` and type `ua::CertificateVerification`.
- Add method `ua::CertificateVerification::custom()` and trait `CustomCertificateVerification` to
  allow custom certificate verification.

### Changed

- Breaking: Bump Minimum Supported Rust Version (MSRV) to 1.80.
- Breaking: Change type `Certificate` to hold certificate without private key.
- Breaking: Use new types `Certificate` and `PrivateKey` instead of raw `&[u8]` in
  `ClientBuilder::default_encryption()`, `ServerBuilder::default_with_security_policies()`, and
  `ServerBuilder::default_with_secure_security_policies()`.

## [0.6.6] - 2024-12-04

### Added

- Implement `Send` and `Sync` for `ua::Array` and `ua::KeyValueMap`.

## [0.6.5] - 2024-12-04

### Added

- Add ability to create SSL certificates using Mbed TLS with `create_certificate()`.
- Allow converting `ua::String` into `ua::ByteString` with method `ua::String::into_byte_string()`.
- Add type `ua::KeyValueMap`.
- Add method `ua::QualifiedName::ns0()` for creating qualified names in namespace 0.

### Changed

- Upgrade to open62541 version [1.4.8](https://github.com/open62541/open62541/releases/tag/v1.4.8).

## [0.6.4] - 2024-11-23

### Added

- Add feature flag `mbedtls` to build with encryption support by using bundled Mbed TLS version
  [3.6.2](https://github.com/Mbed-TLS/mbedtls/releases/tag/mbedtls-3.6.2).
- Add method for client encryption by using `ClientBuilder::default_encryption()`.
- Add methods for server encryption by using `ServerBuilder::default_with_security_policies()`,
  `ServerBuilder::default_with_secure_security_policies()`.
- Add `ClientBuilder::accept_all()` and `ServerBuilder::accept_all()` to skip certificate checks.
- Add method `ServerRunner::run_until_cancelled()` to allow gracefully shutting down server (#169).
- Add method `ServerRunner::access_control()` and related trait `AccessControl` with implementation
  `DefaultAccessControl` and `DefaultAccessControlWithLoginCallback` to allow custom authentication
  and authorization of clients in the server.
- Add method `ua::ByteString::new()` to create new OPC UA byte strings.

### Changed

- Upgrade to open62541 version [1.4.7](https://github.com/open62541/open62541/releases/tag/v1.4.7).
- Respect fill/alignment formatting parameters when printing `ua::String` (#166), as well as types
  `ua::NodeId`, `ua::QualifiedName`, `ua::StatusCode` (#167).

## [0.6.3] - 2024-10-14

### Changed

- Upgrade to open62541 version [1.4.6](https://github.com/open62541/open62541/releases/tag/v1.4.6).

## [0.6.2] - 2024-09-27

### Added

- Implement relations `PartialEq`, `Eq`, `PartialOrd`, `Ord` for `ua::Array` type.
- Add `ua::NodeId::namespace_index()` to get node ID's namespace index.

## [0.6.1] - 2024-09-04

### Added

- Add method `ClientBuilder::user_identity_token()` and associated types to set user identity token.

## [0.6.0] - 2024-08-20

### Added

- Add support for creating OPC UA server (#89), including callback-driven variable nodes, adding and
  removing nodes from the server namespace, querying the server namespace, adding and removing
  references, reading and writing object properties, creating and triggering events, browsing the
  server namespace, reading attributes, implementing callback-driven method nodes.
- Add `ua::StatusCode::is_uncertain()`, `is_bad()` for checking status code severity (#63).
- Add `ua::StatusCode::name()` to get human-readable representation of status code (#93).
- Add support for `ua::Argument` data type and basic support for `ua::ExtensionObject` (#71).
- Add `Debug` implementation for `ua::Array<T>` data types.
- Add `ValueType` enum to check `ua::Variant` without unwrapping (also `ua::Argument`).
- Add tracing log messages when processing service requests and responses (#80).
- Add methods to `ClientBuilder` to set response timeout, client description, and connectivity check
  interval (#81).
- Add logical OR combinator for `ua::BrowseResultMask` and `ua::NodeClassMask`.
- Add const `ua::NodeClassMask` variants to initialize masks.
- Add `serde` serialization for `ua::Array` and `ua::Variant` with array value.
- Add constructors `ua::Variant::scalar()` and `ua::Variant::array()`.
- Add constructor `ua::DataValue::new()`.
- Add helper method `ua::Array::into_array()` for conversion into native Rust array.
- Add known enum variants to `ua::StatusCode`.
- Add method `ua::ExpandedNodeId::numeric()` to create numeric expanded node IDs.
- Add types `ua::RelativePath`, `ua::RelativePathElement`, `ua::BrowsePath`, `ua::BrowsePathResult`,
  `ua::BrowsePathTarget`.
- Add `ua::NodeAttributes` and subtypes `ua::ObjectAttributes`, `ua::VariableAttributes`,
  `ua::MethodAttributes`, `ua::ObjectTypeAttributes`, `ua::VariableTypeAttributes`,
  `ua::ReferenceTypeAttributes`, `ua::DataTypeAttributes`, `ua::ViewAttributes`.
- Add method `Error::status_code()` to get original OPC UA status code that caused the error.
- Add method `ua::NodeId::into_expanded_node_id()`.
- Implement `Index` and `IndexMut` for `ua::Array` to allow direct element access.
- Add methods to `ua::DataValue` to get source/server timestamps.

### Changed

- Breaking: Return `Result` instead of `Option` for references in `AsyncClient::browse_many()` and
  `browse_next()` (#59, #60).
- Breaking: Return `Result` wrapping `ua::DataValue` from `AsyncClient::read_attributes()` (#61).
- Breaking: Move `ua::VariantValue` and `ua::ScalarValue` to top-level export outside `ua`.
- Breaking: Remove `ua::ArrayValue` for now (until we have a better interface).
- Breaking: Return output arguments without `Option` from `AsyncClient::call_method()` (#79).
- Breaking: Remove misleading `FromStr` trait implementation and offer `ua::String::new()` instead.
- Breaking: Upgrade to open62541 version 1.4.4 (#82). This affects the API of this crate as follows:
  - Automatically unwrap `ua::ExtensionObject` arrays inside `ua::Variant`.
- Breaking: Remove `cycle_time` parameter from `AsyncClient`'s interface (#91). The relevance of
  this parameter has been reduced by the upgrade to open62541 version 1.4.
- Breaking: Remove associated functions for enum data types deprecated in 0.5.0, e.g.
  `ua::AttributedId::value()`. Use uppercase constants `ua::AttributedId::VALUE` instead.
- Breaking: Split `Server::new()` and `ServerBuilder::build()` result type into `Server` and
  `ServerRunner` to allow interacting with server's data tree while server is running.
- Coerce _empty_ arrays of `ua::ExtensionObject` into the requested data type. This mirrors the
  auto-unwrapping behavior of open62541 version 1.4.
- Include appropriate trait bounds in return type of `AsyncMonitoredItem::into_stream()`.
- Breaking: Add prefix `with_` in `ua::BrowseNextRequest::with_release_continuation_points()`.
- Breaking: Change signatures of `AsyncClient::browse()` and `AsyncClient::browse_many()` to accept
  `ua::BrowseDescription` instead of `ua::NodeId` for better control over the resulting references.
- Breaking: Return typed variant `DataValue` instead of `ua::DataValue` from `AsyncClient` read
  operations.

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
- Avoid memory leak when calling `ua::Variant::with_scalar()` multiple times on the same value.

## [0.6.0-pre.6] - 2024-08-20

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
- Add generic way of adding nodes with `Server::add_node()` and associated `Node` type.
- Add methods `Server::add_namespace()`, `Server::get_namespace_by_name()`, and
  `Server::get_namespace_by_index()`.
- Add methods `Server::add_reference()` and `Server::delete_reference()`.
- Add method `Error::status_code()` to get original OPC UA status code that caused the error.
- Add method `ua::NodeId::into_expanded_node_id()`.
- Add method `Server::translate_browse_path_to_node_ids()`.
- Implement `Index` and `IndexMut` for `ua::Array` to allow direct element access.
- Add methods `Server::write_object_property()` and `Server::read_object_property()`.
- Add methods `Server::create_event()` and `Server::trigger_event()`.
- Add methods `Server::browse()`, `Server::browse_next()`, `Server::browse_recursive()`, and
  `Server::browse_simplified_browse_path()`.
- Add method `Server::read_attribute()` to read node attributes in a type-safe way.
- Add methods to `ua::DataValue` to get source/server timestamps.
- Add method `Server::add_method_node()`, and accompanying `MethodCallback` trait, to implement
  method nodes.

### Changed

- Include appropriate trait bounds in return type of `AsyncMonitoredItem::into_stream()`.
- Breaking: Add prefix `with_` in `ua::BrowseNextRequest::with_release_continuation_points()`.
- Upgrade to open62541 version 1.4.4.
- Breaking: Change signatures of `AsyncClient::browse()` and `AsyncClient::browse_many()` to accept
  `ua::BrowseDescription` instead of `ua::NodeId` for better control over the resulting references.
- Breaking: Return typed variant `DataValue` instead of `ua::DataValue` from `AsyncClient` read
  operations.
- Breaking: Adjust signatures of `Server::add_object_node()` and `Server::add_variable_node()` to
  match the new methods, returning the inserted node IDs.
- Breaking: Rename `Server::write_variable()` to `Server::write_value()` to better match client
  interface.
- Breaking: Remove special-cased helper method `Server::write_variable_string()`.

## [0.6.0-pre.5] - 2024-05-31

### Changed

- Upgrade to open62541 version 1.4.1. This removes the workaround introduced in 0.6.0-pre.3, it is
  no longer necessary.

## [0.6.0-pre.4] - 2024-05-22

### Changed

- Coerce _empty_ arrays of `ua::ExtensionObject` into the requested data type. This mirrors the
  auto-unwrapping behavior of open62541 version 1.4.

## [0.6.0-pre.3] - 2024-05-18

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
- Upgrade to open62541 version 1.4.0.
- Reintroduce internal mutex in `AsyncClient` to work around issue in open62541 version 1.4.

### Fixed

- Avoid memory leak when calling `ua::Variant::with_scalar()` multiple times on the same value.

## [0.6.0-pre.2] - 2024-04-12

### Added

- Add basic support for creating OPC UA server with static nodes (#89).

## [0.6.0-pre.1] - 2024-04-05

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

### Added

- Fallible conversion from `time::OffsetDateTime` to `ua::DateTime`.

### Changed

- Breaking: Rename `ua::DateTime::as_datetime()` to `ua::DateTime::to_utc()`.
- Use RFC 3339 variant of ISO 8601 for `ua::DateTime` serialization.

## [0.3.0] - 2024-01-23

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

### Changed

- Fix typo in README file and changelog.

## [0.2.1] - 2024-01-12

### Added

- First public release.

[Unreleased]: https://github.com/HMIProject/open62541/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/HMIProject/open62541/compare/v0.8.5...v0.9.0
[0.8.5]: https://github.com/HMIProject/open62541/compare/v0.8.4...v0.8.5
[0.8.4]: https://github.com/HMIProject/open62541/compare/v0.8.3...v0.8.4
[0.8.3]: https://github.com/HMIProject/open62541/compare/v0.8.2...v0.8.3
[0.8.2]: https://github.com/HMIProject/open62541/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/HMIProject/open62541/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/HMIProject/open62541/compare/v0.7.3...v0.8.0
[0.7.3]: https://github.com/HMIProject/open62541/compare/v0.7.2...v0.7.3
[0.7.2]: https://github.com/HMIProject/open62541/compare/v0.7.1...v0.7.2
[0.7.1]: https://github.com/HMIProject/open62541/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/HMIProject/open62541/compare/v0.6.6...v0.7.0
[0.6.6]: https://github.com/HMIProject/open62541/compare/v0.6.5...v0.6.6
[0.6.5]: https://github.com/HMIProject/open62541/compare/v0.6.4...v0.6.5
[0.6.4]: https://github.com/HMIProject/open62541/compare/v0.6.3...v0.6.4
[0.6.3]: https://github.com/HMIProject/open62541/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/HMIProject/open62541/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/HMIProject/open62541/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/HMIProject/open62541/compare/v0.5.0...v0.6.0
[0.6.0-pre.6]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.5...v0.6.0-pre.6
[0.6.0-pre.5]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.4...v0.6.0-pre.5
[0.6.0-pre.4]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.3...v0.6.0-pre.4
[0.6.0-pre.3]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.2...v0.6.0-pre.3
[0.6.0-pre.2]: https://github.com/HMIProject/open62541/compare/v0.6.0-pre.1...v0.6.0-pre.2
[0.6.0-pre.1]: https://github.com/HMIProject/open62541/compare/v0.5.0...v0.6.0-pre.1
[0.5.0]: https://github.com/HMIProject/open62541/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/HMIProject/open62541/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/HMIProject/open62541/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/HMIProject/open62541/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/HMIProject/open62541/releases/tag/v0.2.1
