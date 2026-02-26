# Adding New Algorithms

Each algorithm needs two files: a `cpp!` binding in `flucoma-sys/src/lib.rs` and a safe Rust wrapper in `src/<name>.rs`.

## Read the C++ header

Headers: `vendor/flucoma-core/include/flucoma/algorithms/public/`

Check three things:
- **Constructor** -- does it need an `Allocator&`? Any size parameters?
- **`init()`** -- optional, not all algorithms have one
- **`processFrame()`** -- does it need an `Allocator&`? Returns void or scalar?

## Add `flucoma-sys` bindings

Add the `#include` to the global `cpp! {{ }}` block, then add `create`, `destroy`, (optionally `init`), and `process_frame` functions. Use existing bindings as templates -- `Loudness` is the simplest, `OnsetDetectionFunctions` covers the allocator + constructor-params case.

### cpp! type mapping

| Rust | cpp! annotation | Notes |
|------|-----------------|-------|
| `isize` | `"ptrdiff_t"` | flucoma's `index` type |
| `f64` | `"double"` | |
| `bool` | `"bool"` | |
| `*const f64` | `"const double*"` | input buffers |
| `*mut f64` | `"double*"` | output buffers |
| `*mut u8` | `"ClassName*"` | opaque handle |

### Allocator rule

If the C++ signature includes `Allocator& alloc`, add `Allocator alloc{};` inside the `cpp!` block and pass it.

## Create `src/<name>.rs`

Use `src/loudness.rs` (has `init()`) or `src/onset.rs` (constructor params, no `init()`) as a starting point. The pattern is always:

- `struct` holding `inner: *mut u8` + cached sizes
- `unsafe impl Send`
- `new()` -> `Result<Self, &'static str>`
- `process_frame()` with `assert!` on input lengths
- `Drop` calling `destroy`
- `#[cfg(test)] mod tests` with at least one silence/zero-input test

## Wire up

- Add `mod <name>;` and a `pub use` re-export in `src/lib.rs`
- Mark done in `STATUS.md`
