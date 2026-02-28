# Low-level Rust Bindings for flucoma-core Audio Analysis

`flucoma-sys` provides low-level Rust bindings for the [flucoma-core](https://github.com/flucoma/flucoma-core) C++ audio analysis library.

See also [`flucoma-rs`](../README.md) which uses this crate to provide safe, high-level Rust wrappers.

Note: When building this crate locally, clone the repository with `git clone --recurse-submodules <url>`. The flucoma-core C++ source is included as a git submodule under `vendor/flucoma-core/`.

## Prerequisites

- Rust toolchain (stable)
- C++17 compatible compiler (MSVC, clang++, or g++)
- CMake (used to fetch and build Eigen, HISSTools, Spectra, foonathan/memory, and fmt)

Build note: `build.rs` sets `FETCHCONTENT_UPDATES_DISCONNECTED=ON` for CMake configure, which
prevents network update checks on already-fetched dependencies. A first build on a fresh machine
still requires network access (or pre-populated FetchContent source directories).

Offline CI/local note: you can force fully offline configure with
`FLUCOMA_FULLY_DISCONNECTED=1` and provide local dependency sources via:
`FLUCOMA_HISS_PATH`, `FLUCOMA_EIGEN_PATH`, `FLUCOMA_SPECTRA_PATH`,
`FLUCOMA_JSON_PATH`, `FLUCOMA_MEMORY_PATH`, and `FLUCOMA_FMT_PATH`.

## License

`flucoma-sys` is licensed under the BSD-3-Clause license, consistent with the upstream flucoma-core library.
