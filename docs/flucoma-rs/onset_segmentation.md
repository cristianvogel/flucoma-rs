# OnsetSegmentation

Module path: `flucoma_rs::segmentation::OnsetSegmentation`

## Type

- `OnsetSegmentation`

## API

```rust
pub fn new(window_size: usize, fft_size: usize, filter_size: usize)
    -> Result<OnsetSegmentation, &'static str>;

pub fn process_frame(
    &mut self,
    input: &[f64],
    function: flucoma_rs::analyzation::OnsetFunction,
    filter_size: usize,
    threshold: f64,
    debounce: usize,
    frame_delta: usize,
) -> f64;

pub fn window_size(&self) -> usize;
pub fn fft_size(&self) -> usize;
```

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/onsetslice/+page.svx`.

- Onset slicing tracks spectral-change peaks over frames to mark likely event starts.
- Thresholding and debounce-like controls are central to avoiding false/redundant triggers.
- It is typically used for event-like segmentation where attacks are salient.
