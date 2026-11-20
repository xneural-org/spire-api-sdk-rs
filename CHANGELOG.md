# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-07-24

### Added

- Initial library crate generating Rust bindings from the SPIRE API protobuf
  definitions (spire-api-sdk v1.15.2) with `tonic` and `prost`.
- `spire::api::types` module with the shared SPIRE API types.
- `spire::api::server::entry::v1` module with the server Entry API client and
  server.
- `spire::api::server::agent::v1` module with the server Agent API client and
  server.

[Unreleased]: https://github.com/xneural-org/spire-api-sdk-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/xneural-org/spire-api-sdk-rs/releases/tag/v0.1.0
