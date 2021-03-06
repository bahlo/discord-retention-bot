# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.2] - 2020-12-15
### Added
- Default retention with `*`
- Improve sleep interval to start when the last run started, not ended

## [1.0.1] - 2020-12-11
### Added
- `DELETE_PINNED={true,false}` functionality
- Automated publish to [crates.io](https://crates.io)

## [1.0.0] - 2020-12-08
### Added
- Clippy check job
- Security audit job (runs overnight)
- `failure` crate for simpler results and context
- `thiserror` crate for easier error creation
- Tests for configuration parsing
- Concurrent processing of each guild
### Changed
- Unified GitHub Actions workflow
- Messages are now deleted immediately instead of collecting first
- Print message when channel is not in configuration

## [0.1.0] - 2020-12-08
### Added
- Working prototype for one guild
- Basic GitHub actions setup

[Unreleased]: https://github.com/bahlo/discord-retention-bot/compare/v1.0.2...HEAD
[1.0.2]: https://github.com/bahlo/discord-retention-bot/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/bahlo/discord-retention-bot/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/bahlo/discord-retention-bot/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/olivierlacan/keep-a-changelog/releases/tag/v0.1.0
