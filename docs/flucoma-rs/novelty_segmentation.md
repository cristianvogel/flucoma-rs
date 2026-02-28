# NoveltySegmentation

Module path: `flucoma_rs::segmentation::NoveltySegmentation`

## Type

- `NoveltySegmentation`

## API

```rust
pub fn new(kernel_size: usize, n_dims: usize, filter_size: usize)
    -> Result<NoveltySegmentation, &'static str>;

pub fn process_frame(&mut self, input: &[f64], threshold: f64, min_slice_length: usize) -> f64;
pub fn n_dims(&self) -> usize;
```

## FluCoMa Reference Notes (Archival)

Source: `learn-website/src/routes/(content)/reference/noveltyslice/+page.svx`.

- Novelty slicing is based on self-similarity over time rather than only transient/onset cues.
- A novelty curve is derived from local contrast in a self-similarity matrix.
- Kernel size and smoothing strongly influence slice granularity (longer-form vs finer segmentation).
