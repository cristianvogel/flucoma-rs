# Adding New Algorithms

Each algorithm needs two files: a `cpp!` binding in `flucoma-sys/src/lib.rs` and a safe Rust wrapper in `src/<name>.rs`.

## Read the C++ header

Headers: `vendor/flucoma-core/include/flucoma/algorithms/public/`

Check three things:
- **Constructor** -- does it need an `Allocator&`? Any size parameters?
- **`init()`** -- optional, not all algorithms have one
- **`processFrame()`** -- does it need an `Allocator&`? Returns void or scalar?

## Add `flucoma-sys` bindings

Add the `#include` to the global `cpp! {{ }}` block, then add `create`, `destroy`, (optionally `init`), and `process_frame` functions. Use existing bindings as templates -- `Loudness` is the simplest, `Onset` covers the allocator + constructor-params case. No function comments are needed here.

### cpp! type mapping

| Rust | cpp! annotation | Notes |
|------|-----------------|-------|
| `FlucomaIndex` | `"ptrdiff_t"` | preferred alias for `isize`; `use flucoma_sys::FlucomaIndex` |
| `isize` | `"ptrdiff_t"` | raw form of the above — use `FlucomaIndex` instead |
| `f64` | `"double"` | |
| `bool` | `"bool"` | |
| `*const f64` | `"const double*"` | real input buffers |
| `*mut f64` | `"double*"` | real output buffers |
| `*const f64` (complex) | `"const double*"` | complex input: cast `*const Complex64` — interleaved `[re, im]` f64 pairs |
| `*mut f64` (complex) | `"double*"` | complex output: cast `*mut Complex64` — same layout |
| `*mut u8` | `"ClassName*"` | opaque handle |

### Allocator

If the C++ signature includes `Allocator& alloc`, pass `FluidDefaultAllocator()` inside the `cpp!` as argument.

## Create `src/<name>.rs`

Use `src/loudness.rs` (has `init()`) or `src/onset.rs` (constructor params, no `init()`) as a starting point. The pattern is always:

- `struct` holding `inner: *mut u8` + cached sizes
- `unsafe impl Send`
- `new()` -> `Result<Self, &'static str>`
- `process_frame()` with `assert!` on input lengths
- `Drop` calling `destroy`
- `#[cfg(test)] mod tests` with at least one silence/zero-input test
- Ensure the struct and all public functions are commented properly
- Add links to existing FluCoMa reference pages to structs (see STATUS.md for the links)
 
## Wire up

- Add `mod <name>;` in `src/lib.rs` and a `pub use` re-export in the appropriate module
- Mark done in `STATUS.md`
