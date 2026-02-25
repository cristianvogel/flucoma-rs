# Adding a New Algorithms

This guide covers everything needed to wrap new flucoma-core algorithms. 

---

## Structure

```
flucoma-rs/
  src/
    lib.rs            <- pub mod + pub use exports
    <name>.rs         <- safe Rust wrapper (Step 2)
  flucoma-sys/
    src/
      lib.rs          <- cpp! macro bindings (Step 1)
```

`flucoma-sys` owns all C++ interaction. `flucoma-rs/src/<name>.rs` wraps the raw functions in a safe RAII struct.

---

## Read the C++ header

Headers live in `vendor/flucoma-core/include/flucoma/algorithms/public/`.

Look for three things:

### Constructor
```cpp
// No allocator -- most common
SpectralShape()

// Allocator required -- must pass Allocator alloc{} in cpp!
SpectralShape(Allocator& alloc)

// Parameters required -- capture as isize
OnsetDetectionFunctions(index maxSize, index maxFilterSize, Allocator& alloc)
```

### `init()` method (optional -- not all algorithms have one)
```cpp
void init(index size, double sampleRate);
```

### `processFrame()` signature
```cpp
// Real input -> real output (most common)
void processFrame(RealVectorView in, RealVectorView out, bool flag, Allocator& alloc)

// Returns a scalar
double processFrame(RealVectorView in, index func, index filter, index delta, Allocator& alloc)
```

Determine:
- Does the constructor need an `Allocator&`?
- Is there a separate `init()` call?
- Does `processFrame` need an `Allocator&`?
- What are the fixed-size outputs? (e.g. SpectralShape always outputs 7 values)

---

## Add bindings to `flucoma-sys/src/lib.rs`

### Add the `#include` to the global cpp! block at the top of the file

```rust
cpp! {{
    // ... existing includes ...
    #include <flucoma/algorithms/public/SpectralShape.hpp>
    using namespace fluid;
    using namespace fluid::algorithm;
}}
```

### Add one function per C++ operation

**create -- no allocator:**
```rust
pub fn spectralshape_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            return static_cast<void*>(new SpectralShape());
        })
    }
}
```

**create -- allocator required:**
```rust
pub fn spectralshape_create() -> *mut u8 {
    unsafe {
        cpp!([] -> *mut u8 as "void*" {
            Allocator alloc{};
            return static_cast<void*>(new SpectralShape(alloc));
        })
    }
}
```

**create -- constructor parameters + allocator:**
```rust
pub fn onset_create(max_size: isize, max_filter_size: isize) -> *mut u8 {
    unsafe {
        cpp!([max_size as "index", max_filter_size as "index"] -> *mut u8 as "void*" {
            Allocator alloc{};
            return static_cast<void*>(
                new OnsetDetectionFunctions(max_size, max_filter_size, alloc));
        })
    }
}
```

**destroy** (always the same shape):
```rust
pub fn spectralshape_destroy(ptr: *mut u8) {
    unsafe {
        cpp!([ptr as "SpectralShape*"] {
            delete ptr;
        })
    }
}
```

**init** (only if the algorithm has one):
```rust
pub fn spectralshape_init(ptr: *mut u8, size: isize, sample_rate: f64) {
    unsafe {
        cpp!([ptr as "SpectralShape*", size as "index", sample_rate as "double"] {
            ptr->init(size, sample_rate);
        })
    }
}
```

**processFrame -- real input -> real output, allocator required:**
```rust
pub fn spectralshape_process_frame(
    ptr: *mut u8,
    input: *const f64, input_len: isize,
    output: *mut f64,  output_len: isize,
    sample_rate: f64,
    min_freq: f64, max_freq: f64, rolloff_target: f64,
    log_freq: bool, use_power: bool,
) {
    unsafe {
        cpp!([
            ptr as "SpectralShape*",
            input as "const double*", input_len as "index",
            output as "double*", output_len as "index",
            sample_rate as "double",
            min_freq as "double", max_freq as "double", rolloff_target as "double",
            log_freq as "bool", use_power as "bool"
        ] {
            FluidTensorView<double, 1> in_v(const_cast<double*>(input), 0, input_len);
            FluidTensorView<double, 1> out_v(output, 0, output_len);
            Allocator alloc{};
            ptr->processFrame(in_v, out_v, sample_rate, min_freq, max_freq,
                              rolloff_target, log_freq, use_power, alloc);
        })
    }
}
```

**processFrame -- returns a scalar:**
```rust
pub fn onset_process_frame(ptr: *mut u8, ...) -> f64 {
    unsafe {
        cpp!([ptr as "OnsetDetectionFunctions*", ...] -> f64 as "double" {
            ...
            return ptr->processFrame(...);
        })
    }
}
```

### cpp! type annotation reference

| Rust type     | cpp! annotation     | Notes |
|---------------|---------------------|-------|
| `isize`       | `"index"`           | flucoma's `ptrdiff_t` typedef |
| `f64`         | `"double"`          |  |
| `bool`        | `"bool"`            |  |
| `*const f64`  | `"const double*"`   | read-only input buffer |
| `*mut f64`    | `"double*"`         | output buffer |
| `*mut u8`     | `"ClassName*"`      | opaque handle reinterpreted as the class |
| return `*mut u8` | `-> *mut u8 as "void*"` | new returns void* |
| return `f64`  | `-> f64 as "double"` |  |

---

## Step -- Create `src/<name>.rs`

Use `Loudness` as a template for algorithms with `init()`, or `OnsetDetectionFunctions` for algorithms where all parameters go to the constructor and/or processFrame.

```rust
use flucoma_sys::{
    spectralshape_create, spectralshape_destroy, spectralshape_process_frame,
};

pub struct SpectralShape {
    inner: *mut u8,
    n_bins: usize,
}

unsafe impl Send for SpectralShape {}

impl SpectralShape {
    pub fn new(n_bins: usize) -> Result<Self, &'static str> {
        if n_bins == 0 { return Err("n_bins must be > 0"); }
        let inner = spectralshape_create();
        if inner.is_null() { return Err("failed to create SpectralShape"); }
        Ok(Self { inner, n_bins })
    }

    /// Returns [centroid, spread, skewness, kurtosis, rolloff, flatness, crest].
    pub fn process_frame(
        &mut self,
        magnitudes: &[f64],
        sample_rate: f64,
        min_freq: f64, max_freq: f64, rolloff_target: f64,
        log_freq: bool, use_power: bool,
    ) -> [f64; 7] {
        assert_eq!(magnitudes.len(), self.n_bins);
        let mut out = [0.0f64; 7];
        spectralshape_process_frame(
            self.inner,
            magnitudes.as_ptr(), magnitudes.len() as isize,
            out.as_mut_ptr(), 7,
            sample_rate, min_freq, max_freq, rolloff_target,
            log_freq, use_power,
        );
        out
    }
}

impl Drop for SpectralShape {
    fn drop(&mut self) {
        spectralshape_destroy(self.inner);
    }
}
```

### Add tests

Add `#[cfg(test)] mod tests { ... }` at the bottom -- at minimum one test with silence/zero input to verify the algorithm constructs and runs without crashing.

---

## Wire up in `src/lib.rs`

```rust
pub mod spectral_shape;           // add module
pub use spectral_shape::SpectralShape;  // re-export the type
```

---

## Mark done in `STATUS.md`

Change the relevant `- [ ]` line to `- [x]` and add the `as \`flucoma_rs::...\`` path:

```
- [x] [`SpectralShape`](https://learn.flucoma.org/reference/spectralshape) as `flucoma_rs::spectral_shape` -- 7 shape descriptors: centroid, spread, skewness, kurtosis, rolloff, flatness, crest
```

---

## Allocator patterns -- quick reference

| Algorithm | Constructor allocator | processFrame allocator |
|-----------|-----------------------|------------------------|
| `Loudness` | no | no |
| `STFT` / `ISTFT` | no | no |
| `MelBands` | no | **yes** -- `Allocator alloc{}` |
| `OnsetDetectionFunctions` | **yes** | **yes** |
| `SpectralShape` | **yes** | **yes** |

When in doubt, check the C++ header -- if the parameter list includes `Allocator& alloc`, you must declare `Allocator alloc{};` inside the cpp! block and pass it.

---

## Common mistakes

- **Forgetting `const_cast`** -- `FluidTensorView<double,1>` takes a non-const pointer even for read-only views. Always `const_cast<double*>(input)`.
- **Wrong output size** -- pass the actual runtime length as `output_len`, not a compile-time constant, so the `FluidTensorView` matches what the algorithm expects.
- **Missing include** -- add the header to the global `cpp! {{ }}` block at the top of `flucoma-sys/src/lib.rs`; forgetting it gives an opaque "unknown type" C++ compile error.
- **`index` vs `isize`** -- always annotate integer parameters as `"index"` in cpp!, not `"int"` or `"size_t"`. flucoma-core uses `ptrdiff_t` internally.
