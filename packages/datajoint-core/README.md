# DataJoint Core Library
The **DataJoint Core Library** is a low-level Rust crate for shared code across DataJoint client libraries written in high-level programming languages (such as Python and MATLAB). For example, any DataJoint framework will need to connect to a SQL database. Rather than writing connection code in the user-level programming language, this connection code can be written in the core library to be used by all user-level DataJoint frameworks.

High-level DataJoint frameworks currently interop using the [DataJoint Core C FFI](../datajoint-core-ffi-c), which exposes features from this Rust library through C headers.

## Related Docs
- [Setting Up DataJoint Core](../../doc/SETUP.md)
- [Design Documentation](../../doc/CORE.md)
