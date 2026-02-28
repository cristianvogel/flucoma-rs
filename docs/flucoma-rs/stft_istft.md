# Stft / Istft

Module path: `flucoma_rs::analyzation::{Stft, Istft, ComplexSpectrum, WindowType}`

## Types

- `WindowType`
- `ComplexSpectrum`
- `Stft`
- `Istft`

## Stft API

```rust
pub fn new(window_size: usize, fft_size: usize, hop_size: usize, window_type: WindowType)
    -> Result<Stft, &'static str>;

pub fn process_frame(&mut self, frame: &[f64]) -> ComplexSpectrum;
pub fn window_size(&self) -> usize;
pub fn fft_size(&self) -> usize;
pub fn hop_size(&self) -> usize;
pub fn num_bins(&self) -> usize;
```

## Istft API

```rust
pub fn new(window_size: usize, fft_size: usize, hop_size: usize, window_type: WindowType)
    -> Result<Istft, &'static str>;

pub fn process_frame(&mut self, spectrum: &ComplexSpectrum, output: &mut [f64]);
pub fn window_size(&self) -> usize;
pub fn fft_size(&self) -> usize;
pub fn hop_size(&self) -> usize;
pub fn num_bins(&self) -> usize;
```

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/bufstft/+page.svx`.

- STFT is presented as overlapping windowed FFT analysis over time.
- The reference stresses practical reconstruction constraints (window/hop relationships) for overlap-add resynthesis.
- It also notes the time-vs-frequency tradeoff and spectral smearing/pre-ring artifacts when modifying spectral data.
