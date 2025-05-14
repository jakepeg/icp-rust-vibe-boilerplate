# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 2025-05-14

### Added

- Add set_count update method to allow setting the counter to a specific value

### Changed

- Refactored project structure for scalability by modularizing the codebase:
  - Moved state management to `state.rs`
  - Created `handlers` module with submodules `counter.rs` and `greeting.rs`
  - Updated `lib.rs` to use the new modular structure

## [0.1.0] - 2025-04-24

### Added

- Basic canister structure with Rust
- Counter functionality with increment and get_count methods
- Greeting functionality
- PocketIC testing infrastructure
- Vitest test runner configuration
- GitHub CI workflow for automated end-to-end tests for all methods
- Project documentation
- Add custom instructions for github copilot
