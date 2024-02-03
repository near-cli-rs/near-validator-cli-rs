# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.1...v0.1.2) - 2024-02-03

### Other
- Updated binary releases pipeline to use cargo-dist v0.9.0 (previously v0.7.2) ([#8](https://github.com/near-cli-rs/near-validator-cli-rs/pull/8))

## [0.1.1](https://github.com/near-cli-rs/near-validator-cli-rs/compare/v0.1.0...v0.1.1) - 2024-01-27

### Other
- removed explicit rust-toolchain, so let's use the latest stable by default
- Upgraded NEAR crates to 0.20.0 release ([#5](https://github.com/near-cli-rs/near-validator-cli-rs/pull/5))
- fixed code_style.yml to use ubuntu-latest instead of ubuntu-20.04
- removed old CI integrations that are now replaced by cargo-dist (release.yml workflow)
