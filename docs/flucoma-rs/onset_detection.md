# OnsetDetectionFunctions

Module path: `flucoma_rs::analyzation::{OnsetDetectionFunctions, OnsetFunction}`

## Types

- `OnsetDetectionFunctions`
- `OnsetFunction` (10 variants)

## API

```rust
pub fn new(window_size: usize, fft_size: usize, filter_size: usize)
    -> Result<OnsetDetectionFunctions, &'static str>;

pub fn process_frame(
    &mut self,
    input: &[f64],
    function: OnsetFunction,
    filter_size: usize,
    frame_delta: usize,
) -> f64;

pub fn window_size(&self) -> usize;
pub fn fft_size(&self) -> usize;
```

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/onsetfeature/+page.svx`.

- OnsetFeature exposes the same core spectral-change measure used by OnsetSlice.
- It is useful both for understanding onset slicing internals and as a standalone descriptor.
- Configuration parameters conceptually mirror onset slicing analysis settings.
