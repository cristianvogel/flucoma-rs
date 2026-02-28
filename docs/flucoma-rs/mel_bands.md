# MelBands

Module path: `flucoma_rs::analyzation::MelBands`

## Type

- `MelBands`

## API

```rust
pub fn new(
    n_bands: usize,
    n_bins: usize,
    lo_hz: f64,
    hi_hz: f64,
    sample_rate: f64,
    window_size: usize,
) -> Result<MelBands, &'static str>;

pub fn process_frame(
    &mut self,
    input: &[f64],
    mag_norm: bool,
    use_power: bool,
    log_output: bool,
) -> Vec<f64>;

pub fn n_bands(&self) -> usize;
pub fn n_bins(&self) -> usize;
```

## Notes

- `input` is expected to be magnitude-spectrum bins.

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/melbands/+page.svx`.

- Mel bands are produced by weighting FFT magnitudes with overlapping triangular filters on the Mel scale.
- The Mel scale is linear at low frequencies and logarithmic at higher frequencies, approximating pitch perception.
- The reference highlights the importance of filter normalization and FFT size selection for stable outputs.
