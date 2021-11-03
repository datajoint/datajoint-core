# DataJoint Core Library
The **DataJoint Core Library** is a low-level software library for all shared code across the user-level DataJoint frameworks. Rather than using their own code, DataJoint frameworks, such as those written in Python and MATLAB, can use the library for connecting to Structured Query Language (SQL) databases, executing queries against a connection, and reading query results. The core library aims to remove the burden of writing and maintaining duplicate code across language-specific DataJoint frameworks, enhancing developer productivity, ecosystem maintainability, and framework extensibility. The DataJoint Core Library can be further enhanced by future work to house more code that currently exists at the user level, such as building generic SQL queries, building schemas, and much more.

This project started as a **UTDesign Capstone** project with senior students in Computer Science at The University of Texas at Dallas.

## Repository Layout
This repository primarily contains two Rust packages (aka, crates) that make up the core library.

- [datajoint-core](packages/datajoint-core) - A Rust library that provides common DataJoint features.
- [datajoint-core-ffi-c](packages/datajoint-core-ffi-c) - A C FFI library for calling the core library from user-level languages.

At the moment, [datajoint-python](datajoint-python) is a temporary home for integrating the core library into the Python library.