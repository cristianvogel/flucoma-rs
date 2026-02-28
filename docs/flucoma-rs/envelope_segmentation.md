# EnvelopeSegmentation

Module path: `flucoma_rs::segmentation::EnvelopeSegmentation`

## Type

- `EnvelopeSegmentation`

## API

```rust
pub fn new(floor: f64, hi_pass_freq: f64) -> Result<EnvelopeSegmentation, &'static str>;

pub fn process_sample(
    &mut self,
    sample: f64,
    on_threshold: f64,
    off_threshold: f64,
    floor: f64,
    fast_ramp_up: usize,
    slow_ramp_up: usize,
    fast_ramp_down: usize,
    slow_ramp_down: usize,
    hi_pass_freq: f64,
    debounce: usize,
) -> f64;
```

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/ampslice/+page.svx`.

- AmpSlice compares fast and slow envelope followers and triggers on relative rises.
- `onThreshold` and `offThreshold` form a Schmitt-trigger style onset/offset cycle.
- Ramp parameters shape responsiveness; `floor` adds an absolute gate against very low-level content.
