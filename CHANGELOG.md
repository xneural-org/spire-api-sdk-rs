# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-11-20

### Changed

- Updated the vendored [spire-api-sdk](https://github.com/spiffe/spire-api-sdk)
  protobuf definitions from v1.15.2 to v1.16.0.

### Added

- `spire::api::types::Entry::wit_svid_ttl`: TTL in seconds for WIT-SVIDs issued
  for an entry; the server default is used when unset (SPIRE API v1.16.0).
- `spire::api::types::EntryMask::wit_svid_ttl`: field mask for the above.

## [0.1.0] - 2026-07-24

### Added

- Initial library crate generating Rust bindings from the SPIRE API protobuf
  definitions (spire-api-sdk v1.15.2) with `tonic` and `prost`.
- `spire::api::types` module with the shared SPIRE API types.
- `spire::api::server::entry::v1` module with the server Entry API client and
  server.
- `spire::api::server::agent::v1` module with the server Agent API client and
  server.

[Unreleased]: https://github.com/xneural-org/spire-api-sdk-rs/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/xneural-org/spire-api-sdk-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/xneural-org/spire-api-sdk-rs/releases/tag/v0.1.0
