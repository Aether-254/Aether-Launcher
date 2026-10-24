# Changelog

All notable changes to Aether Launcher will be documented in this file.

The project follows [Semantic Versioning](https://semver.org/) once public releases begin.

## [Unreleased]

### Added

- Rust workspace with separated launcher domain crates
- Tauri 2 desktop shell
- React, TypeScript, and Fluent UI v9 interface foundation
- Frontend lint, tests, and production build
- Rust formatting, linting, and unit-test checks
- Issue Forms, Discussion Forms, pull request template, and CODEOWNERS
- Dependabot, CodeQL, dependency review, labeling, CI, and draft release workflows
- Sidebar navigation between home, instances, library, accounts, and settings
- Typed frontend IPC layer (`src/ipc`)
- Architecture overview in `docs/architecture.md`
- Appearance setting with dark, light, and follow-system colour schemes
- Version manifest parsing for libraries, arguments, asset index, client download and Java version, with rule evaluation

### Changed

- Home greeting follows the local time of day
- Document language set to `zh-CN`
- Download plans reject unsafe or duplicate destination paths
