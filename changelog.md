# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2021-11-15
### Added
- Rust-based libary that has a C FFI. This includes connections, cursors, and generic value decoding.
- Utility in the lib to receive a Generic SQL query and execute against a relational database server.
- Support for MySQL and Postgres database servers.
- Support for placeholder arguments.
- Ability to dynamically configure database without the need to recompoile. i.e. ability to issue query to MySQL or Postgres.
- Simple Python client to demonstrate how it can call datajoint-core.
- Documentation for the project.
- Initial unit tests and integration tests.
- Github actions.


[0.1.0]: https://github.com/datajoint/datajoint-core/milestone/3
