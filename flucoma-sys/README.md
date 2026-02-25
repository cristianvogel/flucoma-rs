# Low-level Rust Bindings for flucoma-core Audio Analysis

`flucoma-sys` provides low-level Rust bindings for the [flucoma-core](https://github.com/flucoma/flucoma-core) C++ audio analysis library.

See also [`flucoma-rs`](../README.md) which uses this crate to provide safe, high-level Rust wrappers.

Note: When building this crate locally, clone the repository with `git clone --recurse-submodules <url>`. The flucoma-core C++ source is included as a git submodule under `vendor/flucoma-core/`.

## Prerequisites

- Rust toolchain (stable)
- C++17 compatible compiler (MSVC, clang++, or g++)
- CMake (used to fetch and build Eigen, HISSTools, Spectra, and foonathan/memory)

## License

`flucoma-sys` is licensed under the BSD-3-Clause license, consistent with the upstream flucoma-core library.
