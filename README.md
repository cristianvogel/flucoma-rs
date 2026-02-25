# Safe Rust Bindings for flucoma-core

`flucoma-rs` provides safe Rust bindings for the [flucoma-core](https://github.com/flucoma/flucoma-core) C++ audio analysis & segmentation library.

Note: When building this crate locally, clone the repository with `git clone --recurse-submodules <url>`. The flucoma-core C++ source is included as a git submodule under `vendor/flucoma-core/`.


## Status

This is a work in progress. See [STATUS.md](./STATUS.md) which functions are wrapped, and which not. 

See [EXTEND.md](./EXTEND.md) on how to create new wrappers. Pull requests are welcome!


## Prerequisites

- Rust toolchain (stable)
- C++17 compatible compiler (MSVC, clang++, or g++)
- CMake (used to fetch and build Eigen, HISSTools, Spectra, and foonathan/memory)


## Examples

All algorithms follow the same two-phase pattern:
1. **Construct** -- allocates internal buffers with `new(...)`.
2. **Process** -- call `process_frame(...)` once per audio frame.

### Loudness

Measures EBU R128-style integrated loudness and peak level per frame.

```rust,no_run
use flucoma_rs::features::Loudness;

let mut analyzer = Loudness::new(1024, 44100.0).unwrap();

let frame = vec![0.0f64; 1024]; // fill with audio samples
let result = analyzer.process_frame(&frame, /*k_weighting=*/true, /*true_peak=*/true);

println!("Loudness: {:.1} dBFS", result.loudness_db);
println!("Peak:     {:.1} dBFS", result.peak_db);
```

### Stft & MelBands

Converts a magnitude spectrum into mel-scaled band energies.

```rust,no_run
use flucoma_rs::features::{Stft, MelBands, WindowType};

let fft_size  = 1024usize;
let n_bins    = fft_size / 2 + 1;
let n_bands   = 40usize;

let mut stft = Stft::new(fft_size, fft_size, fft_size / 2, WindowType::Hann).unwrap();
let mut mel  = MelBands::new(n_bands, n_bins, 80.0, 8000.0, 44100.0, fft_size).unwrap();

let frame = vec![0.0f64; fft_size]; // fill with audio samples
let spectrum   = stft.process_frame(&frame);
let magnitudes = spectrum.magnitudes();
let bands      = mel.process_frame(&magnitudes, /*mag_norm=*/false, /*use_power=*/false, /*log_output=*/false);

println!("Mel bands: {:?}", &bands[..4]);
```

### OnsetDetectionFunctions

Computes a scalar onset detection value per frame using one of ten spectral
difference functions.

```rust,no_run
use flucoma_rs::features::{OnsetDetectionFunctions, OnsetFunction};

let window = 1024usize;
let mut odf = OnsetDetectionFunctions::new(window, window, /*filter_size=*/5).unwrap();

// Feed silent frame to seed history
let silence = vec![0.0f64; window];
let _ = odf.process_frame(&silence, OnsetFunction::PowerSpectrum, 5, 0);

// Then feed a frame with audio -- larger return value = more likely onset
let mut audio_frame = vec![0.0f64; window];
audio_frame[512] = 1.0;
let value = odf.process_frame(&audio_frame, OnsetFunction::PowerSpectrum, 5, 0);
println!("Onset value: {:.4}", value);
```

## License

`flucoma-rs` is licensed under the BSD-3-Clause license, consistent with the upstream flucoma-core library.
